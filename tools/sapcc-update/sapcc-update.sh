#!/usr/bin/env bash
#
# sapcc-update.sh - download and extract the latest SAP Cloud Connector
#                   (Linux x64 ZIP) from tools.hana.ondemand.com, but only
#                   if a newer version than the last downloaded one exists.
#
# Version discovery:
#   1. Parse the download page (https://tools.hana.ondemand.com/) for
#      "additional/sapcc-<version>-linux-x64.zip" links.
#   2. If that yields nothing, probe the next patch/minor/major version
#      with HEAD requests, starting from the last known version.
#
# Layout below $SAPCC_BASE_DIR (default /opt/sapcc-download):
#   downloads/sapcc-<ver>-linux-x64.zip   downloaded archives
#   releases/<ver>/                       extracted content
#   current -> releases/<ver>             symlink to the newest release
#   .version                              last successfully processed version
#
# Exit codes: 0 = success (updated or already current), 1 = error,
#             2 = usage error, 3 = another instance is running.

set -Eeuo pipefail
umask 022

readonly PROG=${0##*/}
readonly BASE_URL=${SAPCC_BASE_URL:-https://tools.hana.ondemand.com}
readonly INDEX_URL=${SAPCC_INDEX_URL:-$BASE_URL/}
# Download links require the accepted SAP Developer License (EULA) cookie.
readonly EULA_COOKIE=${SAPCC_EULA_COOKIE:-eula_3_2_agreed=tools.hana.ondemand.com/developer-license-3_2.txt}
readonly START_VERSION=${SAPCC_START_VERSION:-2.19.2}

BASE_DIR=${SAPCC_BASE_DIR:-/opt/sapcc-download}
KEEP=${SAPCC_KEEP:-3}
DRY_RUN=0
FORCE=0
QUIET=0

usage() {
    cat <<EOF
Usage: $PROG [-d DIR] [-k N] [-n] [-f] [-q] [-h]

Download and extract the newest SAP Cloud Connector (sapcc-*-linux-x64.zip)
if it is newer than the last downloaded version.

Options:
  -d DIR   base directory (default: $BASE_DIR, env SAPCC_BASE_DIR)
  -k N     number of releases to keep, 0 = keep all (default: $KEEP)
  -n       dry run: only report what would be done
  -f       force download/extract even if version is already current
  -q       quiet: log only warnings and errors (useful for cron)
  -h       show this help

Environment:
  SAPCC_EULA_COOKIE    EULA cookie sent with the download request
  SAPCC_START_VERSION  version to probe from if none is known (default: $START_VERSION)
EOF
}

log()  { (( QUIET )) || printf '%s %s[%d]: %s\n' "$(date '+%F %T')" "$PROG" "$$" "$*"; }
warn() { printf '%s %s[%d]: WARNING: %s\n' "$(date '+%F %T')" "$PROG" "$$" "$*" >&2; }
die()  { printf '%s %s[%d]: ERROR: %s\n' "$(date '+%F %T')" "$PROG" "$$" "$*" >&2; exit 1; }

# version_gt A B -> true if A > B (semantic version compare via sort -V)
version_gt() {
    [[ $1 != "$2" ]] && [[ $(printf '%s\n%s\n' "$1" "$2" | sort -V | tail -n1) == "$1" ]]
}

zip_url() { printf '%s/additional/sapcc-%s-linux-x64.zip' "$BASE_URL" "$1"; }

CURL_OPTS=(--fail --silent --show-error --location
           --connect-timeout 20 --max-time 1800 --retry 3 --retry-delay 10
           --cookie "$EULA_COOKIE")

# url_exists URL -> true if the server answers HEAD with 2xx
url_exists() {
    curl "${CURL_OPTS[@]}" --head --max-time 30 --output /dev/null "$1" 2>/dev/null
}

# Highest version found on the download page (empty if none).
latest_from_index() {
    curl "${CURL_OPTS[@]}" --max-time 60 "$INDEX_URL" 2>/dev/null \
        | grep -oE 'sapcc-[0-9]+(\.[0-9]+)+-linux-x64\.zip' \
        | sed -E 's/^sapcc-(.*)-linux-x64\.zip$/\1/' \
        | sort -uV | tail -n1 || true
}

# Probe next patch / minor / major versions starting at $1.
latest_by_probing() {
    local cur=$1 found=1 major minor patch cand
    url_exists "$(zip_url "$cur")" || cur=""
    while (( found )); do
        found=0
        IFS=. read -r major minor patch <<<"${cur:-$START_VERSION}"
        for cand in "$major.$minor.$((patch + 1))" \
                    "$major.$((minor + 1)).0" \
                    "$((major + 1)).0.0"; do
            if url_exists "$(zip_url "$cand")"; then
                cur=$cand found=1
                break
            fi
        done
    done
    printf '%s' "$cur"
}

prune_old() {
    (( KEEP > 0 )) || return 0
    local -a versions
    mapfile -t versions < <(find "$BASE_DIR/releases" -mindepth 1 -maxdepth 1 \
                                -type d -printf '%f\n' | sort -V)
    local n=$(( ${#versions[@]} - KEEP )) i
    for (( i = 0; i < n; i++ )); do
        log "pruning old release ${versions[i]}"
        rm -rf -- "$BASE_DIR/releases/${versions[i]}" \
                  "$BASE_DIR/downloads/sapcc-${versions[i]}-linux-x64.zip"
    done
}

main() {
    local opt
    while getopts ':d:k:nfqh' opt; do
        case $opt in
            d) BASE_DIR=$OPTARG ;;
            k) KEEP=$OPTARG ;;
            n) DRY_RUN=1 ;;
            f) FORCE=1 ;;
            q) QUIET=1 ;;
            h) usage; exit 0 ;;
            :) printf '%s: option -%s requires an argument\n' "$PROG" "$OPTARG" >&2; usage >&2; exit 2 ;;
            *) printf '%s: invalid option -%s\n' "$PROG" "$OPTARG" >&2; usage >&2; exit 2 ;;
        esac
    done
    shift $((OPTIND - 1))
    (( $# == 0 )) || { usage >&2; exit 2; }
    [[ $KEEP =~ ^[0-9]+$ ]] || { printf '%s: -k needs a number\n' "$PROG" >&2; exit 2; }

    local cmd
    for cmd in curl unzip flock sort; do
        command -v "$cmd" >/dev/null || die "required command not found: $cmd"
    done

    mkdir -p -- "$BASE_DIR/downloads" "$BASE_DIR/releases"

    # Only one instance at a time.
    exec 9>"$BASE_DIR/.lock"
    flock -n 9 || { warn "another instance is running, exiting"; exit 3; }

    local installed=""
    [[ -r $BASE_DIR/.version ]] && installed=$(<"$BASE_DIR/.version")

    local latest
    latest=$(latest_from_index)
    if [[ -n $latest ]]; then
        log "latest version on download page: $latest"
    else
        warn "could not parse download page, probing versions instead"
        latest=$(latest_by_probing "${installed:-$START_VERSION}")
        [[ -n $latest ]] || die "no SAP Cloud Connector version found"
        log "latest version found by probing: $latest"
    fi

    if [[ -n $installed ]] && ! version_gt "$latest" "$installed" && (( ! FORCE )); then
        log "already up to date (version $installed)"
        return 0
    fi
    log "update available: ${installed:-none} -> $latest"

    local url zip release
    url=$(zip_url "$latest")
    zip=$BASE_DIR/downloads/sapcc-$latest-linux-x64.zip
    release=$BASE_DIR/releases/$latest

    if (( DRY_RUN )); then
        log "dry run: would download $url and extract to $release"
        return 0
    fi

    # Download to a temp file, verify, then move into place atomically.
    local tmp
    tmp=$(mktemp "$BASE_DIR/downloads/.sapcc-XXXXXX.part")
    trap 'rm -rf -- "${tmp:-}" "${tmpdir:-}"' EXIT
    log "downloading $url"
    curl "${CURL_OPTS[@]}" --output "$tmp" "$url" || die "download failed: $url"
    unzip -tqq "$tmp" >/dev/null || die "downloaded file is not a valid ZIP archive"
    chmod 0644 "$tmp"
    mv -f -- "$tmp" "$zip"

    # Extract into a temp dir next to the target, then rename.
    tmpdir=$(mktemp -d "$BASE_DIR/releases/.extract-XXXXXX")
    log "extracting to $release"
    unzip -qq -o "$zip" -d "$tmpdir" || die "extraction failed"
    chmod 0755 "$tmpdir"
    rm -rf -- "$release"
    mv -- "$tmpdir" "$release"

    ln -sfn -- "releases/$latest" "$BASE_DIR/current"
    printf '%s\n' "$latest" > "$BASE_DIR/.version.tmp"
    mv -f -- "$BASE_DIR/.version.tmp" "$BASE_DIR/.version"

    prune_old
    log "done: SAP Cloud Connector $latest available in $BASE_DIR/current"
}

main "$@"
