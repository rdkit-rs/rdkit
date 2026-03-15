use rdkit::{BondStereo, BondType, ROMol};

#[test]
fn test_bond_count() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    assert_eq!(mol.num_bonds(true), 2);
}

#[test]
fn test_bond_type_single() {
    let mut mol = ROMol::from_smiles("CC").unwrap();
    let bond = mol.bond_with_idx(0);
    assert_eq!(bond.bond_type(), BondType::SINGLE);
    assert_eq!(bond.bond_type_as_double(), 1.0);
}

#[test]
fn test_bond_type_double() {
    let mut mol = ROMol::from_smiles("C=C").unwrap();
    let bond = mol.bond_with_idx(0);
    assert_eq!(bond.bond_type(), BondType::DOUBLE);
}

#[test]
fn test_bond_type_triple() {
    let mut mol = ROMol::from_smiles("C#C").unwrap();
    let bond = mol.bond_with_idx(0);
    assert_eq!(bond.bond_type(), BondType::TRIPLE);
}

#[test]
fn test_bond_aromatic() {
    let mut mol = ROMol::from_smiles("c1ccccc1").unwrap();
    let bond = mol.bond_with_idx(0);
    assert!(bond.is_aromatic());
    assert_eq!(bond.bond_type(), BondType::AROMATIC);
}

#[test]
fn test_bond_atom_indices() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    let bond = mol.bond_with_idx(0);
    assert_eq!(bond.begin_atom_idx(), 0);
    assert_eq!(bond.end_atom_idx(), 1);
}

#[test]
fn test_bond_other_atom_idx() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    let bond = mol.bond_with_idx(0);
    assert_eq!(bond.other_atom_idx(0), 1);
    assert_eq!(bond.other_atom_idx(1), 0);
}

#[test]
fn test_bond_between_atoms() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    assert!(mol.bond_between_atoms(0, 1).is_some());
    let mut mol2 = ROMol::from_smiles("CCO").unwrap();
    assert!(mol2.bond_between_atoms(0, 2).is_none());
}

#[test]
fn test_bond_stereo_none() {
    let mut mol = ROMol::from_smiles("CC").unwrap();
    let bond = mol.bond_with_idx(0);
    assert_eq!(bond.stereo(), BondStereo::STEREONONE);
}

#[test]
fn test_bond_conjugated() {
    let mut mol = ROMol::from_smiles("C=CC=C").unwrap();
    let bond = mol.bond_with_idx(0);
    assert!(bond.is_conjugated());
}

#[test]
fn test_bond_idx() {
    let mut mol = ROMol::from_smiles("CCCO").unwrap();
    for i in 0..mol.num_bonds(true) {
        let bond = mol.bond_with_idx(i);
        assert_eq!(bond.idx(), i);
    }
}

#[test]
fn test_bond_count_benzene() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    assert_eq!(mol.num_bonds(true), 6);
}

#[test]
fn test_bond_debug_format() {
    let mut mol = ROMol::from_smiles("CC").unwrap();
    let bond = mol.bond_with_idx(0);
    let debug = format!("{:?}", bond);
    assert!(debug.contains("Bond"));
    assert!(debug.contains("SINGLE"));
}
