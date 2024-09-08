use cxx::let_cxx_string;

#[test]
fn test_frag_catalog() {
    let_cxx_string!(
        text = "0 20 1e-008\n\
                        1\n\
                        NitroGroup	[N+](=O)[O-]\n"
    );
    let _frag_cat_params = rdkit_sys::frag_catalog_ffi::init_frat_cat_from_string(&text);
}
