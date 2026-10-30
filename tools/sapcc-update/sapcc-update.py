#!/usr/bin/python3.11
"""sapcc-update - fetch the newest SAP Cloud Connector if a newer one exists.

Looks up the newest ``sapcc-<version>-linux-x64.zip`` on
tools.hana.ondemand.com, compares it with the last version this tool
processed, and only then downloads, verifies and extracts the archive into
``<base-dir>/releases/<version>``. ``<base-dir>/current`` points at the newest
release. The archive contains the installer RPM; nothing is installed.
"""

# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import sys
from typing import Final

MIN_PYTHON: Final = (3, 11)

# Kept ahead of all other code, and the file avoids newer-only syntax, so a
# wrong interpreter fails with a sentence instead of a SyntaxError.
if sys.version_info < MIN_PYTHON:
    sys.exit(
        "error: requires Python {}.{} or newer; running {} at {}".format(
            *MIN_PYTHON, sys.version.split()[0], sys.executable
        )
    )

import argparse
import dataclasses
import errno
import fcntl
import http.client
import json
import logging
import os
import re
import shutil
import signal
import ssl
import stat
import tempfile
import time
import tomllib
import urllib.error
import urllib.request
import zipfile
from collections.abc import Mapping, Sequence
from dataclasses import dataclass, field
from enum import IntEnum
from pathlib import Path
from typing import Self, TextIO
from urllib.parse import urlsplit

# Deferred on 3.15+ (PEP 810): only needed when an update is actually done,
# a config file exists, or --json is given.
__lazy_modules__ = ["json", "shutil", "tomllib", "zipfile"]

__version__: Final = "1.0.0"
PROG: Final = "sapcc-update"

DEFAULT_CONFIG: Final = Path("/etc/sapcc-update/sapcc-update.toml")
DEFAULTS: Final[Mapping[str, str]] = {
    "base_dir": "/tmp/sapcc-update",  # noqa: S108 - guarded by ensure_base_dir()
    "keep": "3",
    "base_url": "https://tools.hana.ondemand.com",
    "index_url": "https://tools.hana.ondemand.com/",
    # "auto": derive the licence cookie from the download page.
    "eula_cookie": "auto",
    "start_version": "2.19.2",
    "timeout": "60",
}
# Config-file key -> environment variable.
ENV_VARS: Final[Mapping[str, str]] = {key: f"SAPCC_{key.upper()}" for key in DEFAULTS}

ARCHIVE_RE: Final = re.compile(r"sapcc-(\d+(?:\.\d+)+)-linux-x64\.zip")
# Download links only work with a cookie confirming the SAP Developer License
# that the page currently links to, e.g. developer-license-3_2.txt.
LICENSE_RE: Final = re.compile(r"developer-license-(\d+)_(\d+)\.txt")
FALLBACK_LICENSE: Final = (3, 2)
LAST_RESPONSE: Final = "last-failed-response.html"
MAX_INDEX_BYTES: Final = 16 * 1024 * 1024
MAX_UNPACKED_BYTES: Final = 4 * 1024**3
DOWNLOAD_ATTEMPTS: Final = 3
RETRY_DELAY_S: Final = 10.0
CHUNK: Final = 1024 * 1024
HTTP_SERVER_ERROR: Final = 500

log = logging.getLogger(PROG)


class Exit(IntEnum):
    """Documented exit codes. Keep in sync with the epilog."""

    OK = 0
    FAILURE = 1
    USAGE = 2
    INTERRUPTED = 128 + signal.SIGINT
    BROKEN_PIPE = 128 + signal.SIGPIPE


class Error(Exception):
    """Expected, reportable failure. Never reaches the user as a traceback."""


class UsageError(Error):
    """Invalid configuration or command line. Maps to exit code 2."""


