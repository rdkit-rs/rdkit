use rdkit::{BondType, RWMol, sanitize_mol};

#[test]
fn test_new_empty_rw_mol() {
    let mol = RWMol::new();
    assert_eq!(mol.num_atoms(true), 0);
}

#[test]
fn test_add_atoms() {
    let mut mol = RWMol::new();
    let c_idx = mol.add_atom(6);
    assert_eq!(c_idx, 0);
    assert_eq!(mol.num_atoms(true), 1);
    let o_idx = mol.add_atom(8);
    assert_eq!(o_idx, 1);
    assert_eq!(mol.num_atoms(true), 2);
}

#[test]
fn test_build_ethanol() {
    let mut mol = RWMol::new();
    mol.add_atom(6);
    mol.add_atom(6);
    mol.add_atom(8);
    mol.add_bond(0, 1, BondType::SINGLE);
    mol.add_bond(1, 2, BondType::SINGLE);
    sanitize_mol(&mut mol).unwrap();
    assert_eq!(mol.as_smiles(), "CCO");
}

#[test]
fn test_build_ethene() {
    let mut mol = RWMol::new();
    mol.add_atom(6);
    mol.add_atom(6);
    mol.add_bond(0, 1, BondType::DOUBLE);
    sanitize_mol(&mut mol).unwrap();
    assert_eq!(mol.as_smiles(), "C=C");
}

#[test]
fn test_remove_atom() {
    let mut mol = RWMol::new();
    mol.add_atom(6);
    mol.add_atom(6);
    mol.add_atom(8);
    mol.add_bond(0, 1, BondType::SINGLE);
    mol.add_bond(1, 2, BondType::SINGLE);
    assert_eq!(mol.num_atoms(true), 3);
    mol.remove_atom(2);
    assert_eq!(mol.num_atoms(true), 2);
}

#[test]
fn test_remove_bond() {
    let mut mol = RWMol::new();
    mol.add_atom(6);
    mol.add_atom(6);
    mol.add_atom(8);
    mol.add_bond(0, 1, BondType::SINGLE);
    mol.add_bond(1, 2, BondType::SINGLE);
    mol.remove_bond(1, 2);
    sanitize_mol(&mut mol).unwrap();
}
