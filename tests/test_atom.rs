#[test]
fn test_atom() {
    let mut romol = rdkit::ROMol::from_smiles("[NH4+]").unwrap();
    let atom = romol.atom_with_idx(0);

    assert_eq!(atom.symbol(), "N");
    assert!(!atom.get_is_aromatic());
    assert_eq!(atom.get_atomic_num(), 7);
    assert_eq!(atom.get_hybridization_type(), rdkit::HybridizationType::SP3);
    assert_eq!(atom.get_formal_charge(), 1);
    assert_eq!(atom.get_total_num_hs(), 4);
    assert_eq!(atom.get_total_valence(), 4);

    // TODO: these three need to be wrapped in a Result since it can throw an exception
    // assert_eq!(atoms[0].get_formal_charge(), 0);
    // assert_eq!(atoms[0].get_total_num_hs(), 100);
    // assert_eq!(atoms[0].get_total_valence(), 100);
}

#[test]
fn test_atom_update_property_cache() {
    let mut romol = rdkit::ROMol::from_smiles("C([H])([H])([H])([H])").unwrap();
    let mut carbon = romol.atom_with_idx(0);
    carbon.set_num_explicit_hs(5);

    // This throws an error in c++ which is not captured in rust
    // Note: the "strict" parameter checks the validity of the changes and errors out accordingly
    carbon.update_property_cache(true);
}
