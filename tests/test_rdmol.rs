use rdkit::ROMol;

#[test]
fn test_rdmol() {
    let _ = ROMol::from_smile("c1ccccc1C(=O)NC").unwrap();
}
