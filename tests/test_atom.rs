#[test]
fn test_atom() {
    let mut romol = rdkit::ROMol::from_smile("C").unwrap();

    let atom = romol.atom_with_idx(0);

    assert_eq!(format!("{:?}", atom), "C");

    assert!(!atom.get_is_aromatic());
    assert_eq!(atom.get_atomic_num(), 6);

    assert_eq!(atom.get_hybridization_type(), rdkit::HybridizationType::SP3);

    for idx in 0..romol.num_atoms(true) {
        println!("fetching {}", idx);
        let atom = romol.atom_with_idx(idx);
        println!("{:?}", atom);
    }

    // TODO: these three need to be wrapped in a Result since it can throw an
    // exception
    //
    // assert_eq!(atoms[0].get_formal_charge(), 0);
    // assert_eq!(atoms[0].get_total_num_hs(), 100);
    // assert_eq!(atoms[0].get_total_valence(), 100);
}
