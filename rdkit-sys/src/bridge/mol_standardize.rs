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

        pub fn tautomer_enumerator() -> SharedPtr<TautomerEnumerator>;

        pub fn tautomer_enumerate(
            tautomer_enumerator: &SharedPtr<TautomerEnumerator>,
            mol: &SharedPtr<ROMol>,
        ) -> SharedPtr<TautomerEnumeratorResult>;
        pub fn tautomer_enumerator_result_tautomers_size(
            enumerator_result: &SharedPtr<TautomerEnumeratorResult>,
        ) -> i32;
        pub fn tautomer_enumerator_result_tautomers_at(
            enumerator_result: &SharedPtr<TautomerEnumeratorResult>,
            at: usize,
        ) -> SharedPtr<ROMol>;
        pub fn tautomer_enumerator_canonicalize(
            tautomer_enumerator: &SharedPtr<TautomerEnumerator>,
            mol: &SharedPtr<ROMol>,
        ) -> Result<SharedPtr<ROMol>>;

        pub fn default_cleanup_parameters() -> SharedPtr<CleanupParameters>;

        pub fn new_uncharger(canonical: bool) -> SharedPtr<Uncharger>;
        pub fn uncharger_uncharge(
            uncharger: &SharedPtr<Uncharger>,
            mol: &SharedPtr<ROMol>,
        ) -> SharedPtr<ROMol>;

        pub fn fragment_parent(
            rw_mol: &SharedPtr<RWMol>,
            cleanup_params: &SharedPtr<CleanupParameters>,
            skip_standardize: bool,
        ) -> SharedPtr<RWMol>;

        pub fn normalize(
            rw_mol: &SharedPtr<RWMol>,
            cleanup_params: &SharedPtr<CleanupParameters>,
        ) -> SharedPtr<RWMol>;
    }
}
