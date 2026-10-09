# Binding audit and supported surface

The baseline at `79a26ead54b2dbec2bedfc82976136b6c19017a6` had **145 callable CXX declarations in nine modules**. Every declaration, corresponding C++ shim, and high-level wrapper was reviewed for ownership, borrowing, mutation, nullability, exceptions, allocation pairing, bounds and thread traits. The branch currently has **189 callable declarations in eleven modules**, including bonds and conformers. The [complete name inventory](binding-inventory.md) defines that count; the three original Rust `PeriodicTableOps` helpers are reviewed but are not native declarations.

These counts describe this repository's binding surface. They are not a percentage of all RDKit APIs or a proof of memory safety. There is no claim of complete coverage of every RDKit overload, class, Python wrapper, platform or algorithm. The selected scope is a useful molecule workflow: parse, inspect, edit, sanitize, standardize, fingerprint, describe, search, generate 2D coordinates and construct scaffold networks.

## Ownership and aliasing

Native objects exposed as owners now use `UniquePtr`. Inputs borrow the native object as `&T` or `Pin<&mut T>`, instead of borrowing a nullable shared owner. A clone of a shared pointer previously allowed two safe callers to obtain mutable access to the same molecule or parameter object. The new signatures require an exclusive owner borrow for mutation, including atom setters and parameter setters.

Native atom, bond and conformer views borrow their molecule. Graph edits, coordinate regeneration, owner moves and destruction are blocked while those views are used. `AtomRef`, `Bond` and `Conformer` are read views; `Atom` and `ConformerMut` hold exclusive borrows. Atom and conformer access validates indices, and graph edits also reject invalid atomic numbers, invalid bond enums, self-bonds, duplicate bonds and nonexistent bonds. Integer inputs are checked before conversion to narrower native fields or enums.

`ROMol::clone`, `RWMol::clone`, ROMol-to-RWMol conversion and tautomer extraction use native copies with independently owned atom/bond/conformer data. RWMol-to-ROMol conversion transfers the existing owner through C++'s actual base-class conversion; the former Rust `transmute` is gone. No safe Rust function exposes a raw owning pointer or accepts a raw pointer argument. The scaffold shim's internal shared-pointer input vector owns copies and does not expose shared native mutation to Rust.

## Nulls, exceptions and allocations

The two SMILES factories intentionally preserve RDKit's nullable `UniquePtr` result when parsing fails without throwing. Their high-level callers check it before constructing a molecule and preserve the existing `UnknownConversionError` behavior. This is the explicit exception to the otherwise checked molecule-factory contract. SMARTS and mol-block factories report null results as errors; empty SMILES and SMARTS remain valid empty molecules.

All native callable declarations except the two infallible RWMol reference upcasts translate C++ exceptions through `Result`. Allocation, parsing, chemistry operations, property lookup, indexing and parameter construction are included. High-level fallible methods preserve those errors; existing convenience methods generally panic in Rust on a translated failure. This does not catch native undefined behavior or promise recovery from process-level allocation failure.

Raw RDKit factory returns are immediately adopted by their matching C++ smart pointer. Chemistry-problem and substructure-result containers previously remained raw while potentially throwing reserve/push operations ran; they now acquire ownership immediately. Native destruction stays in C++ with the correct type. Sanitize-exception access checks nulls and uses a checked dynamic cast before requesting an atom-specific index.

Periodic-table ownership is separate from the RDKit singleton: owned handles are copies. Valence lists are copied into owned vectors instead of exposing a static reference tied to native singleton destruction order. Two methods missing from RDKit 2023.09 use small compatibility shims: they call the native methods when present, otherwise use the 118-element table and the same row boundaries. Regression tests check every supported atomic number and an invalid one.

## Threads

Only the high-level, exclusively owned `ROMol` and `RWMol` opt into `Send`. They do not implement `Sync`, because apparently const RDKit chemistry operations can mutate caches. Native opaque types, borrowed views and auxiliary native owners do not gain unsafe thread traits. Compile-time assertions check these choices, and a runtime regression moves an owned molecule between threads and verifies independent copy mutation.

The argument for `Send` depends on exclusive native ownership, native deep-copy behavior and the packaged RDKit build. It is not a general thread-safety guarantee for arbitrary RDKit builds, global extension state or caller-written unsafe code.

## Review and regression map

Each row covers every declaration in that module, including repetitive parameter getters/setters. Test files identify the exercised operations; their presence does not imply exhaustive branch or input coverage. Executed results belong in the [validation record](validation.md).

