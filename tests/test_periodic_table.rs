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

#[test]
fn test_atomic_weight() {
    let weight = PeriodicTable::get_atomic_weight(6);
    assert_eq!(weight, 12.011);
}

#[test]
fn test_atomic_number() {
    let number = PeriodicTable::get_atomic_number("C");
    assert_eq!(number, 6);
}

#[test]
fn test_element_symbol() {
    let symbol = PeriodicTable::get_element_symbol(6);
    assert_eq!(symbol, "C");
}

#[test]
fn test_element_name() {
    let name = PeriodicTable::get_element_name(6);
    assert_eq!(name, "Carbon");
}

#[test]
fn test_radius_van_der_waals() {
    let radius = PeriodicTable::get_radius_van_der_waals(6);
    assert_eq!(radius, 1.7);
}

#[test]
fn test_radius_covalent() {
    let radius = PeriodicTable::get_radius_covalent(6);
    assert_eq!(radius, 0.76);
}

#[test]
fn test_radius_bond() {
    let radius = PeriodicTable::get_radius_b0(6);
    assert_eq!(radius, 0.77);
}

#[test]
fn test_default_valence() {
    let valence = PeriodicTable::get_default_valence(6);
    assert_eq!(valence, 4);
}

#[test]
fn test_outer_electrons() {
    let electrons = PeriodicTable::get_n_outer_elecs(6);
    assert_eq!(electrons, 4);
}
