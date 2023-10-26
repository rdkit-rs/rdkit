#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {

    unsafe extern "C++" {
        include!("wrapper/include/mol_ops.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub type RemoveHsParameters;
        pub fn new_remove_hs_parameters() -> SharedPtr<RemoveHsParameters>;

        pub fn get_remove_degree_zero(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_degree_zero(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_higher_degrees(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_higher_degrees(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_only_h_neighbors(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_only_h_neighbors(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_isotopes(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_isotopes(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_and_track_isotopes(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_and_track_isotopes(
            params: &mut SharedPtr<RemoveHsParameters>,
            what: bool,
        );

        pub fn get_remove_dummy_neighbors(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_dummy_neighbors(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_defining_bond_stereo(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_defining_bond_stereo(
            params: &mut SharedPtr<RemoveHsParameters>,
            what: bool,
        );

        pub fn get_remove_with_wedged_bond(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_with_wedged_bond(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_with_query(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_with_query(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_mapped(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_mapped(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_in_s_groups(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_in_s_groups(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_show_warnings(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_show_warnings(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_nonimplicit(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_nonimplicit(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_update_explicit_count(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_update_explicit_count(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_hydrides(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_hydrides(params: &mut SharedPtr<RemoveHsParameters>, what: bool);

        pub fn get_remove_nontetrahedral_neighbors(params: &SharedPtr<RemoveHsParameters>) -> bool;
        pub fn set_remove_nontetrahedral_neighbors(
            params: &mut SharedPtr<RemoveHsParameters>,
            what: bool,
        );

        pub fn remove_hs_parameters(
            mol: &SharedPtr<ROMol>,
            params: &SharedPtr<RemoveHsParameters>,
            sanitize: bool,
        ) -> SharedPtr<ROMol>;

        pub fn add_hs(
            mol: &SharedPtr<ROMol>,
            explicit_only: bool,
            add_coords: bool,
            add_residue_info: bool,
        ) -> SharedPtr<ROMol>;
    }
}
