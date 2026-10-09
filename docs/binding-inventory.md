# Callable binding inventory

Baseline commit: `79a26ead54b2dbec2bedfc82976136b6c19017a6`. Names below are callable declarations inside the CXX `unsafe extern "C++"` blocks; shared structs, enums, Rust helper methods and overloads in upstream RDKit are not counted. Every baseline declaration and its native implementation was reviewed. A review entry does not mean every parameter combination has a runtime test. See [audit](binding-audit.md) for the ownership contracts and [validation](validation.md) for executed checks.

| Module | Before | Branch |
| --- | ---: | ---: |
| `bond` | 0 | 13 |
| `conformer` | 0 | 9 |
| `descriptors` | 3 | 3 |
| `fingerprint` | 5 | 10 |
| `mol_ops` | 37 | 37 |
| `mol_standardize` | 10 | 10 |
| `periodic_table` | 18 | 18 |
| `ro_mol` | 35 | 36 |
| `rw_mol` | 5 | 13 |
| `scaffold_network` | 13 | 21 |
| `substruct_match` | 19 | 19 |
| Total | 145 | 189 |

## bond

[Bridge](../rdkit-sys/src/bridge/bond.rs) · [C++ shim](../rdkit-sys/wrapper/src/bond.cc)

Baseline reviewed: new module.

Added callable names reviewed: `get_num_bonds`, `get_bond_with_idx`, `get_bond_idx_between_atoms`, `bond_get_bond_type`, `bond_get_bond_type_as_double`, `bond_get_begin_atom_idx`, `bond_get_end_atom_idx`, `bond_get_other_atom_idx`, `bond_get_is_aromatic`, `bond_get_is_conjugated`, `bond_get_stereo`, `bond_get_bond_dir`, `bond_get_idx`.

## conformer

[Bridge](../rdkit-sys/src/bridge/conformer.rs) · [C++ shim](../rdkit-sys/wrapper/src/conformer.cc)

Baseline reviewed: new module.

Added callable names reviewed: `compute_2d_coords`, `get_num_conformers`, `get_conformer`, `get_conformer_mut`, `conformer_id`, `conformer_is_3d`, `conformer_num_atoms`, `conformer_position`, `conformer_set_position`.

## descriptors

[Bridge](../rdkit-sys/src/bridge/descriptors.rs) · [C++ shim](../rdkit-sys/wrapper/src/descriptors.cc)

Baseline reviewed: `new_properties`, `get_property_names`, `compute_properties`.

## fingerprint

[Bridge](../rdkit-sys/src/bridge/fingerprint.rs) · [C++ shim](../rdkit-sys/wrapper/src/fingerprint.cc)

Baseline reviewed: `rdk_fingerprint_mol`, `pattern_fingerprint_mol`, `morgan_fingerprint_mol`, `copy_explicit_bit_vect`, `explicit_bit_vect_to_u64_vec`.

Added callable names reviewed: `explicit_bit_vect_num_bits`, `morgan_fingerprint_mol_with_params`, `rdk_fingerprint_mol_with_params`, `pattern_fingerprint_mol_with_params`, `maccs_fingerprint_mol`.

## mol_ops

[Bridge](../rdkit-sys/src/bridge/mol_ops.rs) · [C++ shim](../rdkit-sys/wrapper/src/mol_ops.cc)

Baseline reviewed: `new_remove_hs_parameters`, `get_remove_degree_zero`, `set_remove_degree_zero`, `get_remove_higher_degrees`, `set_remove_higher_degrees`, `get_remove_only_h_neighbors`, `set_remove_only_h_neighbors`, `get_remove_isotopes`, `set_remove_isotopes`, `get_remove_and_track_isotopes`, `set_remove_and_track_isotopes`, `get_remove_dummy_neighbors`, `set_remove_dummy_neighbors`, `get_remove_defining_bond_stereo`, `set_remove_defining_bond_stereo`, `get_remove_with_wedged_bond`, `set_remove_with_wedged_bond`, `get_remove_with_query`, `set_remove_with_query`, `get_remove_mapped`, `set_remove_mapped`, `get_remove_in_s_groups`, `set_remove_in_s_groups`, `get_show_warnings`, `set_show_warnings`, `get_remove_nonimplicit`, `set_remove_nonimplicit`, `get_update_explicit_count`, `set_update_explicit_count`, `get_remove_hydrides`, `set_remove_hydrides`, `get_remove_nontetrahedral_neighbors`, `set_remove_nontetrahedral_neighbors`, `remove_hs_parameters`, `add_hs`, `romol_set_hybridization`, `clean_up`.

## mol_standardize

[Bridge](../rdkit-sys/src/bridge/mol_standardize.rs) · [C++ shim](../rdkit-sys/wrapper/src/mol_standardize.cc)

