RDKit
---

A high-level library for performing common RDKit tasks such as SMILE parsing, molecule normalization, etc. Uses
the low-level C++ API with bindings from [rdkit-sys](https://crates.io/crate/rdkit-sys).

Provides `Molecule` and `Fingerprint` along with some benchmarks proving speed. Concepts in this `RDKit` may not
correspond one-to-one with RDKit C++ concepts. For a more accurate correspondence, consider using the `rdkit-sys` bindings
directly.

Prerequisites
---

On a Mac:

    brew install rdkit