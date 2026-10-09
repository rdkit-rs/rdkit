use std::fmt::Formatter;

use rdkit_sys::ro_mol_ffi;
pub use rdkit_sys::ro_mol_ffi::HybridizationType;

pub struct AtomRef<'a> {
    ptr: &'a ro_mol_ffi::Atom,
}

impl<'a> std::fmt::Display for AtomRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.symbol();
        f.write_str(&symbol)
    }
}

impl<'a> std::fmt::Debug for AtomRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.symbol();
        f.write_str(&symbol)
    }
}

impl<'a> AtomRef<'a> {
    pub fn from_ptr(ptr: &'a ro_mol_ffi::Atom) -> Self {
        Self { ptr }
    }

    pub fn symbol(&self) -> String {
        ro_mol_ffi::get_symbol(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_is_aromatic(&self) -> bool {
        ro_mol_ffi::get_is_aromatic(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_atomic_num(&self) -> i32 {
        ro_mol_ffi::get_atomic_num(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_formal_charge(&self) -> i32 {
        ro_mol_ffi::get_formal_charge(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_total_num_hs(&self) -> u32 {
        ro_mol_ffi::get_total_num_hs(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_total_valence(&self) -> u32 {
        ro_mol_ffi::get_total_valence(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_hybridization_type(&self) -> HybridizationType {
        ro_mol_ffi::atom_get_hybridization(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_num_radical_electrons(&self) -> u32 {
        ro_mol_ffi::get_num_radical_electrons(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_degree(&self) -> u32 {
        ro_mol_ffi::get_degree(self.ptr).expect("RDKit atom access failed")
    }

    pub fn get_int_prop(&self, key: &str) -> Result<i32, cxx::Exception> {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::get_int_prop(self.ptr, &key)
    }

    pub fn get_float_prop(&self, key: &str) -> Result<f64, cxx::Exception> {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::get_float_prop(self.ptr, &key)
    }

    pub fn get_bool_prop(&self, key: &str) -> Result<bool, cxx::Exception> {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::get_bool_prop(self.ptr, &key)
    }

    pub fn get_prop(&self, key: &str) -> Result<String, cxx::Exception> {
        cxx::let_cxx_string!(key = key);
        ro_mol_ffi::get_prop(self.ptr, &key)
    }
    pub fn try_symbol(&self) -> Result<String, cxx::Exception> {
        ro_mol_ffi::get_symbol(self.ptr)
    }

    pub fn try_get_is_aromatic(&self) -> Result<bool, cxx::Exception> {
        ro_mol_ffi::get_is_aromatic(self.ptr)
    }

    pub fn try_get_atomic_num(&self) -> Result<i32, cxx::Exception> {
        ro_mol_ffi::get_atomic_num(self.ptr)
    }

    pub fn try_get_formal_charge(&self) -> Result<i32, cxx::Exception> {
        ro_mol_ffi::get_formal_charge(self.ptr)
    }

    pub fn try_get_total_num_hs(&self) -> Result<u32, cxx::Exception> {
        ro_mol_ffi::get_total_num_hs(self.ptr)
    }

    pub fn try_get_total_valence(&self) -> Result<u32, cxx::Exception> {
        ro_mol_ffi::get_total_valence(self.ptr)
    }

    pub fn try_get_hybridization_type(&self) -> Result<HybridizationType, cxx::Exception> {
        ro_mol_ffi::atom_get_hybridization(self.ptr)
    }

    pub fn try_get_num_radical_electrons(&self) -> Result<u32, cxx::Exception> {
        ro_mol_ffi::get_num_radical_electrons(self.ptr)
    }

    pub fn try_get_degree(&self) -> Result<u32, cxx::Exception> {
        ro_mol_ffi::get_degree(self.ptr)
    }
}
