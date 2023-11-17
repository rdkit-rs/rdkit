use rdkit::{substruct_match, ROMol, SubstructMatchParameters};

#[test]
fn test_substruct_match() {
    let mol = ROMol::from_smiles("c1ccccc1CCCCCCCC").unwrap();
    let query = ROMol::from_smiles("C").unwrap();
    let params = SubstructMatchParameters::new();

    let atom_matches = substruct_match(&mol, &query, &params);
    assert_ne!(atom_matches.len(), 0);

    let atom_match = atom_matches.get(0).unwrap();
    let atom_match_query_atom_idx = atom_match.query_atom_idx;

    assert_eq!(atom_match_query_atom_idx, 0);
}
