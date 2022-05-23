use bitvec::prelude::*;

#[derive(Clone, Debug)]
pub struct Fingerprint(pub BitVec<u8, bitvec::order::Lsb0>);

impl Fingerprint {
    pub fn tanimoto_distance(&self, other: &Fingerprint) -> f32 {
        let and = self.0.clone() & &other.0;
        let or = self.0.clone() | &other.0;

        let and_ones = and.count_ones();
        let or_ones = or.count_ones();

        and_ones as f32 / or_ones as f32
    }
}
