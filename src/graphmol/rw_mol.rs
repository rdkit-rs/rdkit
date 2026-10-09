use std::fmt::Formatter;

use cxx::{let_cxx_string, UniquePtr};
use rdkit_sys::*;

use crate::{Atom, AtomRef, Bond, BondType, Conformer, ConformerMut, ROMol};

pub struct RWMol {
    pub(crate) ptr: UniquePtr<rw_mol_ffi::RWMol>,
}

unsafe impl Send for RWMol {}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RWMolError {
    #[error("could not parse molecule: {0}")]
    ConversionException(String),
    #[error("parser returned no molecule")]
    UnknownConversionError,
}

impl RWMol {
    pub fn new() -> Self {
        Self::try_new().expect("RDKit molecule allocation failed")
    }

    pub fn try_new() -> Result<Self, cxx::Exception> {
        Ok(Self {
            ptr: rw_mol_ffi::new_rw_mol()?,
        })
    }

    pub fn try_from_mol_block(
        mol_block: &str,
        sanitize: bool,
        remove_hs: bool,
        strict_parsing: bool,
    ) -> Result<Self, RWMolError> {
        let_cxx_string!(mol_block = mol_block);
        let ptr =
            rw_mol_ffi::rw_mol_from_mol_block(&mol_block, sanitize, remove_hs, strict_parsing)
                .map_err(|e| RWMolError::ConversionException(e.to_string()))?;
        if ptr.is_null() {
            Err(RWMolError::UnknownConversionError)
        } else {
            Ok(Self { ptr })
        }
    }

    pub fn from_mol_block(
        mol_block: &str,
        sanitize: bool,
        remove_hs: bool,
        strict_parsing: bool,
    ) -> Option<Self> {
        Self::try_from_mol_block(mol_block, sanitize, remove_hs, strict_parsing).ok()
    }

    pub fn try_as_smiles(&self) -> Result<String, cxx::Exception> {
        ro_mol_ffi::mol_to_smiles(rw_mol_ffi::rw_mol_as_ro_mol(&self.ptr))
    }

    pub fn as_smiles(&self) -> String {
        self.try_as_smiles().expect("RDKit SMILES writing failed")
    }

    pub fn to_ro_mol(self) -> ROMol {
        ROMol {
            ptr: rw_mol_ffi::rw_mol_to_ro_mol(self.ptr).expect("RDKit molecule conversion failed"),
        }
    }

    pub fn from_smarts(smarts: &str) -> Result<Self, RWMolError> {
        let_cxx_string!(smarts = smarts);
        let ptr = rw_mol_ffi::smarts_to_mol(&smarts)
            .map_err(|e| RWMolError::ConversionException(e.to_string()))?;
        if ptr.is_null() {
            Err(RWMolError::UnknownConversionError)
        } else {
            Ok(Self { ptr })
        }
    }

    pub fn try_clone(&self) -> Result<Self, cxx::Exception> {
        Ok(Self {
            ptr: rw_mol_ffi::rw_mol_from_rw_mol(&self.ptr)?,
        })
    }

    pub fn num_atoms(&self, only_explicit: bool) -> u32 {
        ro_mol_ffi::get_num_atoms(rw_mol_ffi::rw_mol_as_ro_mol(&self.ptr), only_explicit)
            .expect("RDKit atom count failed")
    }

    pub fn atom_ref(&self, idx: u32) -> Result<AtomRef<'_>, cxx::Exception> {
        Ok(AtomRef::from_ptr(ro_mol_ffi::get_atom_ref(
            rw_mol_ffi::rw_mol_as_ro_mol(&self.ptr),
            idx,
        )?))
    }

    pub fn atoms(&self) -> impl ExactSizeIterator<Item = AtomRef<'_>> + '_ {
        (0..self.num_atoms(true)).map(|idx| self.atom_ref(idx).expect("RDKit atom access failed"))
    }

    pub fn try_atom_with_idx(&mut self, idx: u32) -> Result<Atom<'_>, cxx::Exception> {
        Ok(Atom::from_ptr(ro_mol_ffi::get_atom_with_idx(
            rw_mol_ffi::rw_mol_as_ro_mol_mut(self.ptr.pin_mut()),
            idx,
        )?))
    }

    pub fn atom_with_idx(&mut self, idx: u32) -> Atom<'_> {
        self.try_atom_with_idx(idx)
            .expect("RDKit atom index out of range")
    }

    pub fn num_bonds(&self) -> u32 {
        bond_ffi::get_num_bonds(rw_mol_ffi::rw_mol_as_ro_mol(&self.ptr), true)
            .expect("RDKit bond count failed")
    }

    pub fn bond(&self, idx: u32) -> Result<Bond<'_>, cxx::Exception> {
        Ok(Bond::from_ptr(bond_ffi::get_bond_with_idx(
            rw_mol_ffi::rw_mol_as_ro_mol(&self.ptr),
            idx,
        )?))
    }

    pub fn bonds(&self) -> impl ExactSizeIterator<Item = Bond<'_>> + '_ {
        (0..self.num_bonds()).map(|idx| self.bond(idx).expect("RDKit bond access failed"))
    }

    pub fn add_atom(&mut self, atomic_num: u32) -> Result<u32, cxx::Exception> {
        rw_mol_ffi::rw_mol_add_atom(self.ptr.pin_mut(), atomic_num)
    }

    pub fn add_bond(
        &mut self,
        begin: u32,
        end: u32,
        kind: BondType,
    ) -> Result<u32, cxx::Exception> {
        rw_mol_ffi::rw_mol_add_bond(self.ptr.pin_mut(), begin, end, kind.repr)
    }

    pub fn remove_atom(&mut self, idx: u32) -> Result<(), cxx::Exception> {
        rw_mol_ffi::rw_mol_remove_atom(self.ptr.pin_mut(), idx)
    }

    pub fn remove_bond(&mut self, begin: u32, end: u32) -> Result<(), cxx::Exception> {
        rw_mol_ffi::rw_mol_remove_bond(self.ptr.pin_mut(), begin, end)
    }

    pub fn sanitize(&mut self) -> Result<(), cxx::Exception> {
        rw_mol_ffi::rw_mol_sanitize(self.ptr.pin_mut())
    }

    pub fn compute_2d_coords(&mut self) -> Result<u32, cxx::Exception> {
        conformer_ffi::compute_2d_coords(rw_mol_ffi::rw_mol_as_ro_mol_mut(self.ptr.pin_mut()))
    }

    pub fn conformer(&self, id: i32) -> Result<Conformer<'_>, cxx::Exception> {
        Ok(Conformer {
            ptr: conformer_ffi::get_conformer(rw_mol_ffi::rw_mol_as_ro_mol(&self.ptr), id)?,
        })
    }

    pub fn conformer_mut(&mut self, id: i32) -> Result<ConformerMut<'_>, cxx::Exception> {
        Ok(ConformerMut {
            ptr: conformer_ffi::get_conformer_mut(
                rw_mol_ffi::rw_mol_as_ro_mol_mut(self.ptr.pin_mut()),
                id,
            )?,
        })
    }
}

impl Default for RWMol {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for RWMol {
    fn clone(&self) -> Self {
        self.try_clone().expect("RDKit molecule copy failed")
    }
}

impl std::fmt::Debug for RWMol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RWMol").field(&self.try_as_smiles()).finish()
    }
}
