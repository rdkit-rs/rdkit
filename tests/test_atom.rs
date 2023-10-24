#[test]
fn test_atom() {
    let romol = rdkit::ROMol::from_smile("C").unwrap();

    let atom_iter = romol.atoms(true);
    let atoms = atom_iter.collect::<Vec<_>>();

    assert_eq!(format!("{:?}", atoms), "[C]");

    assert_eq!(atoms[0].get_is_aromatic(), false);
    assert_eq!(atoms[0].get_atomic_num(), 6);

    // these three need to be wrapped in a Result since it can throw an
    // exception
    //
    // assert_eq!(atoms[0].get_formal_charge(), 0);
    // assert_eq!(atoms[0].get_total_num_hs(), 100);
    // assert_eq!(atoms[0].get_total_valence(), 100);
}
