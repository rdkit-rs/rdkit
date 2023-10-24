use cxx::CxxVector;

pub struct PeriodicTable {}

impl PeriodicTable {
    pub fn get_valence_list(atomic_number: u32) -> &'static CxxVector<i32> {
        rdkit_sys::periodic_table_ffi::get_valence_list(atomic_number)
    }
}
