0.4.0
---

 * Update all relevant APIs to perform borrows, require much less clone() of `std::shared_ptr` objects
 * Scaffold API
 * MolOps API

0.3.0
---

 * Remove all static references from build.rs, the debian packages just won't support it

0.2.13
---

 * use mol parsing converts c++ exceptions to `Result`
 * rewrite RDKit C++ imports to use angle brackets for more natural look
 * rename smile parsing function

0.2.12
---

 * conda support in build.rs and finally builds in CI!

0.2.10
---

 * update bridge to catch atomvalenceexception and return nullptr

0.2.9
---

 * convert fingerprint to vec, switching from boost dynamic_bitvect to rust bitvec for ergonomics

0.2.8
---

 * RWMol molblock support

0.2.7
---

 * build.rs refactoring: introspect bridge files to build list of .cc/.h files

0.2.6
---

 * Substruct match

0.2.5
---

 * ROMol copy

0.2.4
---

 * Now using `examples/` for low level wrapper examples, making it easier to run OS X `leaks` on the wrappers
 * `RDKit::MolStandardize::TautomerEnumerator->canoncalize()` has been ported over, see the tautomer example
 * `RDKit::MolStandardize::TautomerEnumerator->enumerate()` has been ported over with `new`/`size`/`at` flavors on the Rust side