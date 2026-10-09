# Validation record

Recorded 2026-10-09 for this modernization. The PR's GitHub checks identify the tested final commit; this document records the local validation and its limits. No release, tag or merge is included.

## Executed local checks

The original package matrix passed at `18ed53359fbbd21df813b23868d9f5a02261cc4c`. The current-upstream follow-up adds three regression tests; the rows below distinguish the original full runs from the later compatibility checks. The PR's final-head CI runs the complete suite on every configured platform.

| Environment / check | Result |
| --- | --- |
| Ubuntu 26.04 amd64, RDKit `202503.6-4`, GCC 15 | Original 107 workspace tests passed; later 20 safety and three descriptor tests passed with the 2026 compatibility changes |
| Ubuntu 24.04 amd64, RDKit `202309.3-3build2`, GCC 13 | Original 107 workspace tests passed; later 20 safety and three descriptor tests passed with the 2026 compatibility changes |
| Saved Debian 13 amd64, RDKit `202503.1-4`, GCC 14 | 107 tests passed with `--workspace --all-targets`; examples compiled and stable benchmark smoke ran |
| Ubuntu 26.04 amd64, reference RDKit `2026.09.1+ds1-1~ubuntu26.04`, GCC 15 | 110 tests passed with `--workspace --all-targets`; examples and benchmark targets also passed |
| Borrow checking | All six expected compile failures matched their checked-in diagnostics |
| Focused regressions | 20 safety/correctness tests passed on RDKit 2023.09, 2025.03 and 2026.09; the original 17 also passed in the saved environment |
| Ubuntu 26.04 C++ ASan + UBSan | Original 78 tests passed with distro RDKit; 81 passed with reference RDKit 2026.09.1; leak detection enabled and no reported findings |
| Actual downstream application in isolated checkouts | All 50 existing tests passed in the saved environment and again with reference RDKit 2026.09.1 on Ubuntu 26.04 |
| Rust formatting | `cargo fmt --all --check` passed on pinned Rust 1.99.0 |
| C++ formatting | clang-format 18 dry run with warnings as errors passed |
| Clippy | All-targets workspace checking with warnings denied passed in the saved environment, both Ubuntu distro packages, and reference RDKit 2026.09.1 |
| Rustdoc | Workspace documentation with warnings denied passed on both Ubuntu versions and reference RDKit 2026.09.1 |
| Conda feature forwarding | All-features workspace check passed with distro and reference prefixes on Ubuntu 26.04; this is not a Conda package solve |
| Whitespace | `git diff --check` passed |

The 107 and 110 counts each include one Rust harness test that runs six compile-fail fixtures; they do not count those fixtures as six additional runtime tests. Tests are behavioral samples, not exhaustive input or instruction coverage. Empty test/example/doc-test harnesses are not added to the count. The native declaration inventory is separate from the test count.

The isolated downstream validation exercised existing parsing, processing, descriptor/fingerprint, indexing/search and API workflows. Only dependency path overrides, necessary lockfile resolution and local build-output discovery were changed in the isolated checkout. No downstream application source or tests were changed or pushed. A public error-display regression found by that suite was fixed in the bindings while preserving native detail in the error payload. Private downstream implementation/data are intentionally not reproduced here.

The Ubuntu 26.04 downstream run reused a separately installed native dependency from the saved environment after that dependency's fresh binary download was blocked by the executor. It was isolated from the RDKit library path. All chemistry libraries came from the verified 2026.09.1 reference packages; no downstream tests were changed or skipped.

## Reproduce

Install the pinned native package as described in [building](building.md), then run:

```sh
cargo fmt --all --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked
CONDA_PREFIX=/usr cargo check --workspace --all-features --locked
bash ci/test-sanitizers.sh
```

The last command is the Linux x86_64 GCC sanitizer lane. It instruments the C++ bridge and code instantiated from native headers, links ASan/UBSan, and preloads ASan only for test execution through Cargo's target runner. It tests the safety, molecule, scaffold, matching, hydrogen and parser paths plus all sys tests. Rust and the packaged RDKit shared libraries are not themselves rebuilt with sanitizers. This run provides useful executed-path evidence, not a whole-library memory-safety proof.

The all-features command checks feature forwarding/prefix discovery against the installed distro package. It does not validate a real Conda solve. Use a separate target directory for sanitizer builds when overriding `CARGO_TARGET_DIR`.

## Reference-package provenance

The amd64 release bundle, published checksum file and every internal file matched their SHA-256 values. [Building](building.md#current-upstream-reference-package) records the release, source commits, package version and pinned amd64/arm64 bundle hashes. The runtime, development and data packages were installed together in a fresh Ubuntu 26.04 container; `pkg-config` reported `2026.09.1`, and runtime inspection confirmed the test executable loaded `.so.2026.09` libraries.

This is GitHub release-artifact installation using `apt-get install ./package.deb` for dependency resolution. The packaging project's [separate APT verification run](https://github.com/rdkit-rs/rdkit-debian/actions/runs/37992678461) passed clean repository installs and native smoke tests on both architectures. Its results do not substitute for the full binding suite. This workspace's reference CI repeats the verified bundle installation on native amd64 and arm64, alongside the two distro-package jobs.

## Failures investigated and corrected

The first Ubuntu 24.04 build exposed header organization and periodic-table methods that differ in RDKit 2023.09. The bridge now includes the common parser header and uses small checked compatibility shims for row/max-atomic-number lookup. Every atomic number from 0 through 118 is tested for row lookup.

Two existing tests assumed newer native chemistry data or behavior: a tautomer canonicalization case and carbon's covalent radius. The tests now assert the specific observed results of the supported native versions. The canonicalization success case also requires no chemistry problems and a successful sanitized reparse; failures must match the expected native exception. The binding does not substitute one release's chemistry behavior for another's.

The initial build of `18ed533` against RDKit 2026.09.1 failed CXX size assertions: native bond and atom hybridization enums changed from wider storage to one byte. The bridge now maps named native enumerators to stable bridge-owned values, with errors for unsupported values. RDKit 2026 also deletes the periodic-table copy constructor; owned stateless handles now query the singleton per call and never retain or destroy it. Regressions cover all 22 exposed bond types, all nine hybridizations, invalid enum inputs, parsed E/Z and directional bonds, and independent table-handle destruction.

RDKit 2026 corrects the registered `chi2n`/`chi2v` descriptors, which returned third-order values for both fixtures in the older tested releases. Native probes compared the registry with direct second- and third-order calls. Both descriptor fixtures now assert the complete output against the specific old or corrected expected vector; no descriptor fields are skipped. The wrappers preserve the native values.

Initial development runs also exposed the old nightly-only benchmark, missing compile-fail snapshots, and Clippy failures. The benchmark now uses stable timing/black-box operations, the six diagnostics are checked in, and the lint failures were corrected. Final results above supersede those development failures.

The first expanded CI run failed before compilation because Docker Hub returned rate-limit errors and authentication timeouts. CI now pulls the identical pinned Ubuntu images through Docker's public ECR mirror.

## Not run or not established

macOS, Windows, a real Conda environment, Miri, and sanitizer-instrumented RDKit/Rust builds were not run. Arm64 coverage is limited to the reference-package CI job; local checks and sanitizer jobs run on amd64. Miri alone cannot validate these native C++ algorithms. No fuzzing campaign, whole-upstream API coverage or general memory-safety proof is claimed.