@dataclass(frozen=True, slots=True, order=True)
class Version:
    """Numeric dotted version, ordered component-wise (2.9.0 < 2.10.0)."""

    parts: tuple[int, ...]

    @classmethod
    def parse(cls, raw: str) -> Self:
        """Return the Version for *raw*; raise ValueError if malformed."""
        if not re.fullmatch(r"\d+(?:\.\d+)+", raw):
            raise ValueError(f"invalid version: {raw!r}")
        return cls(tuple(int(p) for p in raw.split(".")))

    def successors(self) -> tuple[Self, ...]:
        """Return the next patch, minor and major version, in that order."""
        major, minor, patch = (*self.parts, 0, 0)[:3]
        cls = type(self)
        return (
            cls((major, minor, patch + 1)),
            cls((major, minor + 1, 0)),
            cls((major + 1, 0, 0)),
        )

    def __str__(self) -> str:
        """Return the dotted form, e.g. 2.19.2."""
        return ".".join(map(str, self.parts))


@dataclass(frozen=True, slots=True, kw_only=True)
class Options:
    """Validated configuration, decoupled from argparse.Namespace."""

    base_dir: Path
    keep: int
    base_url: str
    index_url: str
    eula_cookie: str = field(repr=False)
    start_version: Version
    timeout: float
    dry_run: bool
    force: bool
    as_json: bool

    def archive_url(self, version: Version) -> str:
        """Return the download URL of *version*."""
        return f"{self.base_url}/additional/{archive_name(version)}"


def archive_name(version: Version) -> str:
    """Return the archive file name of *version*."""
    return f"sapcc-{version}-linux-x64.zip"


_ENV_HELP: Final = "\n  ".join(
    f"{env:<20} config key {key}" for key, env in ENV_VARS.items()
)
EPILOG: Final = f"""\
examples:
  %(prog)s -v
      Check once and update if needed, with progress messages.

  %(prog)s --dry-run
      Report whether an update is available without downloading.

  %(prog)s --json --keep 0
      Machine-readable result; keep every extracted release.

layout below the base directory:
  downloads/sapcc-<ver>-linux-x64.zip   downloaded archives
  releases/<ver>/                       extracted content
  current -> releases/<ver>             newest release
  .version                              last processed version
  .lock                                 instance lock

exit codes:
  0   success (updated, or already up to date)
  1   runtime failure, or another instance is running
  2   invalid command line or configuration
  130 interrupted (SIGINT/SIGTERM)
  141 output pipe closed (SIGPIPE)

configuration precedence:
  command line > environment > config file > built-in default

environment:
  SAPCC_UPDATE_CONFIG  config file (default: {DEFAULT_CONFIG})
  {_ENV_HELP}

files:
  {DEFAULT_CONFIG}  optional TOML file, same keys as above
"""


class HelpFormatter(argparse.RawDescriptionHelpFormatter):
    """Preserve the layout of description and epilog verbatim."""


def build_parser() -> argparse.ArgumentParser:
    """Return the fully configured argument parser."""
    parser = argparse.ArgumentParser(
        prog=PROG,
        description=(
            "Download and extract the newest SAP Cloud Connector\n"
            "(sapcc-*-linux-x64.zip) if it is newer than the last one processed."
        ),
        epilog=EPILOG,
        formatter_class=HelpFormatter,
    )
    parser.add_argument(
        "--version", action="version", version=f"%(prog)s {__version__}"
    )
    # Defaults are None so the config precedence can be applied afterwards.
    parser.add_argument(
        "-c",
        "--config",
        metavar="PATH",
        type=Path,
        help=f"TOML config file (default: {DEFAULT_CONFIG}, used if present)",
    )
    parser.add_argument(
        "-d",
        "--base-dir",
        metavar="DIR",
        help=f"download/extract directory (default: {DEFAULTS['base_dir']})",
    )
    parser.add_argument(
        "-k",
        "--keep",
        metavar="N",
        help=f"releases to keep, 0 = all (default: {DEFAULTS['keep']})",
    )
    parser.add_argument(
        "-n",
        "--dry-run",
        action="store_true",
        help="report what would be done without downloading",
    )
    parser.add_argument(
        "-f",
        "--force",
        action="store_true",
        help="download and extract even if the version is already current",
    )
    parser.add_argument(
        "--json",
        dest="as_json",
        action="store_true",
        help="print the result as JSON on stdout",
    )

    output = parser.add_argument_group("logging")
    output.add_argument(
        "-v",
        "--verbose",
        action="count",
        default=0,
        help="increase verbosity; repeat for debug output",
    )
    output.add_argument(
        "-q",
        "--quiet",
        action="store_true",
        help="suppress everything below errors",
    )
    return parser


