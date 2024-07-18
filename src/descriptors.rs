use std::collections::HashMap;

use cxx::SharedPtr;

use crate::graphmol::ro_mol::ROMol;

pub struct Properties {
    ptr: SharedPtr<rdkit_sys::descriptors_ffi::Properties>,
}

impl Default for Properties {
    fn default() -> Self {
        Properties::new()
    }
}

impl Properties {
    pub fn new() -> Self {
        Properties {
            ptr: rdkit_sys::descriptors_ffi::new_properties(),
        }
    }

    pub fn compute_properties(&self, ro_mol: &ROMol) -> HashMap<String, f64> {
        let names = rdkit_sys::descriptors_ffi::get_property_names(&self.ptr);
        let computed = rdkit_sys::descriptors_ffi::compute_properties(&self.ptr, &ro_mol.ptr);

        assert!(names.len() != 0);
        assert!(computed.len() == names.len());

        names
            .into_iter()
            .zip(computed.as_slice())
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    }
}
