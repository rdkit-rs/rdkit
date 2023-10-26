use rdkit::PeriodicTable;

#[test]
fn test_valence_list() {
    let valence_list = PeriodicTable::get_valence_list(6);
    assert_eq!(&valence_list.iter().cloned().collect::<Vec<_>>(), &[4]);
}