def configure_logging(verbosity: int, *, quiet: bool) -> None:
    """Send diagnostics to stderr at a level derived from the flags."""
    if quiet:
        level = logging.ERROR
    elif verbosity >= 2:  # noqa: PLR2004 - -vv means debug
        level = logging.DEBUG
    elif verbosity == 1:
        level = logging.INFO
    else:
        level = logging.WARNING
    # journald supplies its own timestamps.
    if "JOURNAL_STREAM" in os.environ:
        fmt = "%(levelname)s: %(message)s"
    else:
        fmt = "%(asctime)s %(levelname)s: %(message)s"
    logging.basicConfig(level=level, format=fmt, stream=sys.stderr)


def load_config(path: Path | None) -> dict[str, str]:
    """Return the settings in the TOML file *path*, as strings.

    A missing default config file is not an error; a missing file that was
    named explicitly is.
    """
    explicit = path is not None or "SAPCC_UPDATE_CONFIG" in os.environ
    if path is None:
        path = Path(os.environ.get("SAPCC_UPDATE_CONFIG", DEFAULT_CONFIG))
    try:
        data = tomllib.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        if explicit:
            raise UsageError(f"config file not found: {path}") from None
        return {}
    except (OSError, UnicodeDecodeError) as exc:
        raise UsageError(f"cannot read config file {path}: {exc}") from exc
    except tomllib.TOMLDecodeError as exc:
        raise UsageError(f"invalid TOML in {path}: {exc}") from exc

    if unknown := sorted(data.keys() - DEFAULTS.keys()):
        raise UsageError(f"unknown key(s) in {path}: {', '.join(unknown)}")
    for key, value in data.items():
        if not isinstance(value, (str, int, float)) or isinstance(value, bool):
            raise UsageError(f"{path}: {key} must be a string or number")
    log.debug("loaded config %s", path)
    return {key: str(value) for key, value in data.items()}


def checked_url(key: str, raw: str) -> str:
    """Return *raw* without trailing slash if it is an http(s) URL."""
    parts = urlsplit(raw)
    if parts.scheme not in ("https", "http") or not parts.netloc:
        raise UsageError(f"{key} must be an https URL, got {raw!r}")
    if parts.scheme == "http":
        log.warning("%s uses plain http: %s", key, raw)
    return raw.rstrip("/") if key == "base_url" else raw


def resolve_options(args: argparse.Namespace) -> Options:
    """Merge command line, environment, config file and defaults."""
    settings = dict(DEFAULTS)
    settings.update(load_config(args.config))
    settings.update(
        {key: os.environ[env] for key, env in ENV_VARS.items() if env in os.environ}
    )
    if args.base_dir is not None:
        settings["base_dir"] = args.base_dir
    if args.keep is not None:
        settings["keep"] = args.keep

    try:
        keep = int(settings["keep"])
        timeout = float(settings["timeout"])
        start_version = Version.parse(settings["start_version"])
    except ValueError as exc:
        raise UsageError(f"invalid setting: {exc}") from exc
    if keep < 0 or timeout <= 0:
        raise UsageError("keep must be >= 0 and timeout must be > 0")
    if "\n" in settings["eula_cookie"] or "\r" in settings["eula_cookie"]:
        raise UsageError("eula_cookie must not contain line breaks")

    return Options(
        base_dir=Path(settings["base_dir"]).expanduser().absolute(),
        keep=keep,
        base_url=checked_url("base_url", settings["base_url"]),
        index_url=checked_url("index_url", settings["index_url"]),
        eula_cookie=settings["eula_cookie"],
        start_version=start_version,
        timeout=timeout,
        dry_run=args.dry_run,
        force=args.force,
        as_json=args.as_json,
    )


