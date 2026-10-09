# Migrating to the ownership API

This branch deliberately changes the low-level API and a few high-level signatures. It does not change crate versions or publish a release. Consumers should test against this branch before a release is chosen.

## Molecules and borrowed views

`ROMol` and `RWMol` exclusively own their native molecules. `clone()` makes a native deep copy; `try_clone()` reports native exceptions. `RWMol::to_ro_mol()` consumes the editable owner through C++'s base-class conversion. No Rust layout cast is involved.

Use `atom_ref(index)` or `atoms()` to read atoms through a shared molecule borrow. The existing `atom_with_idx(index)` needs a mutable molecule and returns an exclusive atom view; `try_atom_with_idx(index)` reports a bad index. Bond and conformer views are also tied to their molecule. Finish using a view before moving, editing or dropping its owner.

```rust
use rdkit::{BondType, RWMol};

let mut mol = RWMol::new();
let carbon = mol.add_atom(6)?;
let oxygen = mol.add_atom(8)?;
mol.add_bond(carbon, oxygen, BondType::SINGLE)?;
mol.sanitize()?;
assert_eq!(mol.atom_ref(oxygen)?.symbol(), "O");
assert_eq!(mol.as_smiles(), "CO");
```

Atom formal charges and explicit hydrogen counts are checked before conversion to RDKit's narrower storage. Graph edits validate atom indices, bond types, duplicate bonds and self-bonds. Successful edits can still produce chemically invalid molecules: explicitly call `sanitize()` or update property caches as appropriate to the workflow. A sanitization error does not roll the molecule back to its previous state.

Coordinate generation takes a mutable molecule. `conformer(id)` reads an existing conformer; `conformer_mut(id)` permits checked coordinate changes. An ID of `-1` asks RDKit for the default conformer. Only 2D generation is added here; these coordinate views do not imply ETKDG, MMFF or UFF support.

Owned molecules remain `Send`, so they can move to another thread or be protected by a mutex. They are not `Sync`: RDKit can update caches in apparently const operations. Borrowed views and raw native types do not promise cross-thread access. Do not add an unsafe `Sync` implementation to bypass this restriction.

## Fallible chemistry and parsing

Potentially throwing native calls return `Result` at the bridge. High-level `try_` alternatives cover molecule writing/copying, fingerprints, descriptor evaluation, hydrogen operations, standardization, periodic-table lookups and substructure matching. Existing convenience methods generally retain their return types and panic in Rust if a translated native exception occurs. Use fallible methods where input errors are expected.

`RWMol::from_smarts` now returns `Result<RWMol, RWMolError>` and rejects a native null result. The empty string remains a valid empty query, matching RDKit. `RWMol::try_from_mol_block` preserves an error; the existing `from_mol_block` remains an `Option` convenience method. Code that explicitly expects the old SMARTS error type must migrate to `RWMolError` or convert the error to text.

`MolSanitizeException` gains `Other { kind: String }`, preserving unknown native exception kinds instead of panicking. Exhaustive matches need a fallback. Existing `ROMolError` display text is retained; its exception variant still carries native detail.

`Uncharger::uncharge` and `try_uncharge` require `&mut self`, matching the native non-const method. Declare an uncharger binding mutable before calling them. Public raw owner fields have been hidden where they could invalidate the wrapper's invariants.

Periodic-table valence lists now return an owned `Vec<i32>` at the high level and an owned C++ vector at the bridge. Callers can keep using `.as_slice()`; code explicitly naming `&'static CxxVector<i32>` must change its type. This removes the assumption that RDKit's singleton storage outlives every Rust reference.

SDF iterators keep their existing item shapes. An I/O failure is returned once, followed by exhaustion. Invalid UTF-8 returns an error for that record; later delimiter-separated records can still be read. Records end at a complete `$$$$` line, and a missing final delimiter is accepted. Embedded dollar characters and an empty molecule title are preserved.

## Fingerprints, tautomers and scaffolds

Default fingerprints retain their previous parameters. New configurable Morgan, pattern and RDK fingerprints validate sizes and parameters; MACCS exposes the native 167-bit vector including its unused first bit. The bit vector length is exactly the native length, including nonmultiples of 64, and extraction no longer depends on Boost's internal block width.

Use `tanimoto_similarity` for the clearly named operation. The historical `tanimoto_distance` remains an alias for similarity to preserve callers' numerical behavior. Two empty or all-zero equal-length fingerprints have similarity 1; different lengths cause a Rust assertion instead of silently comparing a prefix.

Tautomer iteration reports the remaining count and returns independent molecule copies. `get(index)` and `try_next()` expose native errors without the iterator convenience method's panic behavior.

Scaffold networks now return owned nodes, edges and count data. Default fragmentation rules and explicitly empty bond-breaker rules are different choices. Edge direction and empty ring-free scaffolds follow RDKit; see the scaffold tests before imposing application-specific filtering.

## Low-level callers

Replace `SharedPtr<T>` owners with `UniquePtr<T>`. Borrow native inputs as `&T`; use `owner.pin_mut()` for `Pin<&mut T>` inputs. Handle the new `Result` values. Most factories turn a missing native object into an error. The two SMILES factories preserve nullable results to distinguish a nonthrowing parse failure; high-level constructors check those results before constructing a molecule. Safe borrowed inputs no longer accept a nullable smart-pointer reference. The consuming RWMol conversion still accepts a unique owner and checks it for null.

Borrowed native outputs cannot outlive their inputs. Cloning a smart pointer to manufacture another mutable path is no longer supported. The bridge's two reference upcasts are infallible; the remaining native calls translate exceptions. The [binding audit](binding-audit.md) records the reviewed surface and its limits.

For native installation, prefix overrides and the supported platform matrix, see [building](building.md). The matching native libraries must be present at runtime; this branch uses dynamic linking.

Supported native releases can legitimately return different chemistry results. One canonicalization regression yields a valid, reparsable molecule on 2023.09 and a kekulization error on 2025.03. The carbon covalent-radius table entry is 0.68 in the older package and 0.76 in the newer ones. Tests explicitly cover those observed outcomes; the wrappers preserve the selected native library's behavior.
