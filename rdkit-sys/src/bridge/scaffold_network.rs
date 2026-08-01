#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/scaffold_network.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
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

        pub fn create_scaffold_network(
            mol: &SharedPtr<ROMol>,
            params: &SharedPtr<ScaffoldNetworkParams>,
        ) -> Result<SharedPtr<ScaffoldNetworkClass>>;

        pub fn update_scaffold_network(
            network: &mut SharedPtr<ScaffoldNetworkClass>,
            mol: &SharedPtr<ROMol>,
            params: &SharedPtr<ScaffoldNetworkParams>,
        ) -> Result<()>;

        pub fn scaffold_network_nodes(network: &SharedPtr<ScaffoldNetworkClass>) -> Vec<String>;
        pub fn scaffold_network_counts(network: &SharedPtr<ScaffoldNetworkClass>) -> Vec<u32>;
        pub fn scaffold_network_mol_counts(network: &SharedPtr<ScaffoldNetworkClass>) -> Vec<u32>;
        pub fn scaffold_network_edge_begin_indices(
            network: &SharedPtr<ScaffoldNetworkClass>,
        ) -> Vec<usize>;
        pub fn scaffold_network_edge_end_indices(
            network: &SharedPtr<ScaffoldNetworkClass>,
        ) -> Vec<usize>;
        pub fn scaffold_network_edge_types(
            network: &SharedPtr<ScaffoldNetworkClass>,
        ) -> Vec<String>;
    }
}
