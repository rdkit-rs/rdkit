use std::collections::HashMap;

use cxx::SharedPtr;

use crate::ROMol;

impl ROMol {
    pub fn calc_exact_mw(&self) -> f64 {
        rdkit_sys::descriptors_ffi::calc_exact_mw(&self.ptr)
    }

    pub fn calc_amw(&self) -> f64 {
        rdkit_sys::descriptors_ffi::calc_amw(&self.ptr)
    }

    pub fn calc_mol_formula(&self) -> String {
        rdkit_sys::descriptors_ffi::calc_mol_formula(&self.ptr)
    }

    pub fn calc_num_heavy_atoms(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_heavy_atoms(&self.ptr)
    }

    pub fn calc_fraction_csp3(&self) -> f64 {
        rdkit_sys::descriptors_ffi::calc_fraction_csp3(&self.ptr)
    }

    pub fn calc_labute_asa(&self) -> f64 {
        rdkit_sys::descriptors_ffi::calc_labute_asa(&self.ptr)
    }

    pub fn calc_tpsa(&self) -> f64 {
        rdkit_sys::descriptors_ffi::calc_tpsa(&self.ptr)
    }

    pub fn calc_clog_p(&self) -> f64 {
        rdkit_sys::descriptors_ffi::calc_clog_p(&self.ptr)
    }

    pub fn calc_num_hbd(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_hbd(&self.ptr)
    }

    pub fn calc_num_hba(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_hba(&self.ptr)
    }

    pub fn calc_num_rotatable_bonds(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_rotatable_bonds(&self.ptr)
    }

    pub fn calc_num_amide_bonds(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_amide_bonds(&self.ptr)
    }

    pub fn calc_num_heteroatoms(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_heteroatoms(&self.ptr)
    }

    pub fn calc_num_aromatic_rings(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_aromatic_rings(&self.ptr)
    }

    pub fn calc_num_aliphatic_rings(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_aliphatic_rings(&self.ptr)
    }

    pub fn calc_num_saturated_rings(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_saturated_rings(&self.ptr)
    }

    pub fn calc_num_heterocycles(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_heterocycles(&self.ptr)
    }

    pub fn calc_num_aromatic_heterocycles(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_aromatic_heterocycles(&self.ptr)
    }

    pub fn calc_num_spiro_atoms(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_spiro_atoms(&self.ptr)
    }

    pub fn calc_num_bridgehead_atoms(&self) -> u32 {
        rdkit_sys::descriptors_ffi::calc_num_bridgehead_atoms(&self.ptr)
    }
}

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

        assert!(!names.is_empty());
        assert!(computed.len() == names.len());

        names
            .into_iter()
            .zip(computed.as_slice())
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    }
}
