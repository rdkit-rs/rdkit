//! rdkit-sys is a direct, one-to-one Rust binding against the RDKit C++ API
//!
//! Rust cannot represent C++ concepts like classes, subclasses, and structs,
//! nor can Rust perform moves or copies. Hence, all data from the RDKit C++ API
//! must be moved behind a "smart pointer" which will take care of freeing
//! memory after being dropped, you will see this is pervasive as
//! a `SharedPtr<T>` on the Rust side or `std::shared_ptr<T>` on the C++ side.
//!
//! It is highly recommend you read through the [RDKit C++ API documentation](https://www.rdkit.org/docs/cppapi/index.html) to learn more
//! about what exactly is possible with RDKit.
//!
//! If you just want high level access to SMILES parsing and various clean up
//! operations, please refer to the high level accompanying crate [rdkit](https://www.crates.io/crate/rdkit).

mod bridge;
pub use bridge::*;
