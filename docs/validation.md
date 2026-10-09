# Validation record

Recorded 2026-10-09 for this modernization. The PR's GitHub checks identify the tested final commit; this document records the local validation and its limits. No release, tag or merge is included.

## Executed local checks

| Environment / check | Result |
| --- | --- |
| Ubuntu 26.04 amd64, RDKit `202503.6-4`, GCC 15 | 107 workspace tests passed, including the compile-fail harness |
| Ubuntu 24.04 amd64, RDKit `202309.3-3build2`, GCC 13 | 107 workspace tests passed, including the compile-fail harness |
| Saved Debian 13 amd64, RDKit `202503.1-4`, GCC 14 | 107 tests passed with `--workspace --all-targets`; examples compiled and stable benchmark smoke ran |
| Borrow checking | All six expected compile failures matched their checked-in diagnostics |
| Focused regressions | 17 safety/correctness tests passed on both Ubuntu versions and the saved environment |
| Ubuntu 26.04 C++ ASan + UBSan | 78 targeted high-level and sys tests passed; leak detection enabled; no reported findings |
| Actual downstream application in an isolated checkout | All 50 existing tests passed with both crates patched to this branch |
| Rust formatting | `cargo fmt --all --check` passed on pinned Rust 1.99.0 |
| C++ formatting | clang-format 18 dry run with warnings as errors passed |
| Clippy | All-targets workspace checking with warnings denied passed in the saved environment and both Ubuntu versions |
| Rustdoc | Workspace documentation with warnings denied passed on both Ubuntu versions |
| Conda feature forwarding | All-features workspace check passed with the distro prefix on Ubuntu 26.04; this is not a Conda package solve |
| Whitespace | `git diff --check` passed |

The 107 count includes one Rust harness test that runs six compile-fail fixtures; it does not count those fixtures as six additional runtime tests. Tests are behavioral samples, not exhaustive input or instruction coverage. Empty test/example/doc-test harnesses are not added to the count. The native declaration inventory is separate from the test count.

The isolated downstream validation exercised existing parsing, processing, descriptor/fingerprint, indexing/search and API workflows. Only dependency path overrides, necessary lockfile resolution and local build-output discovery were changed in the isolated checkout. No downstream application source or tests were changed or pushed. A public error-display regression found by that suite was fixed in the bindings while preserving native detail in the error payload. Private downstream implementation/data are intentionally not reproduced here.

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

## Failures investigated and corrected

The first Ubuntu 24.04 build exposed header organization and periodic-table methods that differ in RDKit 2023.09. The bridge now includes the common parser header and uses small checked compatibility shims for row/max-atomic-number lookup. Every atomic number from 0 through 118 is tested for row lookup.

Two existing tests assumed newer native chemistry data or behavior: a tautomer canonicalization case and carbon's covalent radius. The tests now assert the specific observed results of the supported native versions. The canonicalization success case also requires no chemistry problems and a successful sanitized reparse; failures must match the expected native exception. The binding does not substitute one release's chemistry behavior for another's.

Initial development runs also exposed the old nightly-only benchmark, missing compile-fail snapshots, and Clippy failures. The benchmark now uses stable timing/black-box operations, the six diagnostics are checked in, and the lint failures were corrected. Final results above supersede those development failures.

## Not run or not established

Current-upstream RDKit 2026.09.1 validation awaits the separate Ubuntu 26.04 reference-package artifact and its checksum/install contract. It must be added as its own compatibility lane without removing the distro matrix. macOS, Windows, Linux arm64, a real Conda environment, Miri, and sanitizer-instrumented RDKit/Rust builds were not run. Miri alone cannot validate these native C++ algorithms. No fuzzing campaign, whole-upstream API coverage or general memory-safety proof is claimed.
