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
        STEREOATROPCW,
        STEREOATROPCCW,
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

        pub fn get_num_bonds(mol: &ROMol, only_heavy: bool) -> Result<u32>;
        pub fn get_bond_with_idx(mol: &ROMol, idx: u32) -> Result<&Bond>;
        pub fn get_bond_idx_between_atoms(mol: &ROMol, begin_idx: u32, end_idx: u32)
            -> Result<i32>;

        pub fn bond_get_bond_type(bond: &Bond) -> Result<BondType>;
        pub fn bond_get_bond_type_as_double(bond: &Bond) -> Result<f64>;
        pub fn bond_get_begin_atom_idx(bond: &Bond) -> Result<u32>;
        pub fn bond_get_end_atom_idx(bond: &Bond) -> Result<u32>;
        pub fn bond_get_other_atom_idx(bond: &Bond, this_idx: u32) -> Result<u32>;
        pub fn bond_get_is_aromatic(bond: &Bond) -> Result<bool>;
        pub fn bond_get_is_conjugated(bond: &Bond) -> Result<bool>;
        pub fn bond_get_stereo(bond: &Bond) -> Result<BondStereo>;
        pub fn bond_get_bond_dir(bond: &Bond) -> Result<BondDir>;
        pub fn bond_get_idx(bond: &Bond) -> Result<u32>;
    }
}
