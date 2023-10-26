use rdkit_sys::substruct_match_ffi::*;

#[test]
fn test_substruct_match_as_bool() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();

    cxx::let_cxx_string!(smile = "C");
    let query = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();

    let match_params = new_substruct_match_parameters();

    let hits = substruct_match_as_bool(&mol, &query, &match_params);

    assert_eq!(hits, true);
}
