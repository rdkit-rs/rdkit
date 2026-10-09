#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/fingerprint.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type ExplicitBitVect;
        pub fn rdk_fingerprint_mol(mol: &ROMol) -> Result<UniquePtr<ExplicitBitVect>>;
        pub fn pattern_fingerprint_mol(mol: &ROMol) -> Result<UniquePtr<ExplicitBitVect>>;
        pub fn morgan_fingerprint_mol(mol: &ROMol) -> Result<UniquePtr<ExplicitBitVect>>;

        pub fn copy_explicit_bit_vect(
            fingerprint: &ExplicitBitVect,
        ) -> Result<UniquePtr<ExplicitBitVect>>;

        pub fn explicit_bit_vect_to_u64_vec(
            bitvect: &ExplicitBitVect,
        ) -> Result<UniquePtr<CxxVector<u64>>>;
        pub fn explicit_bit_vect_num_bits(bits: &ExplicitBitVect) -> Result<u32>;
        pub fn morgan_fingerprint_mol_with_params(
            mol: &ROMol,
            radius: u32,
            n_bits: u32,
        ) -> Result<UniquePtr<ExplicitBitVect>>;
        pub fn rdk_fingerprint_mol_with_params(
            mol: &ROMol,
            min_path: u32,
            max_path: u32,
            n_bits: u32,
        ) -> Result<UniquePtr<ExplicitBitVect>>;
        pub fn pattern_fingerprint_mol_with_params(
            mol: &ROMol,
            n_bits: u32,
        ) -> Result<UniquePtr<ExplicitBitVect>>;
        pub fn maccs_fingerprint_mol(mol: &ROMol) -> Result<UniquePtr<ExplicitBitVect>>;
    }
}
