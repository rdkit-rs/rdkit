use std::fmt::{Debug, Formatter};

use cxx::let_cxx_string;

use rdkit_sys::*;

use crate::{Fingerprint, RWMol};

pub struct ROMol {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::ROMol>,
}

impl ROMol {
    pub fn from_smile(smile: &str) -> Option<Self> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = ro_mol_ffi::mol_from_smiles(&smile_cxx_string);
        Some(Self { ptr })
    }

    pub fn as_smile(&self) -> String {
        ro_mol_ffi::mol_to_smiles(self.ptr.clone())
    }

    pub fn as_rw_mol(&self, quick_copy: bool, conf_id: i32) -> RWMol {
        let ptr = rdkit_sys::rw_mol_ffi::rw_mol_from_ro_mol(self.ptr.clone(), quick_copy, conf_id);
        RWMol { ptr }
    }

    pub fn fingerprint(&self) -> Fingerprint {
        let ptr = fingerprint_ffi::fingerprint_mol(self.ptr.clone());
        Fingerprint { ptr }
    }
}

impl Debug for ROMol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let smile = self.as_smile();
        f.debug_tuple("ROMol").field(&smile).finish()
    }
}

impl Clone for ROMol {
    fn clone(&self) -> Self {
        ROMol {
            ptr: rdkit_sys::ro_mol_ffi::copy_mol(self.ptr.clone()),
        }
    }
}
