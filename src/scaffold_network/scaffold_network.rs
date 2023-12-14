use cxx::SharedPtr;

pub struct ScaffoldNetwork {
    ptr: SharedPtr<rdkit_sys::scaffold_network_ffi::ScaffoldNetworkClass>,
}

impl Default for ScaffoldNetwork {
    fn default() -> Self {
        Self {
            ptr: rdkit_sys::scaffold_network_ffi::default_scaffold_network(),
        }
    }
}

impl ScaffoldNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
