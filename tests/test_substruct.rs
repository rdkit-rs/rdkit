use rdkit::{substruct_match, ROMol, SubstructMatchParameters};

#[test]
fn test_substruct_match() {
    let mol = ROMol::from_smiles("c1ccccc1CCCCCCCC").unwrap();
    let query = ROMol::from_smiles("C").unwrap();
    let params = SubstructMatchParameters::new();

    let does_it_match = substruct_match(&mol, &query, &params);
    assert_eq!(does_it_match.len(), 42);
}
