use cxx::let_cxx_string;
use rdkit_sys::scaffold_network_ffi::*;

#[test]
fn test_scaffold_network() {
    default_scaffold_network_params();
    let mut params = new_scaffold_network_params(&vec![]);

    set_include_generic_scaffolds(&mut params, true);
    set_include_generic_bond_scaffolds(&mut params, true);
    set_include_scaffolds_without_attachments(&mut params, true);
    set_include_scaffolds_with_attachments(&mut params, true);
    set_keep_only_first_fragment(&mut params, true);
    set_prune_before_fragmenting(&mut params, true);
    set_flatten_isotopes(&mut params, true);
    set_flatten_chirality(&mut params, true);
    set_flatten_keep_largest(&mut params, true);
    set_collect_mol_counts(&mut params, true);

    get_include_generic_scaffolds(&params);
    get_include_generic_bond_scaffolds(&params);
    get_include_scaffolds_without_attachments(&params);
    get_include_scaffolds_with_attachments(&params);
    get_keep_only_first_fragment(&params);
    get_prune_before_fragmenting(&params);
    get_flatten_isotopes(&params);
    get_flatten_chirality(&params);
    get_flatten_keep_largest(&params);
    get_collect_mol_counts(&params);

    let mut scaffold_network = default_scaffold_network();
    let_cxx_string!(smiles = "CCC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    update_scaffold_network(&mol, &mut scaffold_network, &params);
}
