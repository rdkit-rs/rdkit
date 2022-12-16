use bitvec::prelude::*;
use cxx::SharedPtr;

#[derive(Clone, Debug)]
pub struct Fingerprint(pub BitVec<u64, bitvec::order::Lsb0>);

impl Fingerprint {
    pub fn new(ptr: SharedPtr<rdkit_sys::fingerprint_ffi::ExplicitBitVect>) -> Self {
        let unique_ptr_bytes = rdkit_sys::fingerprint_ffi::explicit_bit_vect_to_u64_vec(ptr);
        let rust_bytes: Vec<_> = unique_ptr_bytes.into_iter().map(|x| *x).collect();
        let bits = bitvec::vec::BitVec::<_, Lsb0>::from_vec(rust_bytes);
        Fingerprint(bits)
    }

    // pub fn as_bitvec_u8(&self) -> BitVec<u8, bitvec::order::Lsb0> {
    //     self.0.iter().map(|big_word| )
    // }

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
    use std::io::Cursor;

    use bitvec::prelude::*;
    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

    #[test]
    fn convert_bitvect_u64_to_u8() {
        // 283686952306183
        let bytes: [u8; 8] = [
            0b00000000, 0b00000001, 0b00000010, 0b00000011, 0b00000100, 0b00000101, 0b00000110,
            0b00000111,
        ];
        let mut byte_reader = Cursor::new(&bytes[..]);
        let bignum = byte_reader.read_u64::<BigEndian>().unwrap();

        let bitvect_of_bignum: BitVec<u64, Lsb0> = BitVec::from_vec(vec![bignum]);

        let vec_of_smallnum: Vec<u8> = bitvect_of_bignum
            .iter()
            .map(|bignum| {
                let mut bytes = vec![];
                bytes.write_u64(bignum);
                bytes
            })
            .flatten()
            .collect();
        let bitvect_of_smallnum: BitVec<u8, Lsb0> = BitVec::from_vec(vec_of_smallnum);

        panic!("{}", bignum);
    }
}
