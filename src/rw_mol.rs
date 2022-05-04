use cxx::SharedPtr;
use rdkit_sys::*;

pub struct RWMol {
    pub(crate) ptr: SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>
}

impl RWMol {
    pub fn as_smile(&self) -> String {
        let cast_ptr  = unsafe { std::mem::transmute::<SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>, SharedPtr<rdkit_sys::ro_mol_ffi::ROMol>>(self.ptr.clone()) };
        ro_mol_ffi::mol_to_smiles(cast_ptr)
    }
}