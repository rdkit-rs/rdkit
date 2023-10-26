use rdkit::{add_hs, remove_hs, ROMol, RemoveHsParameters};

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
    let ro_mol = ROMol::from_smile("[2H]C").unwrap();

    let mut remove_hs_parameters = RemoveHsParameters::new();
    remove_hs_parameters.set_remove_and_track_isotopes(true);
    remove_hs_parameters.set_remove_defining_bond_stereo(true);

    let new_mol = remove_hs(&ro_mol, &remove_hs_parameters, true);
    let new_smile = new_mol.as_smile();
    assert_eq!(new_smile, "C");

    let new_new_mol = add_hs(&new_mol, true, true, true);
    let new_new_smile = new_new_mol.as_smile();

    let mut remove_hs_parameters = RemoveHsParameters::new();
    let new_new_new_mol = remove_hs(&new_new_mol, &remove_hs_parameters, true);
    let new_new_new_smile = new_new_new_mol.as_smile();

    assert_eq!(new_new_new_smile, "C");
}
