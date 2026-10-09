#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/mol_standardize.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type RWMol = crate::rw_mol_ffi::RWMol;

        pub type TautomerEnumerator;
        pub type TautomerEnumeratorResult;
        pub type CleanupParameters;
        pub type Uncharger;

        pub fn tautomer_enumerator() -> Result<UniquePtr<TautomerEnumerator>>;

        pub fn tautomer_enumerate(
            tautomer_enumerator: &TautomerEnumerator,
            mol: &ROMol,
        ) -> Result<UniquePtr<TautomerEnumeratorResult>>;
        pub fn tautomer_enumerator_result_tautomers_size(
            enumerator_result: &TautomerEnumeratorResult,
        ) -> Result<usize>;
        pub fn tautomer_enumerator_result_tautomers_at(
            enumerator_result: &TautomerEnumeratorResult,
            at: usize,
        ) -> Result<UniquePtr<ROMol>>;
        pub fn tautomer_enumerator_canonicalize(
            tautomer_enumerator: &TautomerEnumerator,
            mol: &ROMol,
        ) -> Result<UniquePtr<ROMol>>;

        pub fn default_cleanup_parameters() -> Result<UniquePtr<CleanupParameters>>;

        pub fn new_uncharger(canonical: bool) -> Result<UniquePtr<Uncharger>>;
        pub fn uncharger_uncharge(
            uncharger: Pin<&mut Uncharger>,
            mol: &ROMol,
        ) -> Result<UniquePtr<ROMol>>;

        pub fn fragment_parent(
            rw_mol: &RWMol,
            cleanup_params: &CleanupParameters,
            skip_standardize: bool,
        ) -> Result<UniquePtr<RWMol>>;

        pub fn normalize(
            rw_mol: &RWMol,
            cleanup_params: &CleanupParameters,
        ) -> Result<UniquePtr<RWMol>>;
    }
}
