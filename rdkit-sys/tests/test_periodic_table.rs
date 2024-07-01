use cxx::let_cxx_string;

#[test]
fn test_get_valence_list() {
    let list = rdkit_sys::periodic_table_ffi::get_valence_list(1);
    assert_eq!(list.as_slice(), &[1]);
}

#[test]
fn test_get_monoisotopic_mass() {
    let_cxx_string!(atom = "C");
    let mass = rdkit_sys::periodic_table_ffi::get_most_common_isotope_mass(&atom);
    assert_eq!(mass, 12.00);
}