Baseline reviewed: `tautomer_enumerator`, `tautomer_enumerate`, `tautomer_enumerator_result_tautomers_size`, `tautomer_enumerator_result_tautomers_at`, `tautomer_enumerator_canonicalize`, `default_cleanup_parameters`, `new_uncharger`, `uncharger_uncharge`, `fragment_parent`, `normalize`.

## periodic_table

[Bridge](../rdkit-sys/src/bridge/periodic_table.rs) · [C++ shim](../rdkit-sys/wrapper/src/periodic_table.cc)

Baseline reviewed: `get_periodic_table`, `getAtomicWeight`, `getAtomicNumber`, `getElementSymbol`, `getElementName`, `getRvdw`, `getRcovalent`, `getRb0`, `getDefaultValence`, `getValenceList`, `getNouterElecs`, `getMostCommonIsotope`, `getMostCommonIsotopeMass`, `getMassForIsotope`, `getMaxAtomicNumber`, `getAbundanceForIsotope`, `moreElectroNegative`, `getRow`.

Added callable names reviewed: `periodic_table_max_atomic_number`, `periodic_table_row`.

The former native `getMaxAtomicNumber` and `getRow` declarations are Rust forwarding methods over the compatibility shims. Their Rust call syntax is preserved.

## ro_mol

[Bridge](../rdkit-sys/src/bridge/ro_mol.rs) · [C++ shim](../rdkit-sys/wrapper/src/ro_mol.cc)

Baseline reviewed: `copy_mol`, `smiles_to_mol`, `smiles_to_mol_with_params`, `new_smiles_parser_params`, `smiles_parser_params_set_sanitize`, `smiles_parser_params_get_sanitize`, `mol_to_smiles`, `mol_to_molblock`, `detect_chemistry_problems`, `mol_sanitize_exception_type`, `atom_sanitize_exception_get_atom_idx`, `get_num_atoms`, `get_atom_with_idx`, `get_symbol`, `get_is_aromatic`, `get_atomic_num`, `get_formal_charge`, `get_total_num_hs`, `get_total_valence`, `set_formal_charge`, `set_num_explicit_hs`, `atom_update_property_cache`, `atom_set_hybridization`, `atom_get_hybridization`, `ro_mol_update_property_cache`, `set_int_prop`, `get_int_prop`, `set_float_prop`, `get_float_prop`, `set_bool_prop`, `get_bool_prop`, `set_prop`, `get_prop`, `get_num_radical_electrons`, `get_degree`.

Added callable names reviewed: `get_atom_ref`.

## rw_mol

[Bridge](../rdkit-sys/src/bridge/rw_mol.rs) · [C++ shim](../rdkit-sys/wrapper/src/rw_mol.cc)

Baseline reviewed: `rw_mol_from_mol_block`, `rw_mol_from_ro_mol`, `rw_mol_from_rw_mol`, `rw_mol_to_ro_mol`, `smarts_to_mol`.

Added callable names reviewed: `rw_mol_as_ro_mol`, `new_rw_mol`, `rw_mol_as_ro_mol_mut`, `rw_mol_add_atom`, `rw_mol_add_bond`, `rw_mol_remove_atom`, `rw_mol_remove_bond`, `rw_mol_sanitize`.

## scaffold_network

[Bridge](../rdkit-sys/src/bridge/scaffold_network.rs) · [C++ shim](../rdkit-sys/wrapper/src/scaffold_network.cc)

Baseline reviewed: `default_scaffold_network_params`, `new_scaffold_network_params`, `set_include_generic_scaffolds`, `include_generic_bond_scaffolds`, `include_scaffolds_without_attachments`, `include_scaffolds_with_attachments`, `keep_only_first_fragment`, `prune_before_fragmenting`, `flatten_isotopes`, `flatten_chirality`, `flatten_keep_largest`, `collect_mol_counts`, `default_scaffold_network`.

Added callable names reviewed: `create_scaffold_network`, `update_scaffold_network`, `scaffold_network_nodes`, `scaffold_network_counts`, `scaffold_network_mol_counts`, `scaffold_network_edge_begin_indices`, `scaffold_network_edge_end_indices`, `scaffold_network_edge_types`.

## substruct_match

[Bridge](../rdkit-sys/src/bridge/substruct_match.rs) · [C++ shim](../rdkit-sys/wrapper/src/substruct_match.cc)

Baseline reviewed: `substruct_match`, `new_substruct_match_parameters`, `get_use_chirality`, `get_use_enhanced_stereo`, `get_aromatic_matches_conjugated`, `get_use_query_query_matches`, `get_use_generic_matchers`, `get_recursion_possible`, `get_uniquify`, `set_use_chirality`, `set_use_enhanced_stereo`, `set_aromatic_matches_conjugated`, `set_use_query_query_matches`, `set_use_generic_matchers`, `set_recursion_possible`, `set_uniquify`, `substruct_matchvect_type_to_vec_substruct_match_item`, `substruct_match_item_query_atom_idx`, `substruct_match_item_mol_atom_idx`.
