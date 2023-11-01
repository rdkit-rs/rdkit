RDKit-Sys
---

Rust code that binds to the C++ rdkit library!

How does it work?
---

RDKit is a C++ mega-library, full of cheminformatics wisdom. We don't want to rewrite RDKit in Rust, we should instead meet somewhere in the middle and
"bridge" Rust to C++ through some wrappers.

The goal is to do 1-1 bindings with the C++ library, exposing all the classes as we need them. The goal is _not_ to create
a high-level functionality like the MinimalLib (cffiwrapper). Our goal is to expose the building blocks. If you're looking
for idiomatic Rust, check out the [rdkit](https://crates.io/crate/rdkit) crate.

Prerequisites
---

On Mac:

    brew install rdkit

Also known to work with conda-managed RDKit, be sure to set the `dynamic-linking-from-conda` feature. Not as tested, please open an issue if you have a hard time.

Testing
---

Or just run the test suite:

    cargo test

TODO
---

 - [ ] figure out how to `cargo publish` without `--no-verify` (otherwise it detects changes outside of OUTDIR)
 - [X] specify path to RDKit's cffiwrapper.h and all required search paths for other dependent headers
 - [X] use conditional rebuild logic to make the library build experience more reliable (for now, if you get stuck, try `cargo clean` and retry with `cargo build -vv`)

Related Documentation
---

 - https://www.rdkit.org/docs/cppapi/index.html
 - https://cxx.rs/

Prior art
---

 - https://github.com/apahl/rdkit_cxx
 - [rdkafka's excellent librdkafka build.rs](https://github.com/fede1024/rust-rdkafka/blob/master/rdkafka-sys/build.rs)
 - https://iwatobipen.wordpress.com/2022/01/29/use-rdkit-from-rust-rdkit-rdkitcffi-rust/
 - [an attempt at using rdkit in rust but without docs on how to build rdkit](https://github.com/iwatobipen/rust_rdkit/)