def acquire_lock(path: Path) -> TextIO:
    """Take an exclusive lock, or raise if another instance holds it."""
    try:
        handle = path.open("w", encoding="utf-8")
    except OSError as exc:
        raise Error(f"cannot create lock file {path}: {exc.strerror}") from exc
    try:
        fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        handle.close()
        raise Error(f"another {PROG} instance is already running ({path})") from None
    return handle


def install_signal_handlers() -> None:
    """Turn SIGTERM into an ordinary exception so cleanup runs."""

    def terminate(signum: int, _frame: object) -> None:
        raise KeyboardInterrupt(f"received signal {signum}")

    signal.signal(signal.SIGTERM, terminate)


def open_url(
    opts: Options, url: str, *, method: str = "GET"
) -> http.client.HTTPResponse:
    """Open *url* with the EULA cookie, TLS verification and a timeout."""
    request = urllib.request.Request(
        url,
        method=method,
        headers={
            "Cookie": opts.eula_cookie,
            "User-Agent": f"{PROG}/{__version__}",
        },
    )
    context = ssl.create_default_context()
    log.debug("%s %s", method, url)
    response: http.client.HTTPResponse = urllib.request.urlopen(
        request, timeout=opts.timeout, context=context
    )
    return response


def https_proxy() -> str | None:
    """Return host:port of the configured HTTPS proxy, without credentials."""
    raw = urllib.request.getproxies().get("https")
    if not raw:
        return None
    parts = urlsplit(raw if "://" in raw else f"http://{raw}")
    return f"{parts.hostname}:{parts.port}" if parts.port else parts.hostname


def eula_cookie(license_version: tuple[int, int]) -> str:
    """Return the cookie that confirms acceptance of *license_version*."""
    tag = "{}_{}".format(*license_version)
    return f"eula_{tag}_agreed=tools.hana.ondemand.com/developer-license-{tag}.txt"


def license_on_page(page: str) -> tuple[int, int] | None:
    """Return the newest developer licence version linked on *page*, if any."""
    found = {(int(a), int(b)) for a, b in LICENSE_RE.findall(page)}
    return max(found, default=None)


def read_index(opts: Options) -> str | None:
    """Return the download page, or None if the server answers with an error.

    An HTTP error answer returns None so that probing can take over; a
    network failure raises, since probing the same host would fail too.
    """
    log.info("reading download page %s (timeout %gs)", opts.index_url, opts.timeout)
    try:
        with open_url(opts, opts.index_url) as response:
            page = response.read(MAX_INDEX_BYTES).decode("utf-8", errors="replace")
    except urllib.error.HTTPError as exc:
        log.warning("download page %s: HTTP %s", opts.index_url, exc.code)
        return None
    except (urllib.error.URLError, TimeoutError, OSError) as exc:
        reason = exc.reason if isinstance(exc, urllib.error.URLError) else exc
        msg = f"cannot reach {opts.index_url}: {reason}"
        if https_proxy() is None:
            msg += " (no proxy configured; set https_proxy if this host needs one)"
        raise Error(msg) from exc
    return page


def archive_exists(opts: Options, version: Version) -> bool:
    """Return True if the archive of *version* is downloadable."""
    try:
        with open_url(opts, opts.archive_url(version), method="HEAD"):
            return True
    except urllib.error.HTTPError as exc:
        if exc.code in (403, 404, 410):
            return False
        raise Error(f"probing {version} failed: HTTP {exc.code}") from exc
    except (urllib.error.URLError, TimeoutError, OSError) as exc:
        raise Error(f"probing {version} failed: {exc}") from exc


