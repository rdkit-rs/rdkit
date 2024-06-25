use std::{fmt::Formatter, pin::Pin};

use rdkit_sys::ro_mol_ffi;

pub struct Atom<'a> {
    ptr: Pin<&'a mut ro_mol_ffi::Atom>,
}
pub use rdkit_sys::ro_mol_ffi::HybridizationType;

impl<'a> std::fmt::Display for Atom<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.symbol();
        f.write_str(&symbol)
    }
}

impl<'a> std::fmt::Debug for Atom<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.symbol();
        f.write_str(&symbol)
    }
}

impl<'a> Atom<'a> {
    pub fn from_ptr(ptr: Pin<&'a mut ro_mol_ffi::Atom>) -> Self {
        Self { ptr }
    }

    pub fn symbol(&self) -> String {
        ro_mol_ffi::get_symbol(self.ptr.as_ref())
    }

    pub fn get_is_aromatic(&self) -> bool {
        ro_mol_ffi::get_is_aromatic(self.ptr.as_ref())
    }

    pub fn get_atomic_num(&self) -> i32 {
        ro_mol_ffi::get_atomic_num(self.ptr.as_ref())
    }

    pub fn get_formal_charge(&self) -> i32 {
        ro_mol_ffi::get_formal_charge(self.ptr.as_ref())
    }

    pub fn get_total_num_hs(&self) -> u32 {
        ro_mol_ffi::get_total_num_hs(self.ptr.as_ref())
    }

    pub fn get_total_valence(&self) -> u32 {
        ro_mol_ffi::get_total_valence(self.ptr.as_ref())
    }

    pub fn set_formal_charge(&mut self, what: i32) {
        ro_mol_ffi::set_formal_charge(self.ptr.as_mut(), what)
    }

    pub fn set_num_explicit_hs(&mut self, what: i32) {
        ro_mol_ffi::set_num_explicit_hs(self.ptr.as_mut(), what)
    }

    pub fn update_property_cache(&mut self, strict: bool) -> Result<(), cxx::Exception> {
        ro_mol_ffi::atom_update_property_cache(self.ptr.as_mut(), strict)
    }

    pub fn set_hybridization_type(&mut self, what: HybridizationType) {
        ro_mol_ffi::atom_set_hybridization(self.ptr.as_mut(), what);
    }

    pub fn get_hybridization_type(&self) -> HybridizationType {
        ro_mol_ffi::atom_get_hybridization(self.ptr.as_ref())
    }
}
