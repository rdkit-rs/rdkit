#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    // Trick from https://github.com/dtolnay/cxx/issues/741
    // pub struct ROMolSharedPtr {
    //     ptr: SharedPtr<ROMol>,
    // }

    unsafe extern "C++" {
        include!("wrapper/include/scaffold_network.h");

        pub type ScaffoldNetworkParamsLocal;
        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn default_scaffold_network_params() -> SharedPtr<ScaffoldNetworkParamsLocal>;

        pub fn new_scaffold_network_params(
            bond_breaker_smarts: &Vec<String>,
        ) -> SharedPtr<ScaffoldNetworkParamsLocal>;

        pub fn set_include_generic_scaffolds(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_include_generic_bond_scaffolds(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_include_scaffolds_without_attachments(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_include_scaffolds_with_attachments(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_keep_only_first_fragment(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_prune_before_fragmenting(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_flatten_isotopes(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_flatten_chirality(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_flatten_keep_largest(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );
        pub fn set_collect_mol_counts(
            params: &mut SharedPtr<ScaffoldNetworkParamsLocal>,
            input: bool,
        );

        pub fn get_include_generic_scaffolds(
            params: &SharedPtr<ScaffoldNetworkParamsLocal>,
        ) -> bool;
        pub fn get_include_generic_bond_scaffolds(
            params: &SharedPtr<ScaffoldNetworkParamsLocal>,
        ) -> bool;
        pub fn get_include_scaffolds_without_attachments(
            params: &SharedPtr<ScaffoldNetworkParamsLocal>,
        ) -> bool;
        pub fn get_include_scaffolds_with_attachments(
            params: &SharedPtr<ScaffoldNetworkParamsLocal>,
        ) -> bool;
        pub fn get_keep_only_first_fragment(params: &SharedPtr<ScaffoldNetworkParamsLocal>)
            -> bool;
        pub fn get_prune_before_fragmenting(params: &SharedPtr<ScaffoldNetworkParamsLocal>)
            -> bool;
        pub fn get_flatten_isotopes(params: &SharedPtr<ScaffoldNetworkParamsLocal>) -> bool;
        pub fn get_flatten_chirality(params: &SharedPtr<ScaffoldNetworkParamsLocal>) -> bool;
        pub fn get_flatten_keep_largest(params: &SharedPtr<ScaffoldNetworkParamsLocal>) -> bool;
        pub fn get_collect_mol_counts(params: &SharedPtr<ScaffoldNetworkParamsLocal>) -> bool;

        pub type ScaffoldNetworkClass;
        pub fn default_scaffold_network() -> SharedPtr<ScaffoldNetworkClass>;

        // pub fn create_scaffold_network(
        //     mol: &CxxVector<ROMolSharedPtr>,
        //     scaffold_network_params: &SharedPtr<ScaffoldNetworkParamsLocal>,
        // ) -> SharedPtr<ScaffoldNetworkClass>;

        // pub fn update_scaffold_network(
        //     scaffold_network: &mut SharedPtr<ScaffoldNetworkClass>,
        //     mol: &CxxVector<ROMolSharedPtr>,
        //     scaffold_network_params: &SharedPtr<ScaffoldNetworkParamsLocal>,
        // );
    }
}
