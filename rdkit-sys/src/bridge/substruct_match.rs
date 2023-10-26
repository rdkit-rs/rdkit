#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/substruct_match.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type SubstructMatchParameters;
        pub type MatchVectType;

        pub fn substruct_match(
            mol: &SharedPtr<ROMol>,
            mol_query: &SharedPtr<ROMol>,
            params: &SharedPtr<SubstructMatchParameters>,
        ) -> UniquePtr<CxxVector<MatchVectType>>;

        pub fn substruct_match_as_bool(
            mol: &SharedPtr<ROMol>,
            mol_query: &SharedPtr<ROMol>,
            params: &SharedPtr<SubstructMatchParameters>,
        ) -> bool;

        pub fn new_substruct_match_parameters() -> SharedPtr<SubstructMatchParameters>;
    }
}
