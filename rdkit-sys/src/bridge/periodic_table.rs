// allow camel case
#![allow(non_snake_case)]

use cxx::{CxxVector, UniquePtr};
use ffi::PeriodicTable;

#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        type PeriodicTable;
        include!("wrapper/include/periodic_table.h");
        pub fn get_periodic_table() -> Result<UniquePtr<PeriodicTable>>;
        pub fn getAtomicWeight(self: &PeriodicTable, atomic_number: u32) -> Result<f64>;
        pub fn getAtomicNumber(self: &PeriodicTable, atom: &CxxString) -> Result<i32>;
        pub fn getElementSymbol(atomic_number: u32) -> Result<String>;
        pub fn getElementName(atomic_number: u32) -> Result<String>;
        pub fn getRvdw(self: &PeriodicTable, atomic_number: u32) -> Result<f64>;
        pub fn getRcovalent(self: &PeriodicTable, atomic_number: u32) -> Result<f64>;
        pub fn getRb0(self: &PeriodicTable, atomic_number: u32) -> Result<f64>;
        pub fn getDefaultValence(self: &PeriodicTable, atomic_number: u32) -> Result<i32>;
        fn getValenceList(atomic_number: u32) -> Result<UniquePtr<CxxVector<i32>>>;
        pub fn getNouterElecs(self: &PeriodicTable, atomic_number: u32) -> Result<i32>;
        pub fn getMostCommonIsotope(self: &PeriodicTable, atomic_number: u32) -> Result<i32>;
        pub fn getMostCommonIsotopeMass(self: &PeriodicTable, atom: &CxxString) -> Result<f64>;
        pub fn getMassForIsotope(
            self: &PeriodicTable,
            atomic_number: u32,
            isotope: u32,
        ) -> Result<f64>;
        pub fn periodic_table_max_atomic_number(table: &PeriodicTable) -> Result<u32>;
        pub fn getAbundanceForIsotope(
            self: &PeriodicTable,
            atomic_number: u32,
            isotope: u32,
        ) -> Result<f64>;
        pub fn moreElectroNegative(
            self: &PeriodicTable,
            atomic_number1: u32,
            atomic_number2: u32,
        ) -> Result<bool>;
        pub fn periodic_table_row(table: &PeriodicTable, atomic_number: u32) -> Result<u32>;
    }
}

pub trait PeriodicTableOps {
    fn getElementSymbol(&self, atomic_number: u32) -> Result<String, cxx::Exception>;
    fn getElementName(&self, atomic_number: u32) -> Result<String, cxx::Exception>;
    fn getValenceList(
        &self,
        atomic_number: u32,
    ) -> Result<UniquePtr<CxxVector<i32>>, cxx::Exception>;
}
impl PeriodicTableOps for UniquePtr<PeriodicTable> {
    fn getElementSymbol(&self, atomic_number: u32) -> Result<String, cxx::Exception> {
        ffi::getElementSymbol(atomic_number)
    }
    fn getElementName(&self, atomic_number: u32) -> Result<String, cxx::Exception> {
        ffi::getElementName(atomic_number)
    }
    fn getValenceList(
        &self,
        atomic_number: u32,
    ) -> Result<UniquePtr<CxxVector<i32>>, cxx::Exception> {
        ffi::getValenceList(atomic_number)
    }
}

impl PeriodicTable {
    pub fn getMaxAtomicNumber(&self) -> Result<u32, cxx::Exception> {
        ffi::periodic_table_max_atomic_number(self)
    }

    pub fn getRow(&self, atomic_number: u32) -> Result<u32, cxx::Exception> {
        ffi::periodic_table_row(self, atomic_number)
    }
}
