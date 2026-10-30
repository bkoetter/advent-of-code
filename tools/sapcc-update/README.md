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

## Download location

Default: `/tmp/sapcc-update`. Change it with, highest priority first:

- `-d DIR` / `--base-dir DIR`
- environment variable `SAPCC_BASE_DIR=DIR`
- `base_dir = "DIR"` in `/etc/sapcc-update/sapcc-update.toml`

The directory must be owned by the user running the script and must not be
writable by group or others; otherwise the script refuses to use it. This
matters in `/tmp`, where any user could create `/tmp/sapcc-update` first.
Files in `/tmp` can be removed by a reboot or by `systemd-tmpfiles`
cleanup; the script then just downloads the current version again.

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
sudo install -d -m 0755 /etc/sapcc-update
sudo install -m 0644 sapcc-update.toml.example /etc/sapcc-update/sapcc-update.toml
sudo install -m 0644 sapcc-update.service sapcc-update.timer /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now sapcc-update.timer
```

The service uses `ProtectSystem=strict`, so only `ReadWritePaths=` is
writable. If you change the base directory, change `ReadWritePaths=` too, and
create the directory beforehand.

Test first: `sudo sapcc-update --dry-run -v`
Logs: `journalctl -u sapcc-update.service`

## Proxy

The script uses the standard `https_proxy` / `no_proxy` environment
variables. Hosts behind a corporate firewall usually need one; without it the
script waits for the timeout (default 60 s, `timeout` / `SAPCC_TIMEOUT`) and
then fails with `cannot reach ...`. For the systemd service, add to the unit:

```ini
Environment=https_proxy=http://proxy.example.com:8080
```

## Development

```bash
python3.11 -m unittest discover -s tests -v
ruff check . && ruff format --check . && mypy sapcc-update.py tests
```

## Notes

- Downloading means you accept the SAP Cloud Connector licence. SAP only
  hands out the ZIP if the request carries the cookie for the current version
  of that licence (its name contains the year, e.g.
  `eula_cloud_connector_license_v1_2026_agreed`). By default
  (`eula_cookie = "auto"`) the script reads the cookie from the download
  page's `eulaConst.cloudConnectorLicense` definition, falling back to the
  2026 one. Set `eula_cookie` / `SAPCC_EULA_COOKIE` to override it.
- If SAP or the proxy answers with something other than a ZIP, the error
  shows the content type, any redirect and the page title, and the response
  is kept as `<base>/downloads/last-failed-response.html` (mode 0600).
- The ZIP contains the installer RPM; the script does not install it.
