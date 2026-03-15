use crate::ROMol;

impl ROMol {
    pub fn num_rings(&self) -> u32 {
        rdkit_sys::ring_info_ffi::mol_num_rings(&self.ptr)
    }

    pub fn is_atom_in_ring_of_size(&self, atom_idx: u32, size: u32) -> bool {
        rdkit_sys::ring_info_ffi::mol_is_atom_in_ring_of_size(&self.ptr, atom_idx, size)
    }

    pub fn is_bond_in_ring_of_size(&self, bond_idx: u32, size: u32) -> bool {
        rdkit_sys::ring_info_ffi::mol_is_bond_in_ring_of_size(&self.ptr, bond_idx, size)
    }

    pub fn num_atom_rings(&self, atom_idx: u32) -> u32 {
        rdkit_sys::ring_info_ffi::mol_num_atom_rings(&self.ptr, atom_idx)
    }

    pub fn num_bond_rings(&self, bond_idx: u32) -> u32 {
        rdkit_sys::ring_info_ffi::mol_num_bond_rings(&self.ptr, bond_idx)
    }

    pub fn atom_ring_sizes(&self, atom_idx: u32) -> Vec<i32> {
        let sizes = rdkit_sys::ring_info_ffi::mol_atom_ring_sizes(&self.ptr, atom_idx);
        sizes.iter().copied().collect()
    }

    pub fn bond_ring_sizes(&self, bond_idx: u32) -> Vec<i32> {
        let sizes = rdkit_sys::ring_info_ffi::mol_bond_ring_sizes(&self.ptr, bond_idx);
        sizes.iter().copied().collect()
    }
}
