use rdkit_sys::scaffold_network_ffi::*;

#[test]
fn test_scaffold_network() {
    default_scaffold_network_params().expect("RDKit operation failed");
    let mut params = new_scaffold_network_params(&vec![]).expect("RDKit operation failed");

    set_include_generic_scaffolds(params.pin_mut(), true).expect("RDKit operation failed");
    include_generic_bond_scaffolds(params.pin_mut(), true).expect("RDKit operation failed");
    include_scaffolds_without_attachments(params.pin_mut(), true).expect("RDKit operation failed");
    include_scaffolds_with_attachments(params.pin_mut(), true).expect("RDKit operation failed");
    keep_only_first_fragment(params.pin_mut(), true).expect("RDKit operation failed");
    prune_before_fragmenting(params.pin_mut(), true).expect("RDKit operation failed");
    flatten_isotopes(params.pin_mut(), true).expect("RDKit operation failed");
    flatten_chirality(params.pin_mut(), true).expect("RDKit operation failed");
    flatten_keep_largest(params.pin_mut(), true).expect("RDKit operation failed");
    collect_mol_counts(params.pin_mut(), true).expect("RDKit operation failed");

    let _scaffold_network = default_scaffold_network().expect("RDKit operation failed");
}
