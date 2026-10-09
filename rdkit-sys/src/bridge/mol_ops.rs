#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {

    unsafe extern "C++" {
        include!("wrapper/include/mol_ops.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type RWMol = crate::rw_mol_ffi::RWMol;

        pub type RemoveHsParameters;
        pub fn new_remove_hs_parameters() -> Result<UniquePtr<RemoveHsParameters>>;

        pub fn get_remove_degree_zero(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_degree_zero(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_higher_degrees(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_higher_degrees(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_only_h_neighbors(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_only_h_neighbors(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_isotopes(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_isotopes(params: Pin<&mut RemoveHsParameters>, what: bool) -> Result<()>;

        pub fn get_remove_and_track_isotopes(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_and_track_isotopes(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_dummy_neighbors(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_dummy_neighbors(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_defining_bond_stereo(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_defining_bond_stereo(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_with_wedged_bond(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_with_wedged_bond(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_with_query(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_with_query(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_mapped(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_mapped(params: Pin<&mut RemoveHsParameters>, what: bool) -> Result<()>;

        pub fn get_remove_in_s_groups(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_in_s_groups(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_show_warnings(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_show_warnings(params: Pin<&mut RemoveHsParameters>, what: bool) -> Result<()>;

        pub fn get_remove_nonimplicit(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_nonimplicit(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_update_explicit_count(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_update_explicit_count(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn get_remove_hydrides(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_hydrides(params: Pin<&mut RemoveHsParameters>, what: bool) -> Result<()>;

        pub fn get_remove_nontetrahedral_neighbors(params: &RemoveHsParameters) -> Result<bool>;
        pub fn set_remove_nontetrahedral_neighbors(
            params: Pin<&mut RemoveHsParameters>,
            what: bool,
        ) -> Result<()>;

        pub fn remove_hs_parameters(
            mol: &ROMol,
            params: &RemoveHsParameters,
            sanitize: bool,
        ) -> Result<UniquePtr<ROMol>>;

        pub fn add_hs(
            mol: &ROMol,
            explicit_only: bool,
            add_coords: bool,
            add_residue_info: bool,
        ) -> Result<UniquePtr<ROMol>>;

        pub fn romol_set_hybridization(mol: Pin<&mut ROMol>) -> Result<()>;

        pub fn clean_up(rw_mol: Pin<&mut RWMol>) -> Result<()>;
    }
}
