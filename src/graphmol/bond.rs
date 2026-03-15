use std::pin::Pin;

use rdkit_sys::bond_ffi;
pub use rdkit_sys::bond_ffi::{BondDir, BondStereo, BondType};

pub struct Bond<'a> {
    ptr: Pin<&'a mut bond_ffi::Bond>,
}

impl<'a> Bond<'a> {
    pub fn from_ptr(ptr: Pin<&'a mut bond_ffi::Bond>) -> Self {
        Self { ptr }
    }

    pub fn bond_type(&self) -> BondType {
        bond_ffi::bond_get_bond_type(self.ptr.as_ref())
    }

    pub fn bond_type_as_double(&self) -> f64 {
        bond_ffi::bond_get_bond_type_as_double(self.ptr.as_ref())
    }

    pub fn begin_atom_idx(&self) -> u32 {
        bond_ffi::bond_get_begin_atom_idx(self.ptr.as_ref())
    }

    pub fn end_atom_idx(&self) -> u32 {
        bond_ffi::bond_get_end_atom_idx(self.ptr.as_ref())
    }

    pub fn other_atom_idx(&self, this_idx: u32) -> u32 {
        bond_ffi::bond_get_other_atom_idx(self.ptr.as_ref(), this_idx)
    }

    pub fn is_aromatic(&self) -> bool {
        bond_ffi::bond_get_is_aromatic(self.ptr.as_ref())
    }

    pub fn is_conjugated(&self) -> bool {
        bond_ffi::bond_get_is_conjugated(self.ptr.as_ref())
    }

    pub fn stereo(&self) -> BondStereo {
        bond_ffi::bond_get_stereo(self.ptr.as_ref())
    }

    pub fn bond_dir(&self) -> BondDir {
        bond_ffi::bond_get_bond_dir(self.ptr.as_ref())
    }

    pub fn idx(&self) -> u32 {
        bond_ffi::bond_get_idx(self.ptr.as_ref())
    }
}

impl std::fmt::Debug for Bond<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bond")
            .field("idx", &self.idx())
            .field("type", &self.bond_type())
            .field("begin", &self.begin_atom_idx())
            .field("end", &self.end_atom_idx())
            .finish()
    }
}
