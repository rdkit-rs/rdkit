#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/rw_mol.h");

        pub type RWMol;
        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn rw_mol_from_mol_block(
            mol_block: &CxxString,
            sanitize: bool,
            remove_hs: bool,
            strict_parsing: bool,
        ) -> Result<UniquePtr<RWMol>>;

        pub fn rw_mol_from_ro_mol(
            mol: &ROMol,
            quick_copy: bool,
            conf_id: i32,
        ) -> Result<UniquePtr<RWMol>>;

        pub fn rw_mol_from_rw_mol(mol: &RWMol) -> Result<UniquePtr<RWMol>>;

        pub fn rw_mol_to_ro_mol(mol: UniquePtr<RWMol>) -> Result<UniquePtr<ROMol>>;

        pub fn rw_mol_as_ro_mol(mol: &RWMol) -> &ROMol;

        pub fn smarts_to_mol(smarts: &CxxString) -> Result<UniquePtr<RWMol>>;
        pub fn new_rw_mol() -> Result<UniquePtr<RWMol>>;
        pub fn rw_mol_as_ro_mol_mut(mol: Pin<&mut RWMol>) -> Pin<&mut ROMol>;
        pub fn rw_mol_add_atom(mol: Pin<&mut RWMol>, atomic_num: u32) -> Result<u32>;
        pub fn rw_mol_add_bond(
            mol: Pin<&mut RWMol>,
            begin: u32,
            end: u32,
            order: i32,
        ) -> Result<u32>;
        pub fn rw_mol_remove_atom(mol: Pin<&mut RWMol>, idx: u32) -> Result<()>;
        pub fn rw_mol_remove_bond(mol: Pin<&mut RWMol>, begin: u32, end: u32) -> Result<()>;
        pub fn rw_mol_sanitize(mol: Pin<&mut RWMol>) -> Result<()>;
    }
}
