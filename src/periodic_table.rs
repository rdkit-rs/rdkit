use cxx::{let_cxx_string, CxxVector};
use rdkit_sys::PeriodicTableOps;

pub struct PeriodicTable {}

impl PeriodicTable {
    /// Returns a vector of all stable valences. For atoms where we really don't
    /// have any idea what a reasonable maximum valence is (like transition
    /// metals), the vector ends with -1
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_valence_list(atomic_number: u32) -> &'static CxxVector<i32> {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getValenceList(atomic_number)
    }

    /// Returns the mass of the most common isotope
    /// # Arguments
    /// * `atom` - The symbol of the element
    pub fn get_most_common_isotope_mass(atom: &str) -> f64 {
        let_cxx_string!(atom_cxx_string = atom);
        rdkit_sys::periodic_table_ffi::get_periodic_table()
            .getMostCommonIsotopeMass(&atom_cxx_string)
    }

    /// Returns the atomic weight of the atom
    pub fn get_atomic_weight(atomic_number: u32) -> f64 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getAtomicWeight(atomic_number)
    }

    /// Returns the atomic number of the atom
    /// # Arguments
    /// * `atom` - The symbol of the element
    pub fn get_atomic_number(atom: &str) -> i32 {
        let_cxx_string!(atom_cxx_string = atom);
        rdkit_sys::periodic_table_ffi::get_periodic_table().getAtomicNumber(&atom_cxx_string)
    }

    /// Returns the symbol of the element
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_element_symbol(atomic_number: u32) -> String {
        rdkit_sys::periodic_table_ffi::getElementSymbol(atomic_number)
    }

    /// Returns the full element name
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_element_name(atomic_number: u32) -> String {
        rdkit_sys::periodic_table_ffi::getElementName(atomic_number)
    }

    /// Returns the atom's Van der Waals radius
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_radius_van_der_waals(atomic_number: u32) -> f64 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getRvdw(atomic_number)
    }

    /// Returns the atom's covalent radius
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_radius_covalent(atomic_number: u32) -> f64 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getRcovalent(atomic_number)
    }

    /// Returns the atom's bond radius
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_radius_b0(atomic_number: u32) -> f64 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getRb0(atomic_number)
    }

    /// Returns the atom's default valence
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_default_valence(atomic_number: u32) -> i32 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getDefaultValence(atomic_number)
    }

    /// Returns the number of outer shell electrons
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_n_outer_elecs(atomic_number: u32) -> i32 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getNouterElecs(atomic_number)
    }

    /// Returns the atom's most common isotope
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_most_common_isotope(atomic_number: u32) -> i32 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getMostCommonIsotope(atomic_number)
    }
}
