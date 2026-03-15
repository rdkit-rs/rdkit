#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/fingerprint.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type ExplicitBitVect;
        pub fn rdk_fingerprint_mol(mol: &SharedPtr<ROMol>) -> SharedPtr<ExplicitBitVect>;
        pub fn pattern_fingerprint_mol(mol: &SharedPtr<ROMol>) -> SharedPtr<ExplicitBitVect>;
        pub fn morgan_fingerprint_mol(mol: &SharedPtr<ROMol>) -> SharedPtr<ExplicitBitVect>;

        pub fn copy_explicit_bit_vect(
            fingerprint: &SharedPtr<ExplicitBitVect>,
        ) -> SharedPtr<ExplicitBitVect>;

        pub fn explicit_bit_vect_to_u64_vec(
            bitvect: &SharedPtr<ExplicitBitVect>,
        ) -> UniquePtr<CxxVector<u64>>;

        // Configurable fingerprints
        pub fn morgan_fingerprint_mol_with_params(
            mol: &SharedPtr<ROMol>,
            radius: u32,
            n_bits: u32,
        ) -> SharedPtr<ExplicitBitVect>;
        pub fn rdk_fingerprint_mol_with_params(
            mol: &SharedPtr<ROMol>,
            min_path: u32,
            max_path: u32,
            fp_size: u32,
        ) -> SharedPtr<ExplicitBitVect>;
        pub fn pattern_fingerprint_mol_with_params(
            mol: &SharedPtr<ROMol>,
            fp_size: u32,
        ) -> SharedPtr<ExplicitBitVect>;
        pub fn maccs_fingerprint_mol(mol: &SharedPtr<ROMol>) -> SharedPtr<ExplicitBitVect>;
        pub fn explicit_bit_vect_num_bits(bitvect: &SharedPtr<ExplicitBitVect>) -> u32;
    }
}
