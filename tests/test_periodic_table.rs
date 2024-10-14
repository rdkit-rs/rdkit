use rdkit::PeriodicTable;

#[test]
fn test_valence_list() {
    let valence_list = PeriodicTable::get_valence_list(6);
    assert_eq!(valence_list.as_slice(), &[4]);
}

#[test]
fn test_most_common_isotope_mass() {
    let mass = PeriodicTable::get_most_common_isotope_mass("C");
    assert_eq!(mass, 12.00);
}
