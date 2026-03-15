use rdkit::ROMol;

#[test]
fn test_exact_mw_water() {
    let mol = ROMol::from_smiles("O").unwrap();
    assert!((mol.calc_exact_mw() - 18.010565).abs() < 0.001);
}

#[test]
fn test_mol_formula() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    assert_eq!(mol.calc_mol_formula(), "C6H6");
}

#[test]
fn test_num_heavy_atoms() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    assert_eq!(mol.calc_num_heavy_atoms(), 3);
}

#[test]
fn test_fraction_csp3() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    assert_eq!(mol.calc_fraction_csp3(), 1.0);
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    assert_eq!(mol.calc_fraction_csp3(), 0.0);
}

#[test]
fn test_tpsa() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    assert!(mol.calc_tpsa() > 0.0);
}

#[test]
fn test_clog_p() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    assert!(mol.calc_clog_p() > 0.0);
}

#[test]
fn test_hbd_hba() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    assert_eq!(mol.calc_num_hbd(), 1);
    assert_eq!(mol.calc_num_hba(), 1);
}

#[test]
fn test_aromatic_rings() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    assert_eq!(mol.calc_num_aromatic_rings(), 1);
    assert_eq!(mol.calc_num_aliphatic_rings(), 0);
}

#[test]
fn test_heterocycles() {
    let mol = ROMol::from_smiles("c1ccncc1").unwrap();
    assert_eq!(mol.calc_num_heterocycles(), 1);
    assert_eq!(mol.calc_num_aromatic_heterocycles(), 1);
}
