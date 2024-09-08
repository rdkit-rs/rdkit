#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/frag_catalog.h");

        pub type FragCatParams;

        pub fn init_frat_cat_from_string(text: &CxxString) -> SharedPtr<FragCatParams>;
    }
}
