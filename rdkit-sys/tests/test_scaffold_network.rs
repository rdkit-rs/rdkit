use rdkit_sys::scaffold_network_ffi::*;

#[test]
fn test_scaffold_network() {
    default_scaffold_network_params();
    let mut params = new_scaffold_network_params(&vec![]);

    set_include_generic_scaffolds(&mut params, true);
    include_generic_bond_scaffolds(&mut params, true);
    include_scaffolds_without_attachments(&mut params, true);
    include_scaffolds_with_attachments(&mut params, true);
    keep_only_first_fragment(&mut params, true);
    prune_before_fragmenting(&mut params, true);
    flatten_isotopes(&mut params, true);
    flatten_chirality(&mut params, true);
    flatten_keep_largest(&mut params, true);
    collect_mol_counts(&mut params, true);

    let _scaffold_network = default_scaffold_network();
}
