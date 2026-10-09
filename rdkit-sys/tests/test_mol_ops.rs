use rdkit_sys::mol_ops_ffi::*;

#[test]
fn test_mol_ops_substruct_match_as_bool() {
    cxx::let_cxx_string!(smiles = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let params = new_remove_hs_parameters().expect("RDKit operation failed");

    let mut new_mol = remove_hs_parameters(&mol, &params, true).expect("RDKit operation failed");
    let new_smiles =
        rdkit_sys::ro_mol_ffi::mol_to_smiles(&new_mol).expect("RDKit operation failed");

    romol_set_hybridization(new_mol.pin_mut()).expect("RDKit operation failed");

    assert_eq!(new_smiles, "CCCCCCCCc1ccccc1");
}

#[test]
fn test_mol_ops_cleanup() {
    cxx::let_cxx_string!(smiles = "[H]C([H])([H])([H])");
    let ro_mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    let mut rw_mol = rdkit_sys::rw_mol_ffi::rw_mol_from_ro_mol(&ro_mol, true, 0)
        .expect("RDKit operation failed");
    clean_up(rw_mol.pin_mut()).expect("RDKit operation failed");
    let new_ro_mol =
        rdkit_sys::rw_mol_ffi::rw_mol_to_ro_mol(rw_mol).expect("RDKit operation failed");
    let smiles = rdkit_sys::ro_mol_ffi::mol_to_smiles(&new_ro_mol).expect("RDKit operation failed");
    assert_eq!(smiles, "C");
}
