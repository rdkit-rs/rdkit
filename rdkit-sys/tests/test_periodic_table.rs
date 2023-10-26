#[test]
fn test_get_valence_list() {
    let list = rdkit_sys::periodic_table_ffi::get_valence_list(1);
    assert_eq!(list.as_slice(), &[1]);
}
