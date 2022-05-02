use rdkit::Molecule;

#[test]
fn test_rdmol() {
    let _ = Molecule::from_smile("c1ccccc1C(=O)NC").unwrap();
}
