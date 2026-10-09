# rdkit-sys

CXX bindings to selected RDKit C++ functionality. This crate is a member of the [rdkit workspace](..) and underpins the safe [rdkit crate](https://crates.io/crates/rdkit).

Install the native library using the workspace [build instructions](../docs/building.md). The bridge uses C++20, exclusive owning pointers, borrowed references tied to their owner, and exception translation. See the [binding audit](../docs/binding-audit.md) and [migration guide](../docs/migration.md) before calling the low-level API directly.

The public low-level surface intentionally does not mirror every C++ overload. CXX checks bridge types; ownership, aliasing, nullability, and exception contracts still require manual review and regression testing.

```sh
cargo test -p rdkit-sys --locked
```

References: [RDKit C++ API](https://www.rdkit.org/docs/cppapi/index.html), [CXX](https://cxx.rs/), and [prior rdkit_cxx work](https://github.com/apahl/rdkit_cxx).
