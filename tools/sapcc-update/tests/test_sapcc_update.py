"""Tests for sapcc-update, run with: python3 -m unittest discover -s tests."""

import contextlib
import fcntl
import functools
import http.server
import importlib.util
import io
import json
import os
import shutil
import socket
import tempfile
import threading
import unittest
import zipfile
from collections.abc import Iterator
from pathlib import Path
from types import ModuleType
from unittest import mock

SCRIPT = Path(__file__).resolve().parent.parent / "sapcc-update.py"


def _load() -> ModuleType:
    spec = importlib.util.spec_from_file_location("sapcc_update", SCRIPT)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


su = _load()


EULA_PAGE = b"<html><head><title>SAP Development Tools - EULA</title></head></html>"


class QuietHandler(http.server.SimpleHTTPRequestHandler):
    """Static file handler that mimics SAP's licence check on downloads.

    If the server has a ``required_cookie`` attribute, archive requests
    without exactly that Cookie header get the licence page, as SAP does.
    """

    def log_message(self, format: str, *args: object) -> None:  # noqa: A002
        """Discard the access log."""

    def _licence_page_instead(self) -> bool:
        required = getattr(self.server, "required_cookie", None)
        if not self.path.startswith("/additional/") or required is None:
            return False
        if self.headers.get("Cookie") == required:
            return False
        self.send_response(200)
        self.send_header("Content-Type", "text/html")
        self.send_header("Content-Length", str(len(EULA_PAGE)))
        self.end_headers()
        if self.command == "GET":
            self.wfile.write(EULA_PAGE)
        return True

    def do_GET(self) -> None:
        """Serve the licence page or the requested file."""
        if not self._licence_page_instead():
            super().do_GET()

    def do_HEAD(self) -> None:
        """Serve the licence page headers or the requested file's."""
        if not self._licence_page_instead():
            super().do_HEAD()


class VersionTest(unittest.TestCase):
    """Version parsing and ordering."""

    def test_numeric_ordering(self) -> None:
        self.assertLess(su.Version.parse("2.9.9"), su.Version.parse("2.10.0"))
        self.assertEqual(str(su.Version.parse("2.19.2")), "2.19.2")

    def test_rejects_garbage(self) -> None:
        for raw in ("", "2", "2.x", "2.19.2-beta", "../2.1"):
            with self.subTest(raw=raw), self.assertRaises(ValueError):
                su.Version.parse(raw)

    def test_successors(self) -> None:
        got = [str(v) for v in su.Version.parse("2.19.2").successors()]
        self.assertEqual(got, ["2.19.3", "2.20.0", "3.0.0"])