def latest_by_probing(opts: Options, start: Version) -> Version | None:
    """Return the newest version reachable by stepping from *start*."""
    current = start if archive_exists(opts, start) else None
    base = start
    while True:
        found = next((v for v in base.successors() if archive_exists(opts, v)), None)
        if found is None:
            return current
        current = base = found


def ensure_base_dir(path: Path) -> None:
    """Create *path* if needed and make sure nobody else controls it.

    The default lives in world-writable /tmp, where another user could
    pre-create the directory, or a symlink to one of our own directories, and
    make us write into or extract to a place of their choosing.
    """
    try:
        path.mkdir(mode=0o755, parents=True, exist_ok=True)
    except OSError as exc:
        msg = f"cannot create {path}: {exc.strerror}; choose another with --base-dir"
        raise Error(msg) from exc
    euid = os.geteuid()
    link = path.lstat()
    if stat.S_ISLNK(link.st_mode) and link.st_uid not in (euid, 0):
        msg = f"{path} is a symlink owned by uid {link.st_uid}, refusing to use it"
        raise Error(msg)
    target = path.stat()
    if not stat.S_ISDIR(target.st_mode):
        raise Error(f"{path} is not a directory")
    if target.st_uid != euid:
        msg = f"{path} is owned by uid {target.st_uid}, not by you (uid {euid})"
        raise Error(f"{msg}; choose another with --base-dir")
    if target.st_mode & 0o022:
        raise Error(f"{path} is writable by group or others, refusing to use it")


def read_state(path: Path) -> Version | None:
    """Return the last processed version stored in *path*, if any."""
    try:
        return Version.parse(path.read_text(encoding="utf-8").strip())
    except FileNotFoundError:
        return None
    except (OSError, ValueError) as exc:
        raise Error(f"cannot read state file {path}: {exc}") from exc


def write_atomic(path: Path, text: str, mode: int = 0o644) -> None:
    """Replace *path* with *text* atomically."""
    fd, tmp = tempfile.mkstemp(dir=path.parent, prefix=f".{path.name}.")
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as handle:
            handle.write(text)
            handle.flush()
            os.fsync(handle.fileno())
        Path(tmp).chmod(mode)
        Path(tmp).replace(path)
    except BaseException:
        Path(tmp).unlink(missing_ok=True)
        raise


def download(opts: Options, url: str, target: Path) -> None:
    """Download *url* to *target* atomically, retrying transient failures."""
    for attempt in range(1, DOWNLOAD_ATTEMPTS + 1):
        fd, tmp = tempfile.mkstemp(dir=target.parent, prefix=".sapcc-", suffix=".part")
        try:
            with os.fdopen(fd, "wb") as out, open_url(opts, url) as response:
                expected = response.headers.get("Content-Length")
                content_type = response.headers.get_content_type()
                final_url = response.geturl()
                shutil.copyfileobj(response, out, CHUNK)
                out.flush()
                os.fsync(out.fileno())
                size = out.tell()
            if expected is not None and int(expected) != size:
                raise OSError(f"incomplete download: {size} of {expected} bytes")
            if not zipfile.is_zipfile(tmp):
                saved = target.parent / LAST_RESPONSE
                Path(tmp).replace(saved)
                raise Error(not_a_zip(url, final_url, content_type, size, saved))
            Path(tmp).chmod(0o644)
            Path(tmp).replace(target)
            (target.parent / LAST_RESPONSE).unlink(missing_ok=True)
            log.info("downloaded %s (%d bytes)", target.name, size)
            return
        except urllib.error.HTTPError as exc:
            Path(tmp).unlink(missing_ok=True)
            if exc.code < HTTP_SERVER_ERROR or attempt == DOWNLOAD_ATTEMPTS:
                raise Error(f"download of {url} failed: HTTP {exc.code}") from exc
            failure: Exception = exc
        except (urllib.error.URLError, TimeoutError, OSError) as exc:
            Path(tmp).unlink(missing_ok=True)
            if attempt == DOWNLOAD_ATTEMPTS:
                raise Error(f"download of {url} failed: {exc}") from exc
            failure = exc
        except BaseException:
            Path(tmp).unlink(missing_ok=True)
            raise
        delay = RETRY_DELAY_S * attempt
        log.warning(
            "attempt %d failed (%s), retrying in %.0fs", attempt, failure, delay
        )
        time.sleep(delay)


