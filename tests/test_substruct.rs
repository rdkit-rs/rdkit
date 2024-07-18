use rdkit::graphmol::ro_mol::ROMol;
use rdkit::substruct_match::{substruct_match, SubstructMatchItem, SubstructMatchParameters};

#[test]
fn test_substruct_match() {
    let mol = ROMol::from_smiles("c1ccccc1CCCCCCCC").unwrap();
    let query = ROMol::from_smiles("C").unwrap();
    let params = SubstructMatchParameters::new();

    let atom_matches = substruct_match(&mol, &query, &params);
    assert_eq!(
        atom_matches,
        vec![
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 0,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 1,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 2,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 3,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 4,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 5,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 6,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 7,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 8,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 9,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 10,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 11,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 12,
            },],
            vec![SubstructMatchItem {
                query_atom_idx: 0,
                mol_atom_idx: 13,
            },],
        ]
    );
}