class ParserAndConfigTest(unittest.TestCase):
    """Command line and configuration handling."""

    def test_usage_error_exits_2(self) -> None:
        with (
            contextlib.redirect_stderr(io.StringIO()),
            self.assertRaises(SystemExit) as cm,
        ):
            su.main(["--bogus"])
        self.assertEqual(cm.exception.code, su.Exit.USAGE)

    def test_invalid_keep_exits_2(self) -> None:
        with (
            contextlib.redirect_stderr(io.StringIO()),
            self.assertRaises(SystemExit) as cm,
        ):
            su.main(["--keep", "-1", "--config", os.devnull])
        self.assertEqual(cm.exception.code, su.Exit.USAGE)

    def test_unknown_config_key(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            cfg = Path(tmp, "c.toml")
            cfg.write_text('base_dri = "/x"\n', encoding="utf-8")
            with self.assertRaisesRegex(su.UsageError, "base_dri"):
                su.load_config(cfg)

    def test_rejects_file_scheme(self) -> None:
        with self.assertRaises(su.UsageError):
            su.checked_url("index_url", "file:///etc/passwd")

    def test_precedence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            cfg = Path(tmp, "c.toml")
            cfg.write_text('keep = 5\nbase_dir = "/from/config"\n', encoding="utf-8")
            args = su.build_parser().parse_args(["-c", str(cfg), "-d", "/from/cli"])
            with mock.patch.dict(os.environ, {"SAPCC_KEEP": "7"}):
                opts = su.resolve_options(args)
        self.assertEqual(opts.base_dir, Path("/from/cli"))
        self.assertEqual(opts.keep, 7)


class ExtractTest(unittest.TestCase):
    """Archive safety checks."""

    def test_rejects_path_traversal(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            archive = Path(tmp, "evil.zip")
            with zipfile.ZipFile(archive, "w") as zf:
                zf.writestr("../escape.txt", "x")
            releases = Path(tmp, "releases")
            releases.mkdir()
            with self.assertRaisesRegex(su.Error, "unsafe path"):
                su.extract(archive, releases / "1.0.0")
            self.assertEqual(list(releases.iterdir()), [])
            self.assertFalse(Path(tmp, "escape.txt").exists())


def licence_js(name: str, value: str) -> str:
    """Return the page JavaScript that defines the Cloud Connector licence."""
    return (
        f"eulaConst.cloudConnectorLicense.cookieName = '{name}'\n"
        f"eulaConst.cloudConnectorLicense.cookieValue = '{value}'\n"
    )


class LicenceTest(unittest.TestCase):
    """Licence cookie detection and the not-a-ZIP diagnosis."""

    def test_cookie_from_page(self) -> None:
        self.assertEqual(
            su.eula_cookie_on_page(licence_js("x_agreed", "a/b-1.2.txt")),
            "x_agreed=a/b-1.2.txt",
        )

    def test_ignores_other_licences(self) -> None:
        page = "eulaConst.devLicense.cookieName = 'eula_3_2_agreed'"
        self.assertIsNone(su.eula_cookie_on_page(page))

    def test_rejects_unsafe_characters(self) -> None:
        self.assertIsNone(su.eula_cookie_on_page(licence_js("a;b", "c")))

    def test_real_page_snippet(self) -> None:
        page = (
            "eulaConst.devLicense.cookieName = 'eula_3_2_agreed'\n"
            "eulaConst.devLicense.cookieValue = "
            "'tools.hana.ondemand.com/developer-license-3_2.txt'\n"
            "eulaConst.cloudConnectorLicense.cookieName = "
            "'eula_cloud_connector_license_v1_2026_agreed'\n"
            "eulaConst.cloudConnectorLicense.cookieValue = "
            "'tools.hana.ondemand.com/cloud-connector-agreement_v1.2026.txt'\n"
        )
        self.assertEqual(su.eula_cookie_on_page(page), su.FALLBACK_EULA_COOKIE)

    def test_not_a_zip_message(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            saved = Path(tmp, "r.html")
            saved.write_bytes(EULA_PAGE)
            msg = su.not_a_zip(
                "https://a/x.zip", "https://a/eula", "text/html", 9, saved
            )
        self.assertIn("text/html, 9 bytes", msg)
        self.assertIn("redirected to: https://a/eula", msg)
        self.assertIn("page title: SAP Development Tools - EULA", msg)
        self.assertIn(str(saved), msg)


class BaseDirTest(unittest.TestCase):
    """Ownership and permission checks on the base directory."""

    def setUp(self) -> None:
        self.tmp = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, self.tmp)

    def test_default_is_in_tmp(self) -> None:
        self.assertEqual(su.DEFAULTS["base_dir"], "/tmp/sapcc-update")  # noqa: S108

    def test_creates_missing_directory(self) -> None:
        base = self.tmp / "a" / "b"
        su.ensure_base_dir(base)
        self.assertTrue(base.is_dir())

    def test_rejects_world_writable(self) -> None:
        base = self.tmp / "open"
        base.mkdir()
        base.chmod(0o777)
        with self.assertRaisesRegex(su.Error, "writable by group or others"):
            su.ensure_base_dir(base)

    def test_rejects_foreign_owner(self) -> None:
        with (
            mock.patch.object(su.os, "geteuid", return_value=os.geteuid() + 1),
            self.assertRaisesRegex(su.Error, "--base-dir"),
        ):
            su.ensure_base_dir(self.tmp)

    def test_rejects_non_directory(self) -> None:
        target = self.tmp / "file"
        target.write_text("", encoding="utf-8")
        with self.assertRaises(su.Error):
            su.ensure_base_dir(target)

    def test_permission_denied_suggests_base_dir(self) -> None:
        with (
            mock.patch.object(
                Path, "mkdir", side_effect=PermissionError(13, "Permission denied")
            ),
            self.assertRaisesRegex(
                su.Error, "Permission denied; choose another with --base-dir"
            ),
        ):
            su.ensure_base_dir(self.tmp / "x")


class EndToEndTest(unittest.TestCase):
    """Full runs against a local HTTP server mimicking the download page."""

    def setUp(self) -> None:
        self.tmp = Path(tempfile.mkdtemp())
        self.www = self.tmp / "www"
        (self.www / "additional").mkdir(parents=True)
        self.base = self.tmp / "base"
        handler = functools.partial(QuietHandler, directory=str(self.www))
        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
        threading.Thread(target=self.server.serve_forever, daemon=True).start()
        url = f"http://127.0.0.1:{self.server.server_port}"
        env = {
            "SAPCC_BASE_URL": url,
            "SAPCC_INDEX_URL": f"{url}/",
            "SAPCC_UPDATE_CONFIG": os.devnull,
            "NO_PROXY": "127.0.0.1",
            "no_proxy": "127.0.0.1",
        }
        patcher = mock.patch.dict(os.environ, env)
        patcher.start()
        self.addCleanup(patcher.stop)

    def tearDown(self) -> None:
        self.server.shutdown()
        self.server.server_close()
        shutil.rmtree(self.tmp)

    def publish(self, version: str, *, link: bool = True) -> None:
        with zipfile.ZipFile(
            self.www / f"additional/{su.archive_name(version)}", "w"
        ) as zf:
            zf.writestr(f"com.sap.scc-ui-{version}.x86_64.rpm", "rpm")
        if link:
            with (self.www / "index.html").open("a", encoding="utf-8") as fh:
                fh.write(f'<a href="additional/{su.archive_name(version)}">x</a>\n')

    @contextlib.contextmanager
    def captured(self) -> Iterator[io.StringIO]:
        out = io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(io.StringIO()):
            yield out

    def run_json(self, *argv: str) -> tuple[int, dict[str, object]]:
        with self.captured() as out:
            rc = su.main(["--json", "-d", str(self.base), *argv])
        return rc, json.loads(out.getvalue()) if out.getvalue() else {}

    def test_update_cycle(self) -> None:
        self.publish("2.18.4")
        self.publish("2.19.2")
        rc, res = self.run_json()
        self.assertEqual((rc, res["status"], res["latest"]), (0, "updated", "2.19.2"))
        self.assertTrue(
            (self.base / "current/com.sap.scc-ui-2.19.2.x86_64.rpm").is_file()
        )

        rc, res = self.run_json()
        self.assertEqual((rc, res["status"]), (0, "current"))

        self.publish("2.20.0")
        rc, res = self.run_json("--dry-run")
        self.assertEqual((rc, res["status"]), (0, "dry-run"))
        self.assertFalse((self.base / "releases/2.20.0").exists())

        rc, res = self.run_json("--keep", "1")
        self.assertEqual(
            (rc, res["status"], res["installed"]), (0, "updated", "2.19.2")
        )
        self.assertEqual(str((self.base / "current").readlink()), "releases/2.20.0")
        self.assertEqual(
            [p.name for p in (self.base / "releases").iterdir()], ["2.20.0"]
        )
        self.assertEqual(
            [p.name for p in (self.base / "downloads").iterdir()],
            [su.archive_name("2.20.0")],
        )
        self.assertEqual(
            (self.base / ".version").read_text(encoding="utf-8"), "2.20.0\n"
        )

    def test_probing_fallback(self) -> None:
        self.publish("2.19.2", link=False)
        self.publish("2.19.3", link=False)
        self.publish("2.20.0", link=False)
        (self.www / "index.html").write_text("no links here", encoding="utf-8")
        rc, res = self.run_json()
        self.assertEqual((rc, res["latest"]), (0, "2.20.0"))

    def test_non_zip_response_fails(self) -> None:
        (self.www / "index.html").write_text(
            '<a href="additional/sapcc-2.19.2-linux-x64.zip">', encoding="utf-8"
        )
        (self.www / "additional/sapcc-2.19.2-linux-x64.zip").write_text(
            "<html>accept the EULA</html>", encoding="utf-8"
        )
        rc, _ = self.run_json()
        self.assertEqual(rc, su.Exit.FAILURE)
        self.assertFalse((self.base / ".version").exists())
        saved = self.base / "downloads" / su.LAST_RESPONSE
        self.assertEqual(list((self.base / "downloads").iterdir()), [saved])
        self.assertEqual(saved.stat().st_mode & 0o777, 0o600)

    def test_licence_cookie_taken_from_page(self) -> None:
        self.publish("2.19.2")
        with (self.www / "index.html").open("a", encoding="utf-8") as fh:
            fh.write(licence_js("eula_cc_v9_agreed", "example/cc-v9.txt"))
        self.server.required_cookie = "eula_cc_v9_agreed=example/cc-v9.txt"  # type: ignore[attr-defined]
        rc, res = self.run_json()
        self.assertEqual((rc, res["status"]), (0, "updated"))

    def test_explicit_cookie_overrides_page(self) -> None:
        self.publish("2.19.2")
        with (self.www / "index.html").open("a", encoding="utf-8") as fh:
            fh.write(licence_js("eula_cc_v9_agreed", "example/cc-v9.txt"))
        self.server.required_cookie = "eula_cc_v9_agreed=example/cc-v9.txt"  # type: ignore[attr-defined]
        with mock.patch.dict(os.environ, {"SAPCC_EULA_COOKIE": "wrong=1"}):
            rc, _ = self.run_json()
        self.assertEqual(rc, su.Exit.FAILURE)
        saved = self.base / "downloads" / su.LAST_RESPONSE
        self.assertEqual(saved.read_bytes(), EULA_PAGE)

    def test_success_removes_saved_failure(self) -> None:
        self.publish("2.19.2")
        self.server.required_cookie = "eula_cc_v9_agreed=example/cc-v9.txt"  # type: ignore[attr-defined]
        rc, _ = self.run_json()
        self.assertEqual(rc, su.Exit.FAILURE)
        with (self.www / "index.html").open("a", encoding="utf-8") as fh:
            fh.write(licence_js("eula_cc_v9_agreed", "example/cc-v9.txt"))
        rc, _ = self.run_json()
        self.assertEqual(rc, su.Exit.OK)
        self.assertFalse((self.base / "downloads" / su.LAST_RESPONSE).exists())

    def test_second_instance_fails(self) -> None:
        self.publish("2.19.2")
        self.base.mkdir()
        with (self.base / ".lock").open("w", encoding="utf-8") as held:
            fcntl.flock(held, fcntl.LOCK_EX)
            rc, _ = self.run_json()
        self.assertEqual(rc, su.Exit.FAILURE)

    def test_index_http_error_falls_back_to_probing(self) -> None:
        self.publish("2.19.2", link=False)
        url = f"http://127.0.0.1:{self.server.server_port}"
        with mock.patch.dict(os.environ, {"SAPCC_INDEX_URL": f"{url}/missing"}):
            rc, res = self.run_json()
        self.assertEqual((rc, res["latest"]), (0, "2.19.2"))


class UnreachableHostTest(unittest.TestCase):
    """A host that accepts connections but never answers."""

    def test_fails_fast_with_proxy_hint(self) -> None:
        listener = socket.create_server(("127.0.0.1", 0))
        self.addCleanup(listener.close)
        url = f"http://127.0.0.1:{listener.getsockname()[1]}"
        env = {
            "SAPCC_BASE_URL": url,
            "SAPCC_INDEX_URL": f"{url}/",
            "SAPCC_UPDATE_CONFIG": os.devnull,
            "SAPCC_TIMEOUT": "0.5",
            "NO_PROXY": "127.0.0.1",
            "no_proxy": "127.0.0.1",
        }
        proxy_vars = ("https_proxy", "HTTPS_PROXY", "http_proxy", "HTTP_PROXY")
        err = io.StringIO()
        with (
            tempfile.TemporaryDirectory() as tmp,
            mock.patch.dict(os.environ, env),
            contextlib.redirect_stderr(err),
            contextlib.redirect_stdout(io.StringIO()),
        ):
            for var in proxy_vars:
                os.environ.pop(var, None)
            logging_handlers = list(su.logging.root.handlers)
            for handler in logging_handlers:
                su.logging.root.removeHandler(handler)
            try:
                rc = su.main(["-d", tmp, "--dry-run"])
            finally:
                for handler in list(su.logging.root.handlers):
                    su.logging.root.removeHandler(handler)
                for handler in logging_handlers:
                    su.logging.root.addHandler(handler)
        self.assertEqual(rc, su.Exit.FAILURE)
        self.assertIn("cannot reach", err.getvalue())
        self.assertIn("https_proxy", err.getvalue())
        self.assertNotIn("probing", err.getvalue())


if __name__ == "__main__":
    unittest.main()
