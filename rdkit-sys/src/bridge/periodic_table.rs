#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/periodic_table.h");
        type PeriodicTable;
        pub fn get_valence_list(atomic_number: u32) -> &'static CxxVector<i32>;
        pub fn getMostCommonIsotopeMass(self: &PeriodicTable, atom: &CxxString) -> f64;
    }
}
