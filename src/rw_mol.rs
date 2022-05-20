use crate::ROMol;
use cxx::{let_cxx_string, SharedPtr};
use rdkit_sys::*;
use std::fmt::Formatter;

pub struct RWMol {
    pub(crate) ptr: SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>,
}

impl RWMol {
    pub fn from_mol_block(
        mol_block: &str,
        sanitize: bool,
        remove_hs: bool,
        strict_parsing: bool,
    ) -> Option<Self> {
        let_cxx_string!(mol_block = mol_block);

        let ptr = rdkit_sys::rw_mol_ffi::rw_mol_from_mol_block(
            &mol_block,
            sanitize,
            remove_hs,
            strict_parsing,
        );

        if ptr.is_null() {
            None
        } else {
            Some(RWMol { ptr })
        }
    }

    pub fn as_smile(&self) -> String {
        let cast_ptr = unsafe {
            std::mem::transmute::<
                SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>,
                SharedPtr<rdkit_sys::ro_mol_ffi::ROMol>,
            >(self.ptr.clone())
        };
        ro_mol_ffi::mol_to_smiles(cast_ptr)
    }

    pub fn to_romol(&self) -> ROMol {
        let ptr = unsafe {
            std::mem::transmute::<
                SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>,
                SharedPtr<rdkit_sys::ro_mol_ffi::ROMol>,
            >(self.ptr.clone())
        };
        ROMol { ptr }
    }
}

impl Clone for RWMol {
    fn clone(&self) -> Self {
        let ptr = rw_mol_ffi::rw_mol_from_rw_mol(self.ptr.clone());
        RWMol { ptr }
    }
}

impl std::fmt::Debug for RWMol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let romol = self.to_romol();
        let smile = romol.as_smile();

        write!(f, "{}", smile)
    }
}
