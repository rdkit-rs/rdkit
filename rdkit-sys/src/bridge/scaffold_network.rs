#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/scaffold_network.h");

        pub type ScaffoldNetworkParams;
        //
        // pub fn default_scaffold_network_params() -> SharedPtr<ScaffoldNetworkParams>;

        pub fn default_scaffold_network_params() -> SharedPtr<ScaffoldNetworkParams>;

        pub fn new_scaffold_network_params(
            bond_breaker_smarts: &Vec<String>,
        ) -> SharedPtr<ScaffoldNetworkParams>;

        pub fn set_include_generic_scaffolds(
            params: &mut SharedPtr<ScaffoldNetworkParams>,
            input: bool,
        );
        pub fn include_generic_bond_scaffolds(
            params: &mut SharedPtr<ScaffoldNetworkParams>,
            input: bool,
        );
        pub fn include_scaffolds_without_attachments(
            params: &mut SharedPtr<ScaffoldNetworkParams>,
            input: bool,
        );
        pub fn include_scaffolds_with_attachments(
            params: &mut SharedPtr<ScaffoldNetworkParams>,
            input: bool,
        );
        pub fn keep_only_first_fragment(params: &mut SharedPtr<ScaffoldNetworkParams>, input: bool);
        pub fn prune_before_fragmenting(params: &mut SharedPtr<ScaffoldNetworkParams>, input: bool);
        pub fn flatten_isotopes(params: &mut SharedPtr<ScaffoldNetworkParams>, input: bool);
        pub fn flatten_chirality(params: &mut SharedPtr<ScaffoldNetworkParams>, input: bool);
        pub fn flatten_keep_largest(params: &mut SharedPtr<ScaffoldNetworkParams>, input: bool);
        pub fn collect_mol_counts(params: &mut SharedPtr<ScaffoldNetworkParams>, input: bool);

        pub type ScaffoldNetworkClass;
        pub fn default_scaffold_network() -> SharedPtr<ScaffoldNetworkClass>;

    }
}
