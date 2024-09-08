use cxx::let_cxx_string;
use rdkit_sys::*;
pub struct FragCatParams {
    pub(crate) ptr: cxx::SharedPtr<frag_catalog_ffi::FragCatParams>,
}

impl FragCatParams {
    pub fn new_from_string(text: &str) -> Self {
        let_cxx_string!(cxx_text = text);
        let ptr = rdkit_sys::frag_catalog_ffi::init_frat_cat_from_string(&cxx_text);
        FragCatParams { ptr }
    }
}
