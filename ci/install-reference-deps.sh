set -euo pipefail
. /etc/os-release
if [ "$ID:$VERSION_ID" != ubuntu:26.04 ]; then
    printf 'The reference package requires Ubuntu 26.04.\n' >&2
    exit 1
fi
rdkit_arch=$(dpkg --print-architecture)
case "$rdkit_arch" in
    amd64) rdkit_bundle_sha256=70899595de95f2e094c4398fe0e7ff41980d6d8fb3ede9936b643fadb3235783 ;;
    arm64) rdkit_bundle_sha256=3e4c8a47f3fd73ffc24ec380d24d8ccdc60081fb0bcbd8be1259de4c4eebd740 ;;
    *) printf 'Unsupported reference architecture: %s\n' "$rdkit_arch" >&2; exit 1 ;;
esac
rdkit_release_url=https://github.com/rdkit-rs/rdkit-debian/releases/download/ubuntu-26.04-rdkit-2026.09.1-1
rdkit_bundle="rdkit-2026.09.1-ubuntu26.04-$rdkit_arch.tar.xz"
rdkit_download_dir=$(mktemp -d)
trap 'rm -rf "$rdkit_download_dir"' EXIT
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential ca-certificates curl git pkg-config xz-utils
curl --fail --silent --show-error --location --retry 3 "$rdkit_release_url/SHA256SUMS" --output "$rdkit_download_dir/SHA256SUMS"
curl --fail --silent --show-error --location --retry 3 "$rdkit_release_url/$rdkit_bundle" --output "$rdkit_download_dir/$rdkit_bundle"
(
    cd "$rdkit_download_dir"
    printf '%s  %s\n' 1f3e0c00d63638a829a54e17dbb5561fb4cbc3381f2193098260692c241b9f57 SHA256SUMS "$rdkit_bundle_sha256" "$rdkit_bundle" | sha256sum --check
    sha256sum --check --ignore-missing SHA256SUMS
    mkdir bundle
    tar -xJf "$rdkit_bundle" -C bundle
    cd bundle
    sha256sum --check SHA256SUMS
)
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "$rdkit_download_dir"/bundle/*.deb
dpkg-query -W librdkit-rs-dev librdkit-rs202609 rdkit-rs-data libboost-dev libeigen3-dev
test "$(pkg-config --modversion rdkit)" = 2026.09.1
