# Building with packaged RDKit

## Version decision, verified 2026-10-09

[RDKit 2026.09.1](https://github.com/rdkit/rdkit/releases/tag/Release_2026_09_1) is the current upstream release, published 2026-10-03. Upstream's newest release and the newest distro package are different things.

[Ubuntu 26.04 LTS](https://documentation.ubuntu.com/release-notes/26.04/) was released 2026-04-23. It is the newest released stable Ubuntu on this verification date. [Ubuntu 26.10's schedule](https://documentation.ubuntu.com/release-notes/26.10/schedule/) places final release on 2026-10-15; its package archive is not our supported baseline.

| Platform | RDKit / development package | Role |
| --- | --- | --- |
| Ubuntu 26.04 LTS x86_64 | 2025.03.6 / `202503.6-4` | Primary CI and installation target |
| Ubuntu 24.04 LTS x86_64 | 2023.09.3 / `202309.3-3build2` | Older supported CI target |
| Debian 13 x86_64 | 2025.03.1 / `202503.1-4` | Saved environment validation |
| Ubuntu 22.04 LTS | 2021.09.2 / `202109.2-1build1` | Too old for this binding surface |
| Ubuntu 25.10 | 2025.03.1 / `202503.1-4` | Interim release; not a CI target |
| Ubuntu 26.10 prerelease | 2025.03.6 / `202503.6-4` | No advantage over released 26.04 for this work |
| Ubuntu 26.04 LTS amd64 and arm64 | Upstream 2026.09.1 / `2026.09.1+ds1-1~ubuntu26.04` | Additional reference-package CI targets |
| macOS, Windows | Varies | Not validated by this PR |

Sources: [Ubuntu package versions](https://packages.ubuntu.com/librdkit-dev), [26.04 package](https://packages.ubuntu.com/resolute/librdkit-dev), [Debian 13 package](https://packages.debian.org/trixie/librdkit-dev). Both Ubuntu package pins were also checked against their actual apt repositories in clean containers.

The LTS package supplies a matched native RDKit, Boost dependency set, and development headers through ordinary package management. The baseline can be installed without a custom binary distribution. Selecting it does not imply that it contains every fix or feature from newer upstream releases, or that Ubuntu Universe has the same support guarantees as Ubuntu Main.

The project's [Debian packaging tutorial](https://github.com/rdkit-rs/rdkit-rs.github.io/blob/1d3778e/content/tutorials/forking-a-debian-package.md) explains the earlier need for newer upstream packages, isolated native builds and CI-produced artifacts. The separate reference package below carries that goal forward for current upstream on Ubuntu 26.04. The distro installation path remains the default.

## Native installation

Run from this checkout on one of the three listed Linux releases:

```sh
sudo bash ci/install-deps.sh
cargo test --workspace --locked
```

The script installs the exact RDKit package above plus a C++ compiler, Boost development headers and serialization library, and Eigen headers. Rustup honors the pinned Rust 1.99.0 toolchain in `rust-toolchain.toml`. CI uses the committed Cargo lockfile. These are tested toolchain versions, not a claim that all older Rust compilers fail.

The Ubuntu CI jobs run on a Linux x86_64 host inside digest-pinned Ubuntu containers. Package versions are explicit; other apt dependencies receive the distribution's current updates. This reproduces the RDKit/API baseline, not a bit-for-bit frozen operating system. If a pinned package leaves the active archive, use the corresponding Ubuntu snapshot and record that change rather than silently taking a different RDKit release.

Install `clang-format-18` for C++ formatting. Standard Rust formatting works without nightly.

## Current-upstream reference package

The [Ubuntu 26.04 reference release](https://github.com/rdkit-rs/rdkit-debian/releases/tag/ubuntu-26.04-rdkit-2026.09.1-1) contains runtime, development and data packages at `2026.09.1+ds1-1~ubuntu26.04`. It was built from packaging commit `0656aade78d3709900552193cee0506c19f96223` and upstream commit `fece8caa860bdf6c9c82bdb9253f22a0cf87150c` (`Release_2026_09_1`). The `+ds1` source repack excludes the optional Amadeus font; the bundle includes source, packaging patches, provenance and license notices.

On a fresh Ubuntu 26.04 amd64 or arm64 environment:

```sh
sudo bash ci/install-reference-deps.sh
cargo test --workspace --locked
```

The installer downloads the architecture's complete release bundle, verifies the pinned hash, the published `SHA256SUMS`, and all internal checksums, then installs the local `.deb` files using `apt-get install`. This resolves their Ubuntu dependencies. It tests installation from GitHub release artifacts; it does not add or validate the project's S3 APT repository. The development package conflicts with Ubuntu's `librdkit-dev`, so use a separate environment when retaining the distro baseline.

| Artifact | SHA-256 |
| --- | --- |
| Published `SHA256SUMS` | `1f3e0c00d63638a829a54e17dbb5561fb4cbc3381f2193098260692c241b9f57` |
| `rdkit-2026.09.1-ubuntu26.04-amd64.tar.xz` | `70899595de95f2e094c4398fe0e7ff41980d6d8fb3ede9936b643fadb3235783` |
| `rdkit-2026.09.1-ubuntu26.04-arm64.tar.xz` | `3e4c8a47f3fd73ffc24ec380d24d8ccdc60081fb0bcbd8be1259de4c4eebd740` |

The bundles preserve the original Debian filenames containing `~`; GitHub's individual `.deb` asset names use `.` in its place. CI uses the bundles directly. Headers and linker symlinks install under `/usr`; the runtime SONAME is `.so.2026.09`. `pkg-config --modversion rdkit` reports `2026.09.1`. No prefix or loader override is needed. This project's tests use built-in chemistry defaults and repository fixtures; consumers needing the packaged data can set `RDBASE` using `pkg-config --variable=rdbase rdkit`.

Reference-package jobs use native amd64 and arm64 runners, with the same pinned Ubuntu 26.04 container as the distro job. C++ ASan/UBSan jobs cover both the distro and reference packages on amd64. The reference packages' own [native build/install checks](https://github.com/rdkit-rs/rdkit-debian/actions/runs/37990535727) are separate from this workspace's binding tests. Neither project claims byte-for-byte package reproducibility; Ubuntu dependency versions are not snapshot-pinned.

The packaging project's later [APT publication verification](https://github.com/rdkit-rs/rdkit-debian/actions/runs/37992678461) passed clean installs and native smoke tests on both architectures. That is separate evidence for its APT repository, not the installation method used by this workspace's release-artifact CI.

## Prefixes and runtime linking

By default, Linux uses `/usr`, Apple Silicon macOS uses `/opt/homebrew`, and Intel macOS uses `/usr/local`. These macOS paths are supported discovery conventions, not macOS validation evidence. Override them explicitly:

```sh
RDKIT_PREFIX=/opt/rdkit cargo build --locked
```

`RDKIT_PREFIX` expects `include/rdkit/GraphMol/ROMol.h`, sibling dependency headers under `include`, optional `include/eigen3`, and native libraries under `lib`. `RDKIT_INCLUDE_DIR` overrides the directory containing `GraphMol`; `RDKIT_LIB_DIR` overrides the RDKit library directory. All inputs are tracked by Cargo so changes cause a rebuild. Cross compilation requires matching target libraries, a target compiler, and explicit paths; it is not in the tested matrix.

```sh
RDKIT_PREFIX=/opt/rdkit \
RDKIT_INCLUDE_DIR=/opt/rdkit/include/rdkit \
RDKIT_LIB_DIR=/opt/rdkit/lib64 \
cargo build --locked
```

Link-time discovery does not configure the runtime loader. Distro packages already install libraries in configured loader paths. With a private installation, set the loader path before running the executable:

```sh
export LD_LIBRARY_PATH=/opt/rdkit/lib64${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
cargo test --workspace --locked
```

Use the installation's matched RDKit and Boost libraries. Renaming a Boost SONAME or creating a symlink between incompatible Boost releases does not make their ABIs compatible.

## Conda and other platforms

An activated native Conda installation can use the existing feature:

```sh
cargo test --workspace --features dynamic-linking-from-conda --locked
```

The high-level crate now forwards this feature to `rdkit-sys`. `RDKIT_PREFIX` takes precedence; otherwise the feature requires `CONDA_PREFIX`. Install native C++ development headers as well as shared libraries. A Python wheel alone is not a C++ development installation. A Conda package solve and native Windows build have not been tested here; the feature is not a promise of Windows support.

The build reads Cargo's target OS and architecture instead of the build host's values. Unsupported targets fail explicitly. A full upstream source build remains an optional integration task; it is not required for the supported packaged targets.
