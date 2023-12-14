use cxx::SharedPtr;

pub struct ScaffoldNetworkParams {
    ptr: SharedPtr<rdkit_sys::scaffold_network_ffi::ScaffoldNetworkParamsLocal>,
}

impl Default for ScaffoldNetworkParams {
    fn default() -> Self {
        let ptr = rdkit_sys::scaffold_network_ffi::default_scaffold_network_params();
        Self { ptr }
    }
}

impl ScaffoldNetworkParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_bond_breakers(bond_breaker_smarts: &Vec<String>) -> Self {
        let ptr = rdkit_sys::scaffold_network_ffi::new_scaffold_network_params(bond_breaker_smarts);
        Self { ptr }
    }

    pub fn set_include_generic_scaffolds(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_include_generic_scaffolds(&mut self.ptr, input);
    }

    pub fn set_include_generic_bond_scaffolds(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_include_generic_bond_scaffolds(&mut self.ptr, input);
    }

    pub fn set_include_scaffolds_without_attachments(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_include_scaffolds_without_attachments(
            &mut self.ptr,
            input,
        );
    }

    pub fn set_include_scaffolds_with_attachments(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_include_scaffolds_with_attachments(
            &mut self.ptr,
            input,
        );
    }

    pub fn set_keep_only_first_fragment(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_keep_only_first_fragment(&mut self.ptr, input);
    }

    pub fn set_prune_before_fragmenting(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_prune_before_fragmenting(&mut self.ptr, input);
    }

    pub fn set_flatten_isotopes(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_flatten_isotopes(&mut self.ptr, input);
    }

    pub fn set_flatten_chirality(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_flatten_chirality(&mut self.ptr, input);
    }

    pub fn set_flatten_keep_largest(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_flatten_keep_largest(&mut self.ptr, input);
    }

    pub fn set_collect_mol_counts(&mut self, input: bool) {
        rdkit_sys::scaffold_network_ffi::set_collect_mol_counts(&mut self.ptr, input);
    }

    pub fn get_include_generic_scaffolds(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_include_generic_scaffolds(&self.ptr)
    }

    pub fn get_include_generic_bond_scaffolds(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_include_generic_bond_scaffolds(&self.ptr)
    }

    pub fn get_include_scaffolds_without_attachments(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_include_scaffolds_without_attachments(&self.ptr)
    }

    pub fn get_include_scaffolds_with_attachments(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_include_scaffolds_with_attachments(&self.ptr)
    }

    pub fn get_keep_only_first_fragment(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_keep_only_first_fragment(&self.ptr)
    }

    pub fn get_prune_before_fragmenting(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_prune_before_fragmenting(&self.ptr)
    }

    pub fn get_flatten_isotopes(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_flatten_isotopes(&self.ptr)
    }

    pub fn get_flatten_chirality(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_flatten_chirality(&self.ptr)
    }

    pub fn get_flatten_keep_largest(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_flatten_keep_largest(&self.ptr)
    }

    pub fn get_collect_mol_counts(&self) -> bool {
        rdkit_sys::scaffold_network_ffi::get_collect_mol_counts(&self.ptr)
    }
}
