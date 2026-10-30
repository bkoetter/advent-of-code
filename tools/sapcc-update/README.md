# sapcc-update

Downloads the newest SAP Cloud Connector for Linux x64
(`https://tools.hana.ondemand.com/additional/sapcc-<version>-linux-x64.zip`)
and extracts it — but only if the version is newer than the last one processed.

## How it works

1. Reads the download page `https://tools.hana.ondemand.com/` and picks the
   highest `sapcc-<version>-linux-x64.zip` link.
2. Fallback if the page can't be parsed: probes the next patch/minor/major
   version with HEAD requests (e.g. `2.19.3`, `2.20.0`, `3.0.0`).
3. Compares with `<base>/.version` (`sort -V`); exits 0 if nothing new.
4. Downloads with the SAP EULA cookie into a temp file, tests the ZIP,
   extracts into `<base>/releases/<version>`, points `<base>/current` at it
   and keeps the last 3 releases (`-k`).

Uses `flock` so overlapping runs exit with code 3.

## Installation (SLES / openSUSE)

```bash
sudo zypper install curl unzip util-linux
sudo install -m 0755 sapcc-update.sh /usr/local/sbin/
sudo install -m 0644 sapcc-update.service sapcc-update.timer /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now sapcc-update.timer
```

Test first: `sudo /usr/local/sbin/sapcc-update.sh -n`

Logs: `journalctl -u sapcc-update.service`

Cron alternative:

```
15 3 * * * /usr/local/sbin/sapcc-update.sh -q
```

## Notes

- Downloading means you accept the SAP Developer License; the cookie is
  configurable via `SAPCC_EULA_COOKIE` should SAP bump the license version.
- The ZIP contains the installer RPM; the script does not install it.
