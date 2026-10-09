use cxx::UniquePtr;
use rdkit_sys::ro_mol_ffi as ro_mol;

use crate::{ROMol, RWMol};

#[derive(Debug, PartialEq)]
pub enum MolSanitizeException {
    Other { kind: String },
    MolSanitizeException,
    AtomSanitizeException { atom_idx: u32 },
    KekulizeException, // todo don't support the vec<u32> return type just yet
    AtomKekulizeException { atom_idx: u32 },
    AtomValenceException { atom_idx: u32 },
}

pub fn detect_chemistry_problems(mol: &ROMol) -> Vec<MolSanitizeException> {
    try_detect_chemistry_problems(mol).expect("RDKit chemistry problem detection failed")
}

pub fn try_detect_chemistry_problems(
    mol: &ROMol,
) -> Result<Vec<MolSanitizeException>, cxx::Exception> {
    let problems = ro_mol::detect_chemistry_problems(&mol.ptr)?;
    problems
        .iter()
        .map(|problem| {
            let kind = ro_mol::mol_sanitize_exception_type(problem)?;
            Ok(match kind.as_str() {
                "MolSanitizeException" => MolSanitizeException::MolSanitizeException,
                "AtomSanitizeException" => MolSanitizeException::AtomSanitizeException {
                    atom_idx: ro_mol::atom_sanitize_exception_get_atom_idx(problem)?,
                },
                "KekulizeException" => MolSanitizeException::KekulizeException,
                "AtomKekulizeException" => MolSanitizeException::AtomKekulizeException {
                    atom_idx: ro_mol::atom_sanitize_exception_get_atom_idx(problem)?,
                },
                "AtomValenceException" => MolSanitizeException::AtomValenceException {
                    atom_idx: ro_mol::atom_sanitize_exception_get_atom_idx(problem)?,
                },
                _ => MolSanitizeException::Other { kind },
            })
        })
        .collect()
}

pub fn add_hs(mol: &ROMol, explicit_only: bool, add_coords: bool, add_residue_info: bool) -> ROMol {
    try_add_hs(mol, explicit_only, add_coords, add_residue_info)
        .expect("RDKit hydrogen addition failed")
}

pub fn try_add_hs(
    mol: &ROMol,
    explicit_only: bool,
    add_coords: bool,
    add_residue_info: bool,
) -> Result<ROMol, cxx::Exception> {
    Ok(ROMol {
        ptr: rdkit_sys::mol_ops_ffi::add_hs(&mol.ptr, explicit_only, add_coords, add_residue_info)?,
    })
}

pub fn remove_hs(mol: &ROMol, params: &RemoveHsParameters, sanitize: bool) -> ROMol {
    try_remove_hs(mol, params, sanitize).expect("RDKit hydrogen removal failed")
}

pub fn try_remove_hs(
    mol: &ROMol,
    params: &RemoveHsParameters,
    sanitize: bool,
) -> Result<ROMol, cxx::Exception> {
    Ok(ROMol {
        ptr: rdkit_sys::mol_ops_ffi::remove_hs_parameters(&mol.ptr, &params.ptr, sanitize)?,
    })
}

pub struct RemoveHsParameters {
    ptr: UniquePtr<rdkit_sys::mol_ops_ffi::RemoveHsParameters>,
}

impl Default for RemoveHsParameters {
    fn default() -> Self {
        RemoveHsParameters::new()
    }
}

impl RemoveHsParameters {
    pub fn new() -> Self {
        let ptr =
            rdkit_sys::mol_ops_ffi::new_remove_hs_parameters().expect("RDKit operation failed");
        RemoveHsParameters { ptr }
    }

    pub fn get_remove_degree_zero(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_degree_zero(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_remove_degree_zero(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_degree_zero(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_higher_degrees(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_higher_degrees(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn set_remove_higher_degrees(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_higher_degrees(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_only_h_neighbors(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_only_h_neighbors(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn set_remove_only_h_neighbors(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_only_h_neighbors(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_isotopes(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_isotopes(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_remove_isotopes(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_isotopes(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_and_track_isotopes(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_and_track_isotopes(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn set_remove_and_track_isotopes(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_and_track_isotopes(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_dummy_neighbors(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_dummy_neighbors(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn set_remove_dummy_neighbors(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_dummy_neighbors(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_defining_bond_stereo(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_defining_bond_stereo(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn set_remove_defining_bond_stereo(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_defining_bond_stereo(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_with_wedged_bond(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_with_wedged_bond(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn set_remove_with_wedged_bond(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_with_wedged_bond(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_with_query(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_with_query(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_remove_with_query(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_with_query(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_mapped(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_mapped(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_remove_mapped(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_mapped(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_in_s_groups(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_in_s_groups(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_remove_in_s_groups(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_in_s_groups(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_show_warnings(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_show_warnings(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_show_warnings(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_show_warnings(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_nonimplicit(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_nonimplicit(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_remove_nonimplicit(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_nonimplicit(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_update_explicit_count(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_update_explicit_count(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn set_update_explicit_count(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_update_explicit_count(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_hydrides(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_hydrides(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_remove_hydrides(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_hydrides(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn get_remove_nontetrahedral_neighbors(&self) -> bool {
        rdkit_sys::mol_ops_ffi::get_remove_nontetrahedral_neighbors(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn set_remove_nontetrahedral_neighbors(&mut self, what: bool) {
        rdkit_sys::mol_ops_ffi::set_remove_nontetrahedral_neighbors(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }
}

pub fn set_hybridization(romol: &mut ROMol) {
    rdkit_sys::mol_ops_ffi::romol_set_hybridization(romol.ptr.pin_mut())
        .expect("RDKit operation failed");
}

pub fn clean_up(rw_mol: &mut RWMol) {
    rdkit_sys::mol_ops_ffi::clean_up(rw_mol.ptr.pin_mut()).expect("RDKit operation failed");
}

pub fn try_set_hybridization(mol: &mut ROMol) -> Result<(), cxx::Exception> {
    rdkit_sys::mol_ops_ffi::romol_set_hybridization(mol.ptr.pin_mut())
}

pub fn try_clean_up(mol: &mut RWMol) -> Result<(), cxx::Exception> {
    rdkit_sys::mol_ops_ffi::clean_up(mol.ptr.pin_mut())
}
