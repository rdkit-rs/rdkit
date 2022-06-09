use std::fmt::{Debug, Formatter};

use cxx::let_cxx_string;
use rdkit_sys::*;

use crate::{Fingerprint, RWMol};

pub struct ROMol {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::ROMol>,
}

impl ROMol {
    pub fn from_smile(smile: &str) -> Result<Self, cxx::Exception> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = ro_mol_ffi::smiles_to_mol(&smile_cxx_string)?;
        Ok(Self { ptr })
    }

    pub fn from_smile_with_params(
        smile: &str,
        params: &SmilesParserParams,
    ) -> Result<Self, cxx::Exception> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = ro_mol_ffi::smiles_to_mol_with_params(&smile_cxx_string, params.ptr.clone())?;
        Ok(Self { ptr })
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
        Fingerprint::new(ptr)
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

pub struct SmilesParserParams {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::SmilesParserParams>,
}

impl SmilesParserParams {
    pub fn sanitize(&mut self, value: bool) {
        rdkit_sys::ro_mol_ffi::smiles_parser_params_set_sanitize(self.ptr.clone(), value);
    }
}

impl Default for SmilesParserParams {
    fn default() -> Self {
        SmilesParserParams {
            ptr: rdkit_sys::ro_mol_ffi::new_smiles_parser_params(),
        }
    }
}
