use bitvec::prelude::*;
use cxx::SharedPtr;

#[derive(Clone, Debug)]
pub struct Fingerprint(pub BitVec<u64, bitvec::order::Lsb0>);

impl Fingerprint {
    pub fn new(ptr: SharedPtr<rdkit_sys::fingerprint_ffi::ExplicitBitVect>) -> Self {
        let unique_ptr_bytes = rdkit_sys::fingerprint_ffi::explicit_bit_vect_to_bytes_vec(ptr);
        let rust_bytes: Vec<_> = unique_ptr_bytes.into_iter().map(|x| *x).collect();
        let bits = bitvec::vec::BitVec::<_, Lsb0>::from_vec(rust_bytes);
        Fingerprint(bits)
    }

    pub fn tanimoto_distance(&self, other: &Fingerprint) -> f32 {
        let and = self.0.clone() & &other.0;
        let or = self.0.clone() | &other.0;

        let and_ones = and.count_ones();
        let or_ones = or.count_ones();

        and_ones as f32 / or_ones as f32
    }
}
