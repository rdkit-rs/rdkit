use std::fmt::{Debug, Formatter};

use cxx::let_cxx_string;
use rdkit_sys::*;

use crate::{Atom, AtomRef, Bond, Conformer, ConformerMut, Fingerprint, RWMol};

pub struct ROMol {
    pub(crate) ptr: cxx::UniquePtr<ro_mol_ffi::ROMol>,
}

unsafe impl Send for ROMol {}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ROMolError {
    #[error("could not convert smiles to romol (nullptr)")]
    UnknownConversionError,
    #[error("could not convert smiles to romol (exception)")]
    ConversionException(String),
}

impl ROMol {
    pub fn from_smiles(smiles: &str) -> Result<Self, ROMolError> {
        let_cxx_string!(smiles = smiles);
        let ptr = ro_mol_ffi::smiles_to_mol(&smiles)
            .map_err(|e| ROMolError::ConversionException(e.to_string()))?;
        if ptr.is_null() {
            Err(ROMolError::UnknownConversionError)
        } else {
            Ok(Self { ptr })
        }
    }

    pub fn from_smiles_with_params(
        smiles: &str,
        params: &SmilesParserParams,
    ) -> Result<Self, String> {
        let_cxx_string!(smiles = smiles);
        let ptr = ro_mol_ffi::smiles_to_mol_with_params(&smiles, &params.ptr)
            .map_err(|e| e.to_string())?;
        if ptr.is_null() {
            Err("could not convert SMILES to molecule".into())
        } else {
            Ok(Self { ptr })
        }
    }

    pub fn try_as_smiles(&self) -> Result<String, cxx::Exception> {
        ro_mol_ffi::mol_to_smiles(&self.ptr)
    }

    pub fn as_smiles(&self) -> String {
        self.try_as_smiles().expect("RDKit SMILES writing failed")
    }

    pub fn try_to_molblock(&self) -> Result<String, cxx::Exception> {
        ro_mol_ffi::mol_to_molblock(&self.ptr)
    }

    pub fn to_molblock(&self) -> String {
        self.try_to_molblock()
            .expect("RDKit mol block writing failed")
    }

    pub fn try_as_rw_mol(&self, quick_copy: bool, conf_id: i32) -> Result<RWMol, cxx::Exception> {
        Ok(RWMol {
            ptr: rw_mol_ffi::rw_mol_from_ro_mol(&self.ptr, quick_copy, conf_id)?,
        })
    }

    pub fn as_rw_mol(&self, quick_copy: bool, conf_id: i32) -> RWMol {
        self.try_as_rw_mol(quick_copy, conf_id)
            .expect("RDKit molecule copy failed")
    }

    pub fn try_clone(&self) -> Result<Self, cxx::Exception> {
        Ok(Self {
            ptr: ro_mol_ffi::copy_mol(&self.ptr)?,
        })
    }

    pub fn try_rdk_fingerprint(&self) -> Result<Fingerprint, cxx::Exception> {
        Fingerprint::try_new(fingerprint_ffi::rdk_fingerprint_mol(&self.ptr)?)
    }

    pub fn rdk_fingerprint(&self) -> Fingerprint {
        self.try_rdk_fingerprint()
            .expect("RDKit fingerprint failed")
    }

    pub fn try_pattern_fingerprint(&self) -> Result<Fingerprint, cxx::Exception> {
        Fingerprint::try_new(fingerprint_ffi::pattern_fingerprint_mol(&self.ptr)?)
    }

    pub fn pattern_fingerprint(&self) -> Fingerprint {
        self.try_pattern_fingerprint()
            .expect("RDKit fingerprint failed")
    }

    pub fn try_morgan_fingerprint(&self) -> Result<Fingerprint, cxx::Exception> {
        Fingerprint::try_new(fingerprint_ffi::morgan_fingerprint_mol(&self.ptr)?)
    }

    pub fn morgan_fingerprint(&self) -> Fingerprint {
        self.try_morgan_fingerprint()
            .expect("RDKit fingerprint failed")
    }

    pub fn morgan_fingerprint_with_params(
        &self,
        radius: u32,
        n_bits: u32,
    ) -> Result<Fingerprint, cxx::Exception> {
        Fingerprint::try_new(fingerprint_ffi::morgan_fingerprint_mol_with_params(
            &self.ptr, radius, n_bits,
        )?)
    }

    pub fn rdk_fingerprint_with_params(
        &self,
        min_path: u32,
        max_path: u32,
        n_bits: u32,
    ) -> Result<Fingerprint, cxx::Exception> {
        Fingerprint::try_new(fingerprint_ffi::rdk_fingerprint_mol_with_params(
            &self.ptr, min_path, max_path, n_bits,
        )?)
    }

