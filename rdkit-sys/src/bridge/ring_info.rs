#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ring_info.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn mol_num_rings(mol: &SharedPtr<ROMol>) -> u32;
        pub fn mol_is_atom_in_ring_of_size(
            mol: &SharedPtr<ROMol>,
            atom_idx: u32,
            size: u32,
        ) -> bool;
        pub fn mol_is_bond_in_ring_of_size(
            mol: &SharedPtr<ROMol>,
            bond_idx: u32,
            size: u32,
        ) -> bool;
        pub fn mol_num_atom_rings(mol: &SharedPtr<ROMol>, atom_idx: u32) -> u32;
        pub fn mol_num_bond_rings(mol: &SharedPtr<ROMol>, bond_idx: u32) -> u32;
        pub fn mol_atom_ring_sizes(
            mol: &SharedPtr<ROMol>,
            atom_idx: u32,
        ) -> UniquePtr<CxxVector<i32>>;
        pub fn mol_bond_ring_sizes(
            mol: &SharedPtr<ROMol>,
            bond_idx: u32,
        ) -> UniquePtr<CxxVector<i32>>;
    }
}
