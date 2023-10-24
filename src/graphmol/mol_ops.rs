use cxx::SharedPtr;
use rdkit_sys::ro_mol_ffi as ro_mol;

use crate::ROMol;

pub fn detect_chemistry_problems(mol: &ROMol) -> Vec<(String, Option<u32>)> {
    let problems = rdkit_sys::ro_mol_ffi::detect_chemistry_problems(&mol.ptr);
    problems
        .iter()
        .map(|p| {
            let type_ = ro_mol::mol_sanitize_exception_type(p);
            match type_.as_str() {
                "AtomValenceException" => {
                    let atom_idx = ro_mol::atom_sanitize_exception_get_atom_idx(p);
                    (type_, Some(atom_idx))
                }
                _ => (type_, None),
            }
        })
        .collect()
}

pub fn add_hs(mol: &ROMol, explicit_only: bool, add_coords: bool, add_residue_info: bool) -> ROMol {
    let ptr = rdkit_sys::mol_ops_ffi::add_hs(&mol.ptr, explicit_only, add_coords, add_residue_info);
    ROMol { ptr }
}

pub fn remove_hs(
    ro_mol: &ROMol,
    remove_hs_parameters: &RemoveHsParameters,
    sanitize: bool,
) -> ROMol {
    let ptr = rdkit_sys::mol_ops_ffi::remove_hs_parameters(
        &ro_mol.ptr,
        &remove_hs_parameters.ptr,
        sanitize,
    );
    ROMol { ptr }
}

pub struct RemoveHsParameters {
    ptr: SharedPtr<rdkit_sys::mol_ops_ffi::RemoveHsParameters>,
}

impl RemoveHsParameters {
    pub fn get_remove_degree_zero(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_degree_zero(&self.ptr)
    }

    pub fn set_remove_degree_zero(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_degree_zero(&mut self.ptr, what)
    }

    pub fn get_remove_higher_degrees(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_higher_degrees(&self.ptr)
    }

    pub fn set_remove_higher_degrees(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_higher_degrees(&mut self.ptr, what)
    }

    pub fn get_remove_only_h_neighbors(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_only_h_neighbors(&self.ptr)
    }

    pub fn set_remove_only_h_neighbors(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_only_h_neighbors(&mut self.ptr, what)
    }

    pub fn get_remove_isotopes(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_isotopes(&self.ptr)
    }

    pub fn set_remove_isotopes(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_isotopes(&mut self.ptr, what)
    }

    pub fn get_remove_and_track_isotopes(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_and_track_isotopes(&self.ptr)
    }

    pub fn set_remove_and_track_isotopes(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_and_track_isotopes(&mut self.ptr, what)
    }

    pub fn get_remove_dummy_neighbors(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_dummy_neighbors(&self.ptr)
    }

    pub fn set_remove_dummy_neighbors(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_dummy_neighbors(&mut self.ptr, what)
    }

    pub fn get_remove_defining_bond_stereo(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_defining_bond_stereo(&self.ptr)
    }

    pub fn set_remove_defining_bond_stereo(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_defining_bond_stereo(&mut self.ptr, what)
    }

    pub fn get_remove_with_wedged_bond(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_with_wedged_bond(&self.ptr)
    }

    pub fn set_remove_with_wedged_bond(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_with_wedged_bond(&mut self.ptr, what)
    }

    pub fn get_remove_with_query(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_with_query(&self.ptr)
    }

    pub fn set_remove_with_query(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_with_query(&mut self.ptr, what)
    }

    pub fn get_remove_mapped(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_mapped(&self.ptr)
    }

    pub fn set_remove_mapped(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_mapped(&mut self.ptr, what)
    }

    pub fn get_remove_in_s_groups(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_in_s_groups(&self.ptr)
    }

    pub fn set_remove_in_s_groups(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_in_s_groups(&mut self.ptr, what)
    }

    pub fn get_show_warnings(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_show_warnings(&self.ptr)
    }

    pub fn set_show_warnings(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_show_warnings(&mut self.ptr, what)
    }

    pub fn get_remove_nonimplicit(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_nonimplicit(&self.ptr)
    }

    pub fn set_remove_nonimplicit(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_nonimplicit(&mut self.ptr, what)
    }

    pub fn get_update_explicit_count(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_update_explicit_count(&self.ptr)
    }

    pub fn set_update_explicit_count(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_update_explicit_count(&mut self.ptr, what)
    }

    pub fn get_remove_hydrides(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_hydrides(&self.ptr)
    }

    pub fn set_remove_hydrides(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_hydrides(&mut self.ptr, what)
    }

    pub fn get_remove_nontetrahedral_neighbors(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_nontetrahedral_neighbors(&self.ptr)
    }

    pub fn set_remove_nontetrahedral_neighbors(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_nontetrahedral_neighbors(&mut self.ptr, what)
    }
}
