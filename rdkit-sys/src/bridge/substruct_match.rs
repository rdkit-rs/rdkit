#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/substruct_match.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type SubstructMatchParameters;
        pub type SubstructMatchItem;
        pub type MatchVectType;

        pub fn substruct_match(
            mol: &ROMol,
            mol_query: &ROMol,
            params: &SubstructMatchParameters,
        ) -> Result<UniquePtr<CxxVector<MatchVectType>>>;

        pub fn new_substruct_match_parameters() -> Result<UniquePtr<SubstructMatchParameters>>;
        pub fn get_use_chirality(params: &SubstructMatchParameters) -> Result<bool>;
        pub fn get_use_enhanced_stereo(params: &SubstructMatchParameters) -> Result<bool>;
        pub fn get_aromatic_matches_conjugated(params: &SubstructMatchParameters) -> Result<bool>;
        pub fn get_use_query_query_matches(params: &SubstructMatchParameters) -> Result<bool>;
        pub fn get_use_generic_matchers(params: &SubstructMatchParameters) -> Result<bool>;
        pub fn get_recursion_possible(params: &SubstructMatchParameters) -> Result<bool>;
        pub fn get_uniquify(params: &SubstructMatchParameters) -> Result<bool>;

        pub fn set_use_chirality(
            params: Pin<&mut SubstructMatchParameters>,
            what: bool,
        ) -> Result<()>;
        pub fn set_use_enhanced_stereo(
            params: Pin<&mut SubstructMatchParameters>,
            what: bool,
        ) -> Result<()>;
        pub fn set_aromatic_matches_conjugated(
            params: Pin<&mut SubstructMatchParameters>,
            what: bool,
        ) -> Result<()>;
        pub fn set_use_query_query_matches(
            params: Pin<&mut SubstructMatchParameters>,
            what: bool,
        ) -> Result<()>;
        pub fn set_use_generic_matchers(
            params: Pin<&mut SubstructMatchParameters>,
            what: bool,
        ) -> Result<()>;
        pub fn set_recursion_possible(
            params: Pin<&mut SubstructMatchParameters>,
            what: bool,
        ) -> Result<()>;
        pub fn set_uniquify(params: Pin<&mut SubstructMatchParameters>, what: bool) -> Result<()>;

        pub fn substruct_matchvect_type_to_vec_substruct_match_item(
            matchvect: &MatchVectType,
        ) -> Result<UniquePtr<CxxVector<SubstructMatchItem>>>;

        pub fn substruct_match_item_query_atom_idx(
            substruct_match_item: &SubstructMatchItem,
        ) -> Result<i32>;
        pub fn substruct_match_item_mol_atom_idx(
            substruct_match_item: &SubstructMatchItem,
        ) -> Result<i32>;
    }
}
