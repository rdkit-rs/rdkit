use rdkit_sys::bond_ffi;
pub use rdkit_sys::bond_ffi::{BondDir, BondStereo, BondType};

pub struct Bond<'a> {
    ptr: &'a bond_ffi::Bond,
}

impl<'a> Bond<'a> {
    pub fn from_ptr(ptr: &'a bond_ffi::Bond) -> Self {
        Self { ptr }
    }

    pub fn bond_type(&self) -> BondType {
        bond_ffi::bond_get_bond_type(self.ptr).expect("RDKit bond access failed")
    }

    pub fn bond_type_as_double(&self) -> f64 {
        bond_ffi::bond_get_bond_type_as_double(self.ptr).expect("RDKit bond access failed")
    }

    pub fn begin_atom_idx(&self) -> u32 {
        bond_ffi::bond_get_begin_atom_idx(self.ptr).expect("RDKit bond access failed")
    }

    pub fn end_atom_idx(&self) -> u32 {
        bond_ffi::bond_get_end_atom_idx(self.ptr).expect("RDKit bond access failed")
    }

    pub fn other_atom_idx(&self, this_idx: u32) -> u32 {
        bond_ffi::bond_get_other_atom_idx(self.ptr, this_idx).expect("RDKit bond access failed")
    }

    pub fn try_other_atom_idx(&self, this_idx: u32) -> Result<u32, cxx::Exception> {
        bond_ffi::bond_get_other_atom_idx(self.ptr, this_idx)
    }

    pub fn is_aromatic(&self) -> bool {
        bond_ffi::bond_get_is_aromatic(self.ptr).expect("RDKit bond access failed")
    }

    pub fn is_conjugated(&self) -> bool {
        bond_ffi::bond_get_is_conjugated(self.ptr).expect("RDKit bond access failed")
    }

    pub fn stereo(&self) -> BondStereo {
        bond_ffi::bond_get_stereo(self.ptr).expect("RDKit bond access failed")
    }

    pub fn bond_dir(&self) -> BondDir {
        bond_ffi::bond_get_bond_dir(self.ptr).expect("RDKit bond access failed")
    }

    pub fn idx(&self) -> u32 {
        bond_ffi::bond_get_idx(self.ptr).expect("RDKit bond access failed")
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
