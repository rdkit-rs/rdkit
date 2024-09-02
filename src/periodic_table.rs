use cxx::{let_cxx_string, CxxVector};

pub struct PeriodicTable {}

impl PeriodicTable {
    pub fn get_valence_list(atomic_number: u32) -> &'static CxxVector<i32> {
        rdkit_sys::periodic_table_ffi::get_valence_list(atomic_number)
    }

    pub fn get_most_common_isotope_mass(atom: &str) -> f64 {
        let_cxx_string!(atom_cxx_string = atom);
        rdkit_sys::periodic_table_ffi::get_most_common_isotope_mass(&atom_cxx_string)
    }
}
