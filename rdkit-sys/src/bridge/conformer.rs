#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Point3D {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    unsafe extern "C++" {
        include!("wrapper/include/conformer.h");
        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type Conformer;
        pub fn compute_2d_coords(mol: Pin<&mut ROMol>) -> Result<u32>;
        pub fn get_num_conformers(mol: &ROMol) -> Result<u32>;
        pub fn get_conformer(mol: &ROMol, id: i32) -> Result<&Conformer>;
        pub fn get_conformer_mut(mol: Pin<&mut ROMol>, id: i32) -> Result<Pin<&mut Conformer>>;
        pub fn conformer_id(conf: &Conformer) -> Result<u32>;
        pub fn conformer_is_3d(conf: &Conformer) -> Result<bool>;
        pub fn conformer_num_atoms(conf: &Conformer) -> Result<u32>;
        pub fn conformer_position(conf: &Conformer, idx: u32) -> Result<Point3D>;
        pub fn conformer_set_position(
            conf: Pin<&mut Conformer>,
            idx: u32,
            pos: Point3D,
        ) -> Result<()>;
    }
}
