use rdkit::ROMol;

#[test]
fn test_morgan_different_radius() {
    let mol = ROMol::from_smiles("c1ccccc1CC").unwrap();
    let fp_r2 = mol.morgan_fingerprint_with_params(2, 2048);
    let fp_r3 = mol.morgan_fingerprint_with_params(3, 2048);
    assert_ne!(fp_r2.0, fp_r3.0);
    assert_eq!(fp_r2.len(), 2048);
}

#[test]
fn test_morgan_custom_nbits() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    let fp = mol.morgan_fingerprint_with_params(2, 1024);
    assert_eq!(fp.len(), 1024);
}

#[test]
fn test_rdk_fingerprint_with_params() {
    let mol = ROMol::from_smiles("c1ccccc1CCCC").unwrap();
    let fp = mol.rdk_fingerprint_with_params(1, 7, 1024);
    assert_eq!(fp.len(), 1024);
}

#[test]
fn test_pattern_fingerprint_with_params() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    let fp = mol.pattern_fingerprint_with_params(1024);
    assert_eq!(fp.len(), 1024);
}

#[test]
fn test_maccs_fingerprint() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    let fp = mol.maccs_fingerprint();
    assert_eq!(fp.len(), 167);
}

#[test]
fn test_fingerprint_len() {
    let mol = ROMol::from_smiles("CC").unwrap();
    let fp = mol.morgan_fingerprint();
    assert!(!fp.is_empty());
    assert_eq!(fp.len(), 2048);
}
