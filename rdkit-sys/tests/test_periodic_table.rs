// ignore camel case
#![allow(non_snake_case)]

use cxx::let_cxx_string;
use rdkit_sys::PeriodicTableOps;

#[test]
fn test_get_valence_list() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let list = periodic_table.getValenceList(6);
    assert_eq!(list.as_slice(), &[4]);
}

#[test]
fn test_get_monoisotopic_mass() {
    let_cxx_string!(atom = "C");
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let mass = periodic_table.getMostCommonIsotopeMass(&atom);
    assert_eq!(mass, 12.00);
}

#[test]
fn test_getAtomicWeight() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let weight = periodic_table.getAtomicWeight(6);
    assert_eq!(weight, 12.011);
}

#[test]
fn test_getAtomicNumber() {
    let_cxx_string!(atom = "C");
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let number = periodic_table.getAtomicNumber(&atom);
    assert_eq!(number, 6);
}

#[test]
fn test_getElementSymbol() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let symbol = periodic_table.getElementSymbol(6);
    assert_eq!(symbol, "C");
}

#[test]
fn test_getElementName() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let name = periodic_table.getElementName(6);
    assert_eq!(name, "Carbon");
}

#[test]
fn test_getRvdw() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let rvdw = periodic_table.getRvdw(6);
    assert_eq!(rvdw, 1.7);
}

#[test]
fn test_getRcovalent() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let rcovalent = periodic_table.getRcovalent(6);
    assert_eq!(rcovalent, 0.76);
}

#[test]
fn test_getRb0() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let rb0 = periodic_table.getRb0(6);
    assert_eq!(rb0, 0.77);
}

#[test]
fn test_getDefaultValence() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let valence = periodic_table.getDefaultValence(6);
    assert_eq!(valence, 4);
}

#[test]
fn test_getNouterElecs() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let nouter = periodic_table.getNouterElecs(6);
    assert_eq!(nouter, 4);
}

#[test]
fn test_getMostCommonIsotope() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let isotope = periodic_table.getMostCommonIsotope(6);
    assert_eq!(isotope, 12);
}

#[test]
fn test_getRow() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let row = periodic_table.getRow(6);
    assert_eq!(row, 2);
}

#[test]
fn test_getMassForIsotope() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let mass = periodic_table.getMassForIsotope(6, 13);
    assert_eq!(mass, 13.00335484);
}

#[test]
fn test_getMaxAtomicNumber() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let max = periodic_table.getMaxAtomicNumber();
    assert_eq!(max, 118);
}

#[test]
fn test_getAbundanceForIsotope() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let abundance = periodic_table.getAbundanceForIsotope(6, 13);
    assert_eq!(abundance, 1.07);
}

#[test]
fn test_moreElectroNegative() {
    let periodic_table = rdkit_sys::periodic_table_ffi::get_periodic_table();
    let more_electro = periodic_table.moreElectroNegative(6, 7);
    assert_eq!(more_electro, false);
}
