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
            mol: &SharedPtr<ROMol>,
            mol_query: &SharedPtr<ROMol>,
            params: &SharedPtr<SubstructMatchParameters>,
        ) -> UniquePtr<CxxVector<MatchVectType>>;

        pub fn new_substruct_match_parameters() -> SharedPtr<SubstructMatchParameters>;
        pub fn get_use_chirality(params: &SharedPtr<SubstructMatchParameters>) -> bool;
        pub fn get_use_enhanced_stereo(params: &SharedPtr<SubstructMatchParameters>) -> bool;
        pub fn get_aromatic_matches_conjugated(
            params: &SharedPtr<SubstructMatchParameters>,
        ) -> bool;
        pub fn get_use_query_query_matches(params: &SharedPtr<SubstructMatchParameters>) -> bool;
        pub fn get_use_generic_matchers(params: &SharedPtr<SubstructMatchParameters>) -> bool;
        pub fn get_recursion_possible(params: &SharedPtr<SubstructMatchParameters>) -> bool;
        pub fn get_uniquify(params: &SharedPtr<SubstructMatchParameters>) -> bool;

        pub fn set_use_chirality(params: &mut SharedPtr<SubstructMatchParameters>, what: bool);
        pub fn set_use_enhanced_stereo(
            params: &mut SharedPtr<SubstructMatchParameters>,
            what: bool,
        );
        pub fn set_aromatic_matches_conjugated(
            params: &mut SharedPtr<SubstructMatchParameters>,
            what: bool,
        );
        pub fn set_use_query_query_matches(
            params: &mut SharedPtr<SubstructMatchParameters>,
            what: bool,
        );
        pub fn set_use_generic_matchers(
            params: &mut SharedPtr<SubstructMatchParameters>,
            what: bool,
        );
        pub fn set_recursion_possible(params: &mut SharedPtr<SubstructMatchParameters>, what: bool);
        pub fn set_uniquify(params: &mut SharedPtr<SubstructMatchParameters>, what: bool);

        pub fn substruct_matchvect_type_to_vec_substruct_match_item(
            matchvect: &MatchVectType,
        ) -> UniquePtr<CxxVector<SubstructMatchItem>>;

        pub fn substruct_match_item_query_atom_idx(
            substruct_match_item: &SubstructMatchItem,
        ) -> i32;
        pub fn substruct_match_item_mol_atom_idx(substruct_match_item: &SubstructMatchItem) -> i32;
    }
}
