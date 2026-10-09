set -euo pipefail
. /etc/os-release
case "$ID:$VERSION_ID" in
    ubuntu:26.04) rdkit_package_version=202503.6-4 ;;
    ubuntu:24.04) rdkit_package_version=202309.3-3build2 ;;
    debian:13) rdkit_package_version=202503.1-4 ;;
    *) printf 'Unsupported package target: %s %s\n' "$ID" "$VERSION_ID" >&2; exit 1 ;;
esac
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential ca-certificates curl git pkg-config \
    "librdkit-dev=$rdkit_package_version" libboost-dev libboost-serialization-dev libeigen3-dev
dpkg-query -W librdkit-dev libboost-dev libeigen3-dev
