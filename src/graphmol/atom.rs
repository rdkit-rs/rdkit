use rdkit_sys::ro_mol_ffi;
use std::{fmt::Formatter, pin::Pin};

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

    // We create a generic function set_prop that can set any property type.
    // if the property type is an integer, we call set_int_prop, if it is a float, we call set_float_prop,
    // if it is a boolean, we call set_bool_prop and if it is a string, we call set_prop.
    pub fn set_prop<T>(&mut self, key: &str, value: T)
    where
        T: SetPropValue,
    {
        value.set_prop(self.ptr.as_mut(), key);
    }

    pub fn get_int_prop(&mut self, key: &str) -> Result<i32, cxx::Exception> {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::get_int_prop(self.ptr.as_mut(), &key)
    }

    pub fn get_float_prop(&mut self, key: &str) -> Result<f64, cxx::Exception> {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::get_float_prop(self.ptr.as_mut(), &key)
    }

    pub fn get_bool_prop(&mut self, key: &str) -> Result<bool, cxx::Exception> {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::get_bool_prop(self.ptr.as_mut(), &key)
    }

    pub fn get_prop(&mut self, key: &str) -> Result<String, cxx::Exception> {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::get_prop(self.ptr.as_mut(), &key)
    }
}

pub trait SetPropValue {
    fn set_prop(self, ptr: Pin<&mut ro_mol_ffi::Atom>, key: &str);
}

impl SetPropValue for i32 {
    fn set_prop(self, ptr: Pin<&mut ro_mol_ffi::Atom>, key: &str) {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::set_int_prop(ptr, &key, self);
    }
}

impl SetPropValue for f64 {
    fn set_prop(self, ptr: Pin<&mut ro_mol_ffi::Atom>, key: &str) {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::set_float_prop(ptr, &key, self);
    }
}

impl SetPropValue for bool {
    fn set_prop(self, ptr: Pin<&mut ro_mol_ffi::Atom>, key: &str) {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::set_bool_prop(ptr, &key, self);
    }
}

impl SetPropValue for &str {
    fn set_prop(self, ptr: Pin<&mut ro_mol_ffi::Atom>, key: &str) {
        cxx::let_cxx_string!(key = key);
        cxx::let_cxx_string!(value = self);
        ro_mol_ffi::set_prop(ptr, &key, &value);
    }
}