def not_a_zip(
    url: str, final_url: str, content_type: str, size: int, saved: Path
) -> str:
    """Describe a response that should have been a ZIP archive."""
    head = saved.read_bytes()[:65536].decode("utf-8", errors="replace")
    title = re.search(r"<title[^>]*>(.*?)</title>", head, re.IGNORECASE | re.DOTALL)
    lines = [
        f"{url} did not return a ZIP archive",
        f"  received: {content_type}, {size} bytes",
    ]
    if final_url != url:
        lines.append(f"  redirected to: {final_url}")
    if title:
        lines.append(f"  page title: {' '.join(title.group(1).split())[:120]}")
    lines += [
        f"  response saved as: {saved}",
        "  likely causes: SAP did not accept the licence cookie (see eula_cookie),",
        "  or the proxy blocked the download",
    ]
    return "\n".join(lines)


def extract(archive: Path, target: Path) -> None:
    """Verify *archive* and extract it to *target*, replacing any old copy."""
    staging = Path(tempfile.mkdtemp(dir=target.parent, prefix=".extract-"))
    try:
        with zipfile.ZipFile(archive) as zf:
            if (bad := zf.testzip()) is not None:
                raise Error(f"corrupt member in {archive.name}: {bad}")
            members = zf.infolist()
            if sum(m.file_size for m in members) > MAX_UNPACKED_BYTES:
                raise Error(f"{archive.name} unpacks to more than the size limit")
            root = staging.resolve()
            for member in members:
                unix_mode = member.external_attr >> 16
                dest = (staging / member.filename).resolve()
                if (
                    stat.S_ISLNK(unix_mode)
                    or Path(member.filename).is_absolute()
                    or not dest.is_relative_to(root)
                ):
                    raise Error(f"unsafe path in {archive.name}: {member.filename!r}")
                zf.extract(member, staging)
                if not member.is_dir():
                    # Keep execute bits, never setuid/setgid/sticky.
                    dest.chmod(0o755 if unix_mode & 0o111 else 0o644)
        staging.chmod(0o755)
        if target.exists():
            shutil.rmtree(target)
        staging.replace(target)
    except BaseException:
        shutil.rmtree(staging, ignore_errors=True)
        raise


def update_symlink(link: Path, target: str) -> None:
    """Point *link* at *target* atomically."""
    tmp = link.with_name(f".{link.name}.{os.getpid()}")
    tmp.unlink(missing_ok=True)
    tmp.symlink_to(target)
    tmp.replace(link)


def prune(opts: Options, releases: Path, downloads: Path) -> None:
    """Delete all but the newest ``opts.keep`` releases and their archives."""
    if opts.keep == 0:
        return
    found: list[Version] = []
    for entry in releases.iterdir():
        if entry.is_dir() and not entry.is_symlink():
            try:
                found.append(Version.parse(entry.name))
            except ValueError:
                continue
    for version in sorted(found)[: -opts.keep]:
        log.info("pruning old release %s", version)
        shutil.rmtree(releases / str(version))
        (downloads / archive_name(version)).unlink(missing_ok=True)


