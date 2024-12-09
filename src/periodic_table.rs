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

    /// Returns the mass of the isotope
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    /// * `isotope` - The isotope number
    pub fn get_mass_for_isotope(atomic_number: u32, isotope: u32) -> f64 {
        rdkit_sys::periodic_table_ffi::get_periodic_table()
            .getMassForIsotope(atomic_number, isotope)
    }

    /// Returns the maximum recognized atomic number
    pub fn get_max_atomic_number() -> u32 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getMaxAtomicNumber()
    }

    /// Returns the abundance of the isotope
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    /// * `isotope` - The isotope number
    pub fn get_abundance_for_isotope(atomic_number: u32, isotope: u32) -> f64 {
        rdkit_sys::periodic_table_ffi::get_periodic_table()
            .getAbundanceForIsotope(atomic_number, isotope)
    }

    /// Returns true if the first atom is more electronegative than the second
    /// # Arguments
    /// * `atomic_number1` - The atomic number of the first element
    /// * `atomic_number2` - The atomic number of the second element
    pub fn more_electro_negative(atomic_number1: u32, atomic_number2: u32) -> bool {
        rdkit_sys::periodic_table_ffi::get_periodic_table()
            .moreElectroNegative(atomic_number1, atomic_number2)
    }

    /// Returns the row of the periodic table
    /// # Arguments
    /// * `atomic_number` - The atomic number of the element
    pub fn get_row(atomic_number: u32) -> u32 {
        rdkit_sys::periodic_table_ffi::get_periodic_table().getRow(atomic_number)
    }
}
