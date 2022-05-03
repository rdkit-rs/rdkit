use cxx::SharedPtr;

use rdkit_sys::*;

pub struct Fingerprint {
    pub(crate) ptr: SharedPtr<fingerprint_ffi::ExplicitBitVect>,
}

impl Clone for Fingerprint {
    fn clone(&self) -> Self {
        Fingerprint {
            ptr: fingerprint_ffi::copy_explicit_bit_vect(self.ptr.clone()),
        }
    }
}

impl Fingerprint {
    pub fn or(&self, other: &Fingerprint) -> Fingerprint {
        let clone = self.clone();
        fingerprint_ffi::fingerprint_or(clone.ptr.clone(), other.ptr.clone());
        clone
    }

    pub fn and(&self, other: &Fingerprint) -> Fingerprint {
        let clone = self.clone();
        fingerprint_ffi::fingerprint_and(clone.ptr.clone(), other.ptr.clone());
        clone
    }

    pub fn tanimoto_distance(&self, other: &Fingerprint) -> f32 {
        let and = self.and(other);
        let or = self.or(other);

        let and_ones = fingerprint_ffi::get_num_on_bits(and.ptr);
        let or_ones = fingerprint_ffi::get_num_on_bits(or.ptr);

        and_ones as f32 / or_ones as f32
    }
}
