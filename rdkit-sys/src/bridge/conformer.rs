#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/conformer.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn embed_molecule(mol: &mut SharedPtr<ROMol>) -> i32;
        pub fn embed_multiple_confs(
            mol: &mut SharedPtr<ROMol>,
            num_confs: u32,
        ) -> UniquePtr<CxxVector<i32>>;
        pub fn compute_2d_coords(mol: &mut SharedPtr<ROMol>) -> u32;

        pub fn mol_get_num_conformers(mol: &SharedPtr<ROMol>) -> u32;
        pub fn conformer_is_3d(mol: &SharedPtr<ROMol>, conf_id: i32) -> bool;
        pub fn get_atom_pos(
            mol: &SharedPtr<ROMol>,
            conf_id: i32,
            atom_idx: u32,
        ) -> UniquePtr<CxxVector<f64>>;
    }
}