| Area | Review findings and resulting contract | Regression evidence |
| --- | --- | --- |
| `ro_mol` | Unique molecule/parser owners; exclusive atom mutation; shared atom reads; nullable SMILES result checked; all throwing calls translated; checked subtype access; deep copies | High-level atom/graph tests; sys atom/ROMol tests; safety and borrowing tests |
| `rw_mol` | Native upcast replaces transmute; invalid SMARTS rejected; valid empty queries retained; checked graph edits and sanitization | Sys RWMol tests; graph/safety tests; borrow-failure fixtures |
| `bond` | Parent-borrowed read view; checked lookup/endpoints; no mutable alias; unknown returned enum representations remain representable | Graph/safety tests; bond edit borrow-failure fixture |
| `conformer` | Shared/exclusive parent borrows; checked atom positions and finite coordinate writes; 2D generation takes exclusive access | Safety tests; conformer regeneration borrow-failure fixture |
| `descriptors` | Unique property collection; borrowed molecule; owned result vectors; native exceptions translated | High-level and sys descriptor tests; downstream descriptor workflows |
| `fingerprint` | Unique native bit vectors; portable public-bit extraction; exact bit length; checked sizes; similarity edge cases | High-level and sys fingerprint tests; safety tests; downstream search workflows |
| `mol_ops` | Unique parameter objects; exclusive setters; copied hydrogen-operation results; exclusive cleanup/hybridization | High-level and sys mol-ops tests; downstream processing workflows |
| `mol_standardize` | Unique enumerator/result/cleanup/uncharger; native non-const uncharging requires mutation; owned yielded molecules; checked indices and decreasing iterator length | High-level graph and sys standardization tests; safety tests; downstream processing workflows |
| `periodic_table` | Copied table and valence-list ownership; translated invalid lookups; older-version row/max shims | High-level and sys periodic tests; exhaustive row and invalid-input regressions |
| `scaffold_network` | Unique parameters/network; copied input molecules; owned Rust node/edge/count data; native exceptions translated | High-level and sys scaffold tests, including default versus empty rules and edge direction |
| `substruct_match` | Unique parameters; exclusive setters; owned nested match vectors; immediate ownership of allocations | High-level and sys matching tests; downstream search workflows |
| SDF readers and Rust iterators | Complete delimiter lines; preserve title/dollar characters; propagate stream/UTF-8 failures; fused iteration; checked remaining length | Existing parser tests; added malformed-stream, record-content and iterator regressions |

Six compile-fail fixtures reject an atom outliving its molecule, graph mutation while a bond is borrowed, coordinate regeneration while a conformer is borrowed, two simultaneous low-level mutable atom views, cloning a unique native owner, and mutation while an `AtomRef` remains in use. Positive cases also run in the safety tests.

The ASan/UBSan lane instruments the C++ bridge and inlined native header code. The distribution's RDKit shared libraries and Rust code are not sanitizer-instrumented. It can expose real defects in executed paths, but it cannot establish whole-library memory safety.

## Compatibility and remaining scope

Compatibility means the crate builds and its selected behavioral tests pass against the exact native/platform matrix in [building](building.md), and the tested downstream workflows retain their documented behavior. It does not mean binary ABI interchangeability, identical chemistry results across every RDKit version, or source compatibility for every previous low-level caller. The [migration guide](migration.md) lists deliberate signature changes.

The `Properties` collection exposes the descriptors registered by the selected native build; it does not claim all Python descriptors. Remaining gaps include dedicated ring-information APIs, additional named descriptors, 3D embedding and ensemble generation, ETKDG options, MMFF/UFF force fields, standalone Murcko helpers, reactions, drawing, databases and many advanced stereochemistry/query controls. Native Windows, macOS and arm64 are not in this PR's validated matrix. The current-upstream reference-package lane depends on a separately produced artifact.

## Contributor work

This implementation preserves and adapts useful work from [#19](https://github.com/rdkit-rs/rdkit/pull/19) (Alexandra Ebberg: invalid SMARTS), [#45](https://github.com/rdkit-rs/rdkit/pull/45) (Guillaume CHARIFI: explicit installation prefix), [#50](https://github.com/rdkit-rs/rdkit/pull/50), [#51](https://github.com/rdkit-rs/rdkit/pull/51), [#52](https://github.com/rdkit-rs/rdkit/pull/52), [#54](https://github.com/rdkit-rs/rdkit/pull/54) (Sharif Haason: modernization, atom/bond views, graph editing, fingerprints and coordinate direction), and [#56](https://github.com/rdkit-rs/rdkit/pull/56) (Chris Petersen: functional scaffold networks and tests). Ownership and exception signatures were reworked as part of this audit. Incorporation into a draft does not by itself resolve or merge the original requests.
