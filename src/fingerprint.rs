use bitvec::prelude::*;
use cxx::UniquePtr;
use rdkit_sys::fingerprint_ffi;

#[derive(Clone, Debug)]
pub struct Fingerprint(pub BitVec<u8, bitvec::order::Lsb0>);

impl Fingerprint {
    pub fn new(ptr: UniquePtr<fingerprint_ffi::ExplicitBitVect>) -> Self {
        Self::try_new(ptr).expect("RDKit fingerprint conversion failed")
    }

    pub(crate) fn try_new(
        ptr: UniquePtr<fingerprint_ffi::ExplicitBitVect>,
    ) -> Result<Self, cxx::Exception> {
        let bits = fingerprint_ffi::explicit_bit_vect_num_bits(&ptr)? as usize;
        let words = fingerprint_ffi::explicit_bit_vect_to_u64_vec(&ptr)?;
        let mut bytes = Vec::with_capacity(words.len() * 8);
        for word in words.iter() {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        let mut result = BitVec::from_vec(bytes);
        result.truncate(bits);
        Ok(Self(result))
    }

    pub fn tanimoto_similarity(&self, other: &Fingerprint) -> f32 {
        assert_eq!(self.0.len(), other.0.len(), "fingerprint sizes must match");
        let intersection = (self.0.clone() & &other.0).count_ones();
        let union = self.0.count_ones() + other.0.count_ones() - intersection;
        if union == 0 {
            1.0
        } else {
            intersection as f32 / union as f32
        }
    }

    pub fn tanimoto_distance(&self, other: &Fingerprint) -> f32 {
        self.tanimoto_similarity(other)
    }
}
