# RDKit for Rust

The [rdkit crate](https://crates.io/crates/rdkit) provides Rust interfaces for molecule parsing, graph inspection and editing, fingerprints, descriptors, standardization, substructure matching, scaffold networks, and conformer coordinates. The workspace member [rdkit-sys](https://crates.io/crates/rdkit-sys) bridges to the installed RDKit C++ library using CXX; its source is in [rdkit-sys/](rdkit-sys).

## Install and build

The primary target is Ubuntu 26.04 LTS, x86_64, with RDKit 2025.03.6 from Ubuntu's `librdkit-dev=202503.6-4` package. Ubuntu 24.04 LTS with `202309.3-3build2` is the compatibility target. The native library and its matching development headers must both be installed.

```sh
sudo bash ci/install-deps.sh
cargo test --workspace --locked
```

The checkout pins Rust 1.99.0 and commits `Cargo.lock` for repeatable validation. See [building](docs/building.md) for exact package versions, containers, custom installations, Conda, and the target matrix.

Current-upstream RDKit 2026.09.1 also has a separate Ubuntu 26.04 reference-package lane on amd64 and arm64. Its [installer](ci/install-reference-deps.sh) verifies the GitHub release bundle before installing it; this does not require the project's APT repository. Use a fresh environment because its development headers conflict with Ubuntu's RDKit development package.

Use the crate named `rdkit`, with `use rdkit::ROMol`. The unrelated `rdkit-rs` crate is not this workspace.

```rust
use rdkit::ROMol;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mol = ROMol::from_smiles("OCC")?;
    println!("{}", mol.try_as_smiles()?);
    Ok(())
}
```

## Scope and compatibility

This is a selected binding surface, not a Rust reimplementation or a claim of complete RDKit API coverage. The [binding audit](docs/binding-audit.md) defines the inventory and ownership rules. The [migration guide](docs/migration.md) covers breaking low-level changes, checked methods, and the compatibility convenience methods. [Validation](docs/validation.md) records what was actually run and the remaining gaps.

Canonicalization, descriptors, and fingerprints follow the linked RDKit release. Persisted chemical indexes may need rebuilding when that release changes. ABI compatibility across arbitrary RDKit, Boost, or platform versions is not promised.

## Development

```sh
cargo fmt --all --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo doc --workspace --no-deps --locked
find rdkit-sys/wrapper -type f \( -name '*.h' -o -name '*.cc' \) -print0 | xargs -0 clang-format-18 --dry-run --Werror
```

Both crates remain at their existing versions during this unreleased modernization. Publishing requires a coordinated version decision for both crates because the low-level API changes are breaking. No release or tag is part of this work.
