use rdkit::ROMol;

#[test]
fn test_no_conformers_initially() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    assert_eq!(mol.num_conformers(), 0);
}

#[test]
fn test_embed_molecule() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    let conf_id = mol.embed_molecule();
    assert!(conf_id >= 0);
    assert_eq!(mol.num_conformers(), 1);
}

#[test]
fn test_embed_molecule_3d() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    let conf_id = mol.embed_molecule();
    assert!(mol.conformer_is_3d(conf_id));
}

#[test]
fn test_embed_multiple_confs() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    let ids = mol.embed_multiple_confs(5);
    assert_eq!(ids.len(), 5);
    assert_eq!(mol.num_conformers(), 5);
}

#[test]
fn test_compute_2d_coords() {
    let mut mol = ROMol::from_smiles("c1ccccc1").unwrap();
    let conf_id = mol.compute_2d_coords();
    assert!(!mol.conformer_is_3d(conf_id as i32));
}

#[test]
fn test_atom_position_3d() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    let conf_id = mol.embed_molecule();
    let (x, y, z) = mol.atom_position(conf_id, 0);
    assert!(x != 0.0 || y != 0.0 || z != 0.0);
}

#[test]
fn test_2d_coords_z_zero() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    let conf_id = mol.compute_2d_coords();
    for i in 0..mol.num_atoms(true) {
        let (_, _, z) = mol.atom_position(conf_id as i32, i);
        assert_eq!(z, 0.0);
    }
}
