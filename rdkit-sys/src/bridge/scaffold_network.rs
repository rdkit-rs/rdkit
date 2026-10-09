#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/scaffold_network.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type ScaffoldNetworkParams;

        pub fn default_scaffold_network_params() -> Result<UniquePtr<ScaffoldNetworkParams>>;

        pub fn new_scaffold_network_params(
            bond_breaker_smarts: &Vec<String>,
        ) -> Result<UniquePtr<ScaffoldNetworkParams>>;

        pub fn set_include_generic_scaffolds(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;
        pub fn include_generic_bond_scaffolds(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;
        pub fn include_scaffolds_without_attachments(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;
        pub fn include_scaffolds_with_attachments(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;
        pub fn keep_only_first_fragment(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;
        pub fn prune_before_fragmenting(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;
        pub fn flatten_isotopes(params: Pin<&mut ScaffoldNetworkParams>, input: bool)
            -> Result<()>;
        pub fn flatten_chirality(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;
        pub fn flatten_keep_largest(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;
        pub fn collect_mol_counts(
            params: Pin<&mut ScaffoldNetworkParams>,
            input: bool,
        ) -> Result<()>;

        pub type ScaffoldNetworkClass;
        pub fn default_scaffold_network() -> Result<UniquePtr<ScaffoldNetworkClass>>;

        pub fn create_scaffold_network(
            mol: &ROMol,
            params: &ScaffoldNetworkParams,
        ) -> Result<UniquePtr<ScaffoldNetworkClass>>;

        pub fn update_scaffold_network(
            network: Pin<&mut ScaffoldNetworkClass>,
            mol: &ROMol,
            params: &ScaffoldNetworkParams,
        ) -> Result<()>;

        pub fn scaffold_network_nodes(network: &ScaffoldNetworkClass) -> Result<Vec<String>>;
        pub fn scaffold_network_counts(network: &ScaffoldNetworkClass) -> Result<Vec<u32>>;
        pub fn scaffold_network_mol_counts(network: &ScaffoldNetworkClass) -> Result<Vec<u32>>;
        pub fn scaffold_network_edge_begin_indices(
            network: &ScaffoldNetworkClass,
        ) -> Result<Vec<usize>>;
        pub fn scaffold_network_edge_end_indices(
            network: &ScaffoldNetworkClass,
        ) -> Result<Vec<usize>>;
        pub fn scaffold_network_edge_types(network: &ScaffoldNetworkClass) -> Result<Vec<String>>;
    }
}
