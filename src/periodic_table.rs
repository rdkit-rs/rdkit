use cxx::{let_cxx_string, CxxVector};
use rdkit_sys::PeriodicTableOps;

pub struct PeriodicTable {}

impl PeriodicTable {
    pub fn get_valence_list(atomic_number: u32) -> &'static CxxVector<i32> {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getValenceList(atomic_number)
    }

    pub fn get_most_common_isotope_mass(atom: &str) -> f64 {
        let_cxx_string!(atom_cxx_string = atom);
        rdkit_sys::periodic_table_ffi::get_periodic_table()
            .getMostCommonIsotopeMass(&atom_cxx_string)
    }
}
