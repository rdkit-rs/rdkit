#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/descriptors.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type Properties;

        pub fn new_properties() -> Result<UniquePtr<Properties>>;
        pub fn get_property_names(
            properties: &Properties,
        ) -> Result<UniquePtr<CxxVector<CxxString>>>;
        pub fn compute_properties(
            properties: &Properties,
            mol: &ROMol,
        ) -> Result<UniquePtr<CxxVector<f64>>>;
    }
}
