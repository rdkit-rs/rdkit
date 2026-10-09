use cxx::let_cxx_string;
use rdkit_sys::{periodic_table_ffi as ffi, PeriodicTableOps};

pub struct PeriodicTable {}

impl PeriodicTable {
    pub fn get_valence_list(atomic_number: u32) -> Vec<i32> {
        Self::try_get_valence_list(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_valence_list(atomic_number: u32) -> Result<Vec<i32>, cxx::Exception> {
        let values = ffi::get_periodic_table()?.getValenceList(atomic_number)?;
        Ok(values.iter().copied().collect())
    }

    pub fn get_most_common_isotope_mass(atom: &str) -> f64 {
        Self::try_get_most_common_isotope_mass(atom).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_most_common_isotope_mass(atom: &str) -> Result<f64, cxx::Exception> {
        let_cxx_string!(atom = atom);
        ffi::get_periodic_table()?.getMostCommonIsotopeMass(&atom)
    }

    pub fn get_atomic_weight(atomic_number: u32) -> f64 {
        Self::try_get_atomic_weight(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_atomic_weight(atomic_number: u32) -> Result<f64, cxx::Exception> {
        ffi::get_periodic_table()?.getAtomicWeight(atomic_number)
    }

    pub fn get_atomic_number(atom: &str) -> i32 {
        Self::try_get_atomic_number(atom).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_atomic_number(atom: &str) -> Result<i32, cxx::Exception> {
        let_cxx_string!(atom = atom);
        ffi::get_periodic_table()?.getAtomicNumber(&atom)
    }

    pub fn get_element_symbol(atomic_number: u32) -> String {
        Self::try_get_element_symbol(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_element_symbol(atomic_number: u32) -> Result<String, cxx::Exception> {
        ffi::getElementSymbol(atomic_number)
    }

    pub fn get_element_name(atomic_number: u32) -> String {
        Self::try_get_element_name(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_element_name(atomic_number: u32) -> Result<String, cxx::Exception> {
        ffi::getElementName(atomic_number)
    }

    pub fn get_radius_van_der_waals(atomic_number: u32) -> f64 {
        Self::try_get_radius_van_der_waals(atomic_number)
            .expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_radius_van_der_waals(atomic_number: u32) -> Result<f64, cxx::Exception> {
        ffi::get_periodic_table()?.getRvdw(atomic_number)
    }

    pub fn get_radius_covalent(atomic_number: u32) -> f64 {
        Self::try_get_radius_covalent(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_radius_covalent(atomic_number: u32) -> Result<f64, cxx::Exception> {
        ffi::get_periodic_table()?.getRcovalent(atomic_number)
    }

    pub fn get_radius_b0(atomic_number: u32) -> f64 {
        Self::try_get_radius_b0(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_radius_b0(atomic_number: u32) -> Result<f64, cxx::Exception> {
        ffi::get_periodic_table()?.getRb0(atomic_number)
    }

    pub fn get_default_valence(atomic_number: u32) -> i32 {
        Self::try_get_default_valence(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_default_valence(atomic_number: u32) -> Result<i32, cxx::Exception> {
        ffi::get_periodic_table()?.getDefaultValence(atomic_number)
    }

    pub fn get_n_outer_elecs(atomic_number: u32) -> i32 {
        Self::try_get_n_outer_elecs(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_n_outer_elecs(atomic_number: u32) -> Result<i32, cxx::Exception> {
        ffi::get_periodic_table()?.getNouterElecs(atomic_number)
    }

    pub fn get_most_common_isotope(atomic_number: u32) -> i32 {
        Self::try_get_most_common_isotope(atomic_number)
            .expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_most_common_isotope(atomic_number: u32) -> Result<i32, cxx::Exception> {
        ffi::get_periodic_table()?.getMostCommonIsotope(atomic_number)
    }

    pub fn get_mass_for_isotope(atomic_number: u32, isotope: u32) -> f64 {
        Self::try_get_mass_for_isotope(atomic_number, isotope)
            .expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_mass_for_isotope(
        atomic_number: u32,
        isotope: u32,
    ) -> Result<f64, cxx::Exception> {
        ffi::get_periodic_table()?.getMassForIsotope(atomic_number, isotope)
    }

    pub fn get_max_atomic_number() -> u32 {
        Self::try_get_max_atomic_number().expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_max_atomic_number() -> Result<u32, cxx::Exception> {
        ffi::get_periodic_table()?.getMaxAtomicNumber()
    }

    pub fn get_abundance_for_isotope(atomic_number: u32, isotope: u32) -> f64 {
        Self::try_get_abundance_for_isotope(atomic_number, isotope)
            .expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_abundance_for_isotope(
        atomic_number: u32,
        isotope: u32,
    ) -> Result<f64, cxx::Exception> {
        ffi::get_periodic_table()?.getAbundanceForIsotope(atomic_number, isotope)
    }

    pub fn more_electro_negative(atomic_number1: u32, atomic_number2: u32) -> bool {
        Self::try_more_electro_negative(atomic_number1, atomic_number2)
            .expect("RDKit periodic table lookup failed")
    }

    pub fn try_more_electro_negative(
        atomic_number1: u32,
        atomic_number2: u32,
    ) -> Result<bool, cxx::Exception> {
        ffi::get_periodic_table()?.moreElectroNegative(atomic_number1, atomic_number2)
    }

    pub fn get_row(atomic_number: u32) -> u32 {
        Self::try_get_row(atomic_number).expect("RDKit periodic table lookup failed")
    }

    pub fn try_get_row(atomic_number: u32) -> Result<u32, cxx::Exception> {
        ffi::get_periodic_table()?.getRow(atomic_number)
    }
}
