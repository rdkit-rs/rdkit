RDKit
---

A high-level library for performing common RDKit tasks such as SMILE parsing, molecule normalization, etc. Uses
the C++ API via bindings from [rdkit-sys](https://crates.io/crate/rdkit-sys).

Prerequisites
---

On a Mac:

    brew install rdkit

On Linux you will need a custom build of rdkit with static libraries. We are working to provide suitable debian packages.
This is holding up our CI.