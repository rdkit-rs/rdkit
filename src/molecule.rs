use std::fmt::{Debug, Formatter};

use cxx::let_cxx_string;

use rdkit_sys::{
    fingerprint::ffi as fingerprint_ffi, ro_mol::ffi as rdmol_ffi,
};

use crate::Fingerprint;

pub struct Molecule {
    ptr: cxx::SharedPtr<rdmol_ffi::ROMol>,
}

impl Molecule {
    pub fn from_smile(smile: &str) -> Option<Self> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = rdmol_ffi::mol_from_smiles(&smile_cxx_string);
        Some(Self { ptr })
    }

    pub fn fingerprint(&self) -> Fingerprint {
        let ptr = fingerprint_ffi::fingerprint_mol(self.ptr.clone());
        Fingerprint { ptr }
    }
}

impl Debug for Molecule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let smile = rdmol_ffi::mol_to_smiles(self.ptr.clone());
        f.debug_struct("Molecule").field("ptr", &smile).finish()
    }
}
