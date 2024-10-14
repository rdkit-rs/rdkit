use cxx::{CxxVector, UniquePtr};
use ffi::PeriodicTable;

#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        type PeriodicTable;
        include!("wrapper/include/periodic_table.h");
        pub fn get_periodic_table() -> UniquePtr<PeriodicTable>;
        pub fn getAtomicWeight(self: &PeriodicTable, atomic_number: u32) -> f64;
        pub fn getAtomicNumber(self: &PeriodicTable, atom: &CxxString) -> i32;
        pub fn getElementSymbol(atomic_number: u32) -> String;
        pub fn getElementName(atomic_number: u32) -> String;
        pub fn getRvdw(self: &PeriodicTable, atomic_number: u32) -> f64;
        pub fn getRcovalent(self: &PeriodicTable, atomic_number: u32) -> f64;
        pub fn getRb0(self: &PeriodicTable, atomic_number: u32) -> f64;
        pub fn getDefaultValence(self: &PeriodicTable, atomic_number: u32) -> i32;
        fn getValenceList(atomic_number: u32) -> &'static CxxVector<i32>;
        pub fn getNouterElecs(self: &PeriodicTable, atomic_number: u32) -> i32;
        pub fn getMostCommonIsotope(self: &PeriodicTable, atomic_number: u32) -> i32;
        pub fn getMostCommonIsotopeMass(self: &PeriodicTable, atom: &CxxString) -> f64;
    }
}

pub trait PeriodicTableOps {
    fn getElementSymbol(self, atomic_number: u32) -> String;
    fn getElementName(self, atomic_number: u32) -> String;
    fn getValenceList(self, atomic_number: u32) -> &'static CxxVector<i32>;
}
impl<'a> PeriodicTableOps for UniquePtr<PeriodicTable> {
    fn getElementSymbol(self, atomic_number: u32) -> String {
        ffi::getElementSymbol(atomic_number)
    }

    fn getElementName(self, atomic_number: u32) -> String {
        ffi::getElementName(atomic_number)
    }

    fn getValenceList(self, atomic_number: u32) -> &'static CxxVector<i32> {
        ffi::getValenceList(atomic_number)
    }
}
