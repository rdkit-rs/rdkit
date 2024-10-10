#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        type PeriodicTable;
        include!("wrapper/include/periodic_table.h");
        pub fn get_valence_list(atomic_number: u32) -> &'static CxxVector<i32>;
        pub fn getMostCommonIsotopeMass(self: &PeriodicTable, atom: &CxxString) -> f64;
    }
}
