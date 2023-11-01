use rdkit::{substruct_match::substruct_match, ROMol, SubstructMatchParameters};

#[test]
fn test_substruct_match() {
    let mol = ROMol::from_smile("c1ccccc1CCCCCCCC").unwrap();
    let query = ROMol::from_smile("C").unwrap();
    let params = SubstructMatchParameters::new();

    let does_it_match = substruct_match(&mol, &query);
    assert_eq!(does_it_match, true);
}
