use cxx::SharedPtr;

pub struct RWMol {
    pub(crate) ptr: SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>
}