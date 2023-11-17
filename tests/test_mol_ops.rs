use rdkit::{add_hs, clean_up, remove_hs, set_hybridization, ROMol, RemoveHsParameters};

// # Create mol and check for chemistry problems
//        mol = Chem.MolFromSmiles(smiles, sanitize=False)
//        mol = fix_chem_problems(mol)
//
//        # Sanitize
//        Chem.SanitizeMol(mol)
//
//        # Track heavy hydrogens
//        heavy_hydrogen = False
//        if '[2H]' in smiles or '[3H]' in smiles:
//            heavy_hydrogen = True
//            remove_hs_params = Chem.RemoveHsParameters()
//            remove_hs_params.removeAndTrackIsotopes = True
//            remove_hs_params.removeDefiningBondStereo = True
//            mol = Chem.RemoveHs(mol, remove_hs_params)

#[test]
fn test_remove_hs() {
    let ro_mol = ROMol::from_smiles("[2H]C").unwrap();

    let mut remove_hs_parameters = RemoveHsParameters::new();
    remove_hs_parameters.set_remove_and_track_isotopes(true);
    remove_hs_parameters.set_remove_defining_bond_stereo(true);

    let new_mol = remove_hs(&ro_mol, &remove_hs_parameters, true);
    let new_smile = new_mol.as_smiles();
    assert_eq!(new_smile, "C");

    let new_new_mol = add_hs(&new_mol, true, true, true);
    let new_new_smile = new_new_mol.as_smiles();
    assert_eq!(new_new_smile, "C");

    let remove_hs_parameters = RemoveHsParameters::new();
    let mut new_new_new_mol = remove_hs(&new_new_mol, &remove_hs_parameters, true);
    let new_new_new_smile = new_new_new_mol.as_smiles();

    set_hybridization(&mut new_new_new_mol);

    assert_eq!(new_new_new_smile, "C");
}

#[test]
fn test_mol_ops_clean_up() {
    let ro_mol = ROMol::from_smiles("[H]C([H])([H])([H])").unwrap();
    let mut rw_mol = ro_mol.as_rw_mol(true, 0);
    clean_up(&mut rw_mol);
    let new_ro_mol = rw_mol.to_ro_mol();
    let smiles = new_ro_mol.as_smiles();
    assert_eq!(smiles, "C");
}