    pub fn pattern_fingerprint_with_params(
        &self,
        n_bits: u32,
    ) -> Result<Fingerprint, cxx::Exception> {
        Fingerprint::try_new(fingerprint_ffi::pattern_fingerprint_mol_with_params(
            &self.ptr, n_bits,
        )?)
    }

    pub fn maccs_fingerprint(&self) -> Result<Fingerprint, cxx::Exception> {
        Fingerprint::try_new(fingerprint_ffi::maccs_fingerprint_mol(&self.ptr)?)
    }

    pub fn num_atoms(&self, only_explicit: bool) -> u32 {
        ro_mol_ffi::get_num_atoms(&self.ptr, only_explicit).expect("RDKit atom count failed")
    }

    pub fn atom_ref(&self, idx: u32) -> Result<AtomRef<'_>, cxx::Exception> {
        Ok(AtomRef::from_ptr(ro_mol_ffi::get_atom_ref(&self.ptr, idx)?))
    }

    pub fn atoms(&self) -> impl ExactSizeIterator<Item = AtomRef<'_>> + '_ {
        (0..self.num_atoms(true)).map(|idx| self.atom_ref(idx).expect("RDKit atom access failed"))
    }

    pub fn try_atom_with_idx(&mut self, idx: u32) -> Result<Atom<'_>, cxx::Exception> {
        Ok(Atom::from_ptr(ro_mol_ffi::get_atom_with_idx(
            self.ptr.pin_mut(),
            idx,
        )?))
    }

    pub fn atom_with_idx(&mut self, idx: u32) -> Atom<'_> {
        self.try_atom_with_idx(idx)
            .expect("RDKit atom index out of range")
    }

    pub fn num_bonds(&self) -> u32 {
        bond_ffi::get_num_bonds(&self.ptr, true).expect("RDKit bond count failed")
    }

    pub fn bond(&self, idx: u32) -> Result<Bond<'_>, cxx::Exception> {
        Ok(Bond::from_ptr(bond_ffi::get_bond_with_idx(&self.ptr, idx)?))
    }

    pub fn bonds(&self) -> impl ExactSizeIterator<Item = Bond<'_>> + '_ {
        (0..self.num_bonds()).map(|idx| self.bond(idx).expect("RDKit bond access failed"))
    }

    pub fn bond_between_atoms(
        &self,
        begin: u32,
        end: u32,
    ) -> Result<Option<Bond<'_>>, cxx::Exception> {
        let idx = bond_ffi::get_bond_idx_between_atoms(&self.ptr, begin, end)?;
        if idx < 0 {
            Ok(None)
        } else {
            self.bond(idx as u32).map(Some)
        }
    }

    pub fn try_update_property_cache(&mut self, strict: bool) -> Result<(), cxx::Exception> {
        ro_mol_ffi::ro_mol_update_property_cache(self.ptr.pin_mut(), strict)
    }

    pub fn update_property_cache(&mut self, strict: bool) {
        self.try_update_property_cache(strict)
            .expect("RDKit property cache update failed")
    }

    pub fn compute_2d_coords(&mut self) -> Result<u32, cxx::Exception> {
        conformer_ffi::compute_2d_coords(self.ptr.pin_mut())
    }

    pub fn num_conformers(&self) -> u32 {
        conformer_ffi::get_num_conformers(&self.ptr).expect("RDKit conformer count failed")
    }

    pub fn conformer(&self, id: i32) -> Result<Conformer<'_>, cxx::Exception> {
        Ok(Conformer {
            ptr: conformer_ffi::get_conformer(&self.ptr, id)?,
        })
    }

    pub fn conformer_mut(&mut self, id: i32) -> Result<ConformerMut<'_>, cxx::Exception> {
        Ok(ConformerMut {
            ptr: conformer_ffi::get_conformer_mut(self.ptr.pin_mut(), id)?,
        })
    }
}

impl Debug for ROMol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ROMol").field(&self.try_as_smiles()).finish()
    }
}

impl Clone for ROMol {
    fn clone(&self) -> Self {
        self.try_clone().expect("RDKit molecule copy failed")
    }
}

pub struct SmilesParserParams {
    pub(crate) ptr: cxx::UniquePtr<ro_mol_ffi::SmilesParserParams>,
}

impl SmilesParserParams {
    pub fn set_sanitize(&mut self, value: bool) {
        ro_mol_ffi::smiles_parser_params_set_sanitize(self.ptr.pin_mut(), value)
            .expect("RDKit parser parameter update failed");
    }

    pub fn get_sanitize(&self) -> bool {
        ro_mol_ffi::smiles_parser_params_get_sanitize(&self.ptr)
            .expect("RDKit parser parameter access failed")
    }
}

impl Default for SmilesParserParams {
    fn default() -> Self {
        Self {
            ptr: ro_mol_ffi::new_smiles_parser_params().expect("RDKit parser allocation failed"),
        }
    }
}
