use std::fmt::{Debug, Formatter};

use cxx::let_cxx_string;
use rdkit_sys::*;

use crate::{Atom, Fingerprint, RWMol};

pub struct ROMol {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::ROMol>,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ROMolError {
    #[error("could not convert smile to romol (nullptr)")]
    UnknownConversionError,
    #[error("could not convert smile to romol (exception)")]
    ConversionException(String),
}

impl ROMol {
    pub fn from_smile(smile: &str) -> Result<Self, ROMolError> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = ro_mol_ffi::smiles_to_mol(&smile_cxx_string);
        match ptr {
            Ok(ptr) => {
                if ptr.is_null() {
                    Err(ROMolError::UnknownConversionError)
                } else {
                    Ok(ROMol { ptr })
                }
            }
            Err(e) => Err(ROMolError::ConversionException(e.to_string())),
        }
    }

    pub fn from_smile_with_params(
        smile: &str,
        params: &SmilesParserParams,
    ) -> Result<Self, cxx::Exception> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = ro_mol_ffi::smiles_to_mol_with_params(&smile_cxx_string, &params.ptr)?;
        Ok(Self { ptr })
    }

    pub fn as_smile(&self) -> String {
        ro_mol_ffi::mol_to_smiles(&self.ptr)
    }

    pub fn as_rw_mol(&self, quick_copy: bool, conf_id: i32) -> RWMol {
        let ptr = rw_mol_ffi::rw_mol_from_ro_mol(&self.ptr, quick_copy, conf_id);
        RWMol { ptr }
    }

    pub fn fingerprint(&self) -> Fingerprint {
        let ptr = fingerprint_ffi::fingerprint_mol(&self.ptr);
        Fingerprint::new(ptr)
    }

    pub fn num_atoms(&self, only_explicit: bool) -> u32 {
        ro_mol_ffi::get_num_atoms(&self.ptr, only_explicit)
    }

    pub fn atom_with_idx(&mut self, idx: u32) -> Atom {
        let ptr = ro_mol_ffi::get_atom_with_idx(&mut self.ptr, idx);
        Atom::from_ptr(ptr)
    }

    pub fn update_property_cache(&mut self, strict: bool) {
        ro_mol_ffi::ro_mol_update_property_cache(&mut self.ptr, strict)
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
            ptr: ro_mol_ffi::copy_mol(&self.ptr),
        }
    }
}

pub struct SmilesParserParams {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::SmilesParserParams>,
}

impl SmilesParserParams {
    pub fn sanitize(&mut self, value: bool) {
        ro_mol_ffi::smiles_parser_params_set_sanitize(&self.ptr, value);
    }
}

impl Default for SmilesParserParams {
    fn default() -> Self {
        SmilesParserParams {
            ptr: ro_mol_ffi::new_smiles_parser_params(),
        }
    }
}
