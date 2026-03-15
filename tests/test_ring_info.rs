use rdkit::ROMol;

#[test]
fn test_benzene_ring_count() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    assert_eq!(mol.num_rings(), 1);
}

#[test]
fn test_naphthalene_ring_count() {
    let mol = ROMol::from_smiles("c1ccc2ccccc2c1").unwrap();
    assert_eq!(mol.num_rings(), 2);
}

#[test]
fn test_no_rings() {
    let mol = ROMol::from_smiles("CCCC").unwrap();
    assert_eq!(mol.num_rings(), 0);
}

#[test]
fn test_atom_in_ring_of_size() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    for i in 0..6 {
        assert!(mol.is_atom_in_ring_of_size(i, 6));
        assert!(!mol.is_atom_in_ring_of_size(i, 5));
    }
}

#[test]
fn test_atom_ring_sizes() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    assert_eq!(mol.atom_ring_sizes(0), vec![6]);
}

#[test]
fn test_naphthalene_junction_atoms() {
    let mol = ROMol::from_smiles("c1ccc2ccccc2c1").unwrap();
    let mut found = false;
    for i in 0..mol.num_atoms(true) {
        if mol.num_atom_rings(i) == 2 {
            found = true;
            break;
        }
    }
    assert!(found);
}
