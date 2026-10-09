use rdkit_sys::conformer_ffi;
pub use rdkit_sys::conformer_ffi::Point3D;
use std::pin::Pin;

pub struct Conformer<'a> {
    pub(crate) ptr: &'a conformer_ffi::Conformer,
}

impl Conformer<'_> {
    pub fn id(&self) -> u32 {
        conformer_ffi::conformer_id(self.ptr).expect("RDKit conformer access failed")
    }

    pub fn is_3d(&self) -> bool {
        conformer_ffi::conformer_is_3d(self.ptr).expect("RDKit conformer access failed")
    }

    pub fn num_atoms(&self) -> u32 {
        conformer_ffi::conformer_num_atoms(self.ptr).expect("RDKit conformer access failed")
    }

    pub fn position(&self, idx: u32) -> Result<Point3D, cxx::Exception> {
        conformer_ffi::conformer_position(self.ptr, idx)
    }
}

pub struct ConformerMut<'a> {
    pub(crate) ptr: Pin<&'a mut conformer_ffi::Conformer>,
}

impl ConformerMut<'_> {
    pub fn as_ref(&self) -> Conformer<'_> {
        Conformer {
            ptr: self.ptr.as_ref().get_ref(),
        }
    }

    pub fn position(&self, idx: u32) -> Result<Point3D, cxx::Exception> {
        self.as_ref().position(idx)
    }

    pub fn set_position(&mut self, idx: u32, pos: Point3D) -> Result<(), cxx::Exception> {
        conformer_ffi::conformer_set_position(self.ptr.as_mut(), idx, pos)
    }
}
