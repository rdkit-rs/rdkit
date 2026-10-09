use std::collections::HashMap;

use cxx::UniquePtr;
use rdkit_sys::descriptors_ffi;

use crate::ROMol;

pub struct Properties {
    ptr: UniquePtr<descriptors_ffi::Properties>,
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}

impl Properties {
    pub fn new() -> Self {
        Self::try_new().expect("RDKit descriptor allocation failed")
    }

    pub fn try_new() -> Result<Self, cxx::Exception> {
        Ok(Self {
            ptr: descriptors_ffi::new_properties()?,
        })
    }

    pub fn compute_properties(&self, mol: &ROMol) -> HashMap<String, f64> {
        self.try_compute_properties(mol)
            .expect("RDKit descriptor calculation failed")
    }

    pub fn try_compute_properties(
        &self,
        mol: &ROMol,
    ) -> Result<HashMap<String, f64>, cxx::Exception> {
        let names = descriptors_ffi::get_property_names(&self.ptr)?;
        let values = descriptors_ffi::compute_properties(&self.ptr, &mol.ptr)?;
        Ok(names
            .iter()
            .zip(values.iter())
            .map(|(name, value)| (name.to_string(), *value))
            .collect())
    }
}
