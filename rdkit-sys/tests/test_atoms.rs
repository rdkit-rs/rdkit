#[test]
fn test_atoms() {
    cxx::let_cxx_string!(smiles = "c1ccccc1CCCCCCCC");
    let mut romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let num_atoms =
        rdkit_sys::ro_mol_ffi::get_num_atoms(&romol, true).expect("RDKit operation failed");

    assert_eq!(num_atoms, 14);

    let atoms = (0..num_atoms)
        .into_iter()
        .map(|idx| {
            let atom = rdkit_sys::ro_mol_ffi::get_atom_with_idx(romol.pin_mut(), idx)
                .expect("RDKit operation failed");
            rdkit_sys::ro_mol_ffi::get_symbol(atom.as_ref().get_ref())
                .expect("RDKit operation failed")
        })
        .collect::<Vec<_>>();

    assert_eq!(
        &atoms,
        &["C", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C"]
    );

    let mut atom = rdkit_sys::ro_mol_ffi::get_atom_with_idx(romol.pin_mut(), 2)
        .expect("RDKit operation failed");
    rdkit_sys::ro_mol_ffi::atom_set_hybridization(
        atom.as_mut(),
        rdkit_sys::ro_mol_ffi::HybridizationType::SP3.repr,
    )
    .expect("RDKit operation failed");
}
