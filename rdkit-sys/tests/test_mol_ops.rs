use rdkit_sys::mol_ops_ffi::*;

#[test]
fn test_substruct_match_as_bool() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();

    let params = new_remove_hs_parameters();

    let new_mol = remove_hs_parameters(&mol, &params, true);
    let new_smile = rdkit_sys::ro_mol_ffi::mol_to_smiles(&new_mol);

    assert_eq!(new_smile, "CCCCCCCCc1ccccc1");
}
