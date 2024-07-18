use rdkit_sys::mol_ops_ffi::*;

#[test]
fn test_mol_ops_substruct_match_as_bool() {
    cxx::let_cxx_string!(smiles = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let params = new_remove_hs_parameters();

    let mut new_mol = remove_hs_parameters(&mol, &params, true);
    let new_smiles = rdkit_sys::ro_mol_ffi::mol_to_smiles(&new_mol);

    romol_set_hybridization(&mut new_mol);

    assert_eq!(new_smiles, "CCCCCCCCc1ccccc1");
}

#[test]
fn test_mol_ops_cleanup() {
    cxx::let_cxx_string!(smiles = "[H]C([H])([H])([H])");
    let ro_mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    let mut rw_mol = rdkit_sys::rw_mol_ffi::rw_mol_from_ro_mol(&ro_mol, true, 0);
    clean_up(&mut rw_mol);
    let new_ro_mol = rdkit_sys::rw_mol_ffi::rw_mol_to_ro_mol(rw_mol); // low-cost pointer swap
    let smiles = rdkit_sys::ro_mol_ffi::mol_to_smiles(&new_ro_mol);
    assert_eq!(smiles, "C");
}

#[test]
fn test_number_of_fragments() {
    cxx::let_cxx_string!(smiles = "CC");
    let ro_mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    let num_frags = rdkit_sys::mol_ops_ffi::get_number_of_fragments(&ro_mol);
    assert_eq!(num_frags, 1);

    cxx::let_cxx_string!(smiles = "CC(=O)[O-].[NH3+]C");
    let ro_mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    let num_frags = rdkit_sys::mol_ops_ffi::get_number_of_fragments(&ro_mol);
    assert_eq!(num_frags, 2);
}
