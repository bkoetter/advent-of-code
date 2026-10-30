# sapcc-update

Downloads the newest SAP Cloud Connector for Linux x64
(`https://tools.hana.ondemand.com/additional/sapcc-<version>-linux-x64.zip`)
and extracts it, but only if that version is newer than the last one processed.

Single-file Python script (3.11 or newer), stdlib only.

## How it works

1. Reads `https://tools.hana.ondemand.com/` and picks the highest
   `sapcc-<version>-linux-x64.zip` link.
2. Fallback if the page yields nothing: probes the next patch, minor and major
   version with HEAD requests (e.g. `2.19.3`, `2.20.0`, `3.0.0`).
3. Compares with `<base>/.version`; exits 0 if nothing is newer.
4. Downloads with the SAP EULA cookie into a temp file, checks size and ZIP
   integrity, rejects unsafe paths/symlinks, extracts into
   `<base>/releases/<version>`, points `<base>/current` at it and keeps the
   newest 3 releases (`--keep`).

Only one instance runs at a time (`flock` on `<base>/.lock`).
See `sapcc-update --help` for options, exit codes and environment variables.

## Installation (SLES / openSUSE)

Requires Python 3.11 or newer. The shebang is `/usr/bin/python3.11`; on
SLES 15 that comes from the Python 3 module:

```bash
sudo SUSEConnect -p sle-module-python3/15.7/x86_64
sudo zypper install python311
```

Adjust the shebang if you use a different interpreter.

```bash
sudo install -m 0755 sapcc-update.py /usr/local/sbin/sapcc-update
sudo install -d -m 0755 /opt/sapcc-download /etc/sapcc-update
sudo install -m 0644 sapcc-update.toml.example /etc/sapcc-update/sapcc-update.toml
sudo install -m 0644 sapcc-update.service sapcc-update.timer /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now sapcc-update.timer
```

`/opt/sapcc-download` must exist before the service starts, because the unit
uses `ProtectSystem=strict` with `ReadWritePaths=`. If you change `base_dir`,
change `ReadWritePaths=` too.

Test first: `sudo sapcc-update --dry-run -v`
Logs: `journalctl -u sapcc-update.service`

## Development

```bash
python3.11 -m unittest discover -s tests -v
ruff check . && ruff format --check . && mypy sapcc-update.py tests
```

## Notes

- Downloading means you accept the SAP Developer License; the cookie is
  configurable (`eula_cookie` / `SAPCC_EULA_COOKIE`) should SAP change it.
- The ZIP contains the installer RPM; the script does not install it.
