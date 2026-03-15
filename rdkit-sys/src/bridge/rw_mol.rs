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
        ) -> SharedPtr<RWMol>;

        pub fn rw_mol_from_ro_mol(
            mol: &SharedPtr<ROMol>,
            quick_copy: bool,
            conf_id: i32,
        ) -> SharedPtr<RWMol>;

        pub fn rw_mol_from_rw_mol(mol: &SharedPtr<RWMol>) -> SharedPtr<RWMol>;

        pub fn rw_mol_to_ro_mol(mol: SharedPtr<RWMol>) -> SharedPtr<ROMol>;

        pub fn smarts_to_mol(smarts: &CxxString) -> Result<SharedPtr<RWMol>>;

        // Molecule editing
        pub fn new_rw_mol() -> SharedPtr<RWMol>;
        pub fn rw_mol_add_atom(mol: &mut SharedPtr<RWMol>, atomic_num: u32) -> u32;
        pub fn rw_mol_add_bond(
            mol: &mut SharedPtr<RWMol>,
            begin_idx: u32,
            end_idx: u32,
            bond_order: i32,
        ) -> u32;
        pub fn rw_mol_remove_atom(mol: &mut SharedPtr<RWMol>, idx: u32);
        pub fn rw_mol_remove_bond(mol: &mut SharedPtr<RWMol>, begin_idx: u32, end_idx: u32);
        pub fn rw_mol_get_num_atoms(mol: &SharedPtr<RWMol>, only_explicit: bool) -> u32;
    }
}
