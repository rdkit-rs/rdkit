#[test]
fn test_fingerprint_to_vec() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();

    let fingerprint = rdkit_sys::fingerprint_ffi::fingerprint_mol(&mol);
    let bytes = rdkit_sys::fingerprint_ffi::explicit_bit_vect_to_u64_vec(&fingerprint);
    let bytes: Vec<u64> = bytes.into_iter().map(|x| *x).collect();
    assert_eq!(bytes.len(), 32);
}
