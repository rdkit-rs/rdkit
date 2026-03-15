#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    #[repr(i32)]
    #[derive(Debug, PartialEq)]
    pub enum BondType {
        UNSPECIFIED,
        SINGLE,
        DOUBLE,
        TRIPLE,
        QUADRUPLE,
        QUINTUPLE,
        HEXTUPLE,
        ONEANDAHALF,
        TWOANDAHALF,
        THREEANDAHALF,
        FOURANDAHALF,
        FIVEANDAHALF,
        AROMATIC,
        IONIC,
        HYDROGEN,
        THREECENTER,
        DATIVEONE,
        DATIVE,
        DATIVEL,
        DATIVER,
        OTHER,
        ZERO,
    }

    #[repr(i32)]
    #[derive(Debug, PartialEq)]
    pub enum BondStereo {
        STEREONONE,
        STEREOANY,
        STEREOZ,
        STEREOE,
        STEREOCIS,
        STEREOTRANS,
    }

    #[repr(i32)]
    #[derive(Debug, PartialEq)]
    pub enum BondDir {
        NONE,
        BEGINWEDGE,
        BEGINDASH,
        ENDDOWNRIGHT,
        ENDUPRIGHT,
        EITHERDOUBLE,
        UNKNOWN,
    }

    unsafe extern "C++" {
        include!("wrapper/include/bond.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type Bond;
        pub type BondType;
        pub type BondStereo;
        pub type BondDir;

        pub fn get_num_bonds(mol: &SharedPtr<ROMol>, only_heavy: bool) -> u32;
        pub fn get_bond_with_idx(mol: &mut SharedPtr<ROMol>, idx: u32) -> Pin<&mut Bond>;
        pub fn get_bond_idx_between_atoms(
            mol: &SharedPtr<ROMol>,
            begin_idx: u32,
            end_idx: u32,
        ) -> i32;

        pub fn bond_get_bond_type(bond: Pin<&Bond>) -> BondType;
        pub fn bond_get_bond_type_as_double(bond: Pin<&Bond>) -> f64;
        pub fn bond_get_begin_atom_idx(bond: Pin<&Bond>) -> u32;
        pub fn bond_get_end_atom_idx(bond: Pin<&Bond>) -> u32;
        pub fn bond_get_other_atom_idx(bond: Pin<&Bond>, this_idx: u32) -> u32;
        pub fn bond_get_is_aromatic(bond: Pin<&Bond>) -> bool;
        pub fn bond_get_is_conjugated(bond: Pin<&Bond>) -> bool;
        pub fn bond_get_stereo(bond: Pin<&Bond>) -> BondStereo;
        pub fn bond_get_bond_dir(bond: Pin<&Bond>) -> BondDir;
        pub fn bond_get_idx(bond: Pin<&Bond>) -> u32;
    }
}