def run(opts: Options) -> dict[str, str | None]:
    """Check for, download and extract a newer version; return the result."""
    downloads = opts.base_dir / "downloads"
    releases = opts.base_dir / "releases"
    state_file = opts.base_dir / ".version"
    if not opts.dry_run:
        for directory in (downloads, releases):
            try:
                directory.mkdir(mode=0o755, parents=True, exist_ok=True)
            except OSError as exc:
                raise Error(f"cannot create {directory}: {exc.strerror}") from exc

    installed = read_state(state_file)
    page = read_index(opts) or ""
    latest = max((Version.parse(v) for v in ARCHIVE_RE.findall(page)), default=None)
    if opts.eula_cookie == "auto":
        if (license_version := license_on_page(page)) is None:
            log.info("no licence link on download page, assuming the default")
            license_version = FALLBACK_LICENSE
        opts = dataclasses.replace(opts, eula_cookie=eula_cookie(license_version))
    log.info("licence cookie: %s", opts.eula_cookie)
    if latest is not None:
        log.info("latest version on download page: %s", latest)
    else:
        log.warning("no version found on download page, probing instead")
        latest = latest_by_probing(opts, installed or opts.start_version)
        if latest is None:
            raise Error("no SAP Cloud Connector version found")
        log.info("latest version found by probing: %s", latest)

    result: dict[str, str | None] = {
        "installed": str(installed) if installed else None,
        "latest": str(latest),
        "path": None,
    }
    if installed is not None and latest <= installed and not opts.force:
        log.info("already up to date (%s)", installed)
        return result | {"status": "current"}

    url = opts.archive_url(latest)
    release = releases / str(latest)
    if opts.dry_run:
        log.warning("dry run: would download %s and extract to %s", url, release)
        return result | {"status": "dry-run"}

    log.info("update available: %s -> %s", installed or "none", latest)
    archive = downloads / archive_name(latest)
    download(opts, url, archive)
    extract(archive, release)
    update_symlink(opts.base_dir / "current", f"releases/{latest}")
    write_atomic(state_file, f"{latest}\n")
    prune(opts, releases, downloads)
    return result | {"status": "updated", "path": str(release)}


def report(result: Mapping[str, str | None], *, as_json: bool) -> None:
    """Write the result to stdout."""
    if as_json:
        json.dump(result, sys.stdout)
        sys.stdout.write("\n")
        return
    match result["status"]:
        case "updated":
            print(
                f"updated {result['installed'] or 'none'} -> {result['latest']}: "
                f"{result['path']}"
            )
        case "dry-run":
            print(
                f"update available: {result['installed'] or 'none'} -> "
                f"{result['latest']}"
            )
        case _:
            print(f"up to date: {result['installed']}")


def main(argv: Sequence[str] | None = None) -> int:
    """Parse arguments, dispatch, and translate failures into exit codes."""
    parser = build_parser()
    args = parser.parse_args(argv)
    configure_logging(args.verbose, quiet=args.quiet)
    install_signal_handlers()

    try:
        opts = resolve_options(args)
        log.info(
            "%s %s on Python %s, base dir %s, https proxy %s",
            PROG,
            __version__,
            sys.version.split()[0],
            opts.base_dir,
            https_proxy() or "none",
        )
        lock = None
        if not opts.dry_run:
            ensure_base_dir(opts.base_dir)
            lock = acquire_lock(opts.base_dir / ".lock")
        try:
            result = run(opts)
        finally:
            if lock is not None:
                lock.close()
        report(result, as_json=opts.as_json)
        sys.stdout.flush()
        return Exit.OK
    except UsageError as exc:
        parser.error(str(exc))  # exits 2 with the usage line attached
    except Error as exc:
        log.error("%s", exc)
        return Exit.FAILURE
    except KeyboardInterrupt:
        log.warning("interrupted")
        return Exit.INTERRUPTED
    except BrokenPipeError:
        os.dup2(os.open(os.devnull, os.O_WRONLY), sys.stdout.fileno())
        return Exit.BROKEN_PIPE
    except OSError as exc:
        if exc.errno in (errno.EACCES, errno.EPERM, errno.ENOSPC, errno.EROFS):
            log.error("%s", exc)
            return Exit.FAILURE
        raise


if __name__ == "__main__":
    sys.exit(main())
