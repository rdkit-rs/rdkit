use bitvec::prelude::*;
use cxx::SharedPtr;

#[derive(Clone, Debug)]
pub struct Fingerprint(pub BitVec<u8, bitvec::order::Lsb0>);

impl Fingerprint {
    pub fn new(ptr: SharedPtr<rdkit_sys::fingerprint_ffi::ExplicitBitVect>) -> Self {
        let unique_ptr_bytes = rdkit_sys::fingerprint_ffi::explicit_bit_vect_to_u64_vec(&ptr);
        let rdkit_fingerprint_bytes: Vec<u64> = unique_ptr_bytes.into_iter().map(|x| *x).collect();
        let mut bitvec_u64 = bitvec::vec::BitVec::<u64, Lsb0>::from_vec(rdkit_fingerprint_bytes);

        let mut idiomatic_bitvec_u8 = bitvec::vec::BitVec::<u8, Lsb0>::new();
        idiomatic_bitvec_u8.append(&mut bitvec_u64);

        Fingerprint(idiomatic_bitvec_u8)
    }

    pub fn tanimoto_distance(&self, other: &Fingerprint) -> f32 {
        let and = self.0.clone() & &other.0;
        let or = self.0.clone() | &other.0;

        let and_ones = and.count_ones();
        let or_ones = or.count_ones();

        and_ones as f32 / or_ones as f32
    }
}

#[cfg(test)]
mod tests {
    use crate::ROMol;

    #[test]
    fn make_sure_fingerprint_works() {
        let mol = ROMol::from_smile("CCC=O").unwrap();
        let fingerprint = mol.fingerprint();

        let mol_two = ROMol::from_smile("CCC=N").unwrap();
        let fingerprint_two = mol_two.fingerprint();

        let distance = fingerprint.tanimoto_distance(&fingerprint_two);

        assert_eq!(distance, 0.25);
    }
}
