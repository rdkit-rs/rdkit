#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    #[repr(i32)]
    #[derive(Debug, PartialEq)]
    pub enum HybridizationType {
        UNSPECIFIED,
        S,
        SP,
        SP2,
        SP3,
        SP2D,
        SP3D,
        SP3D2,
        OTHER,
    }

    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");

        pub type ROMol;
        pub type ExplicitBitVect = crate::fingerprint_ffi::ExplicitBitVect;
        pub type SmilesParserParams;
        pub type Atom;
        pub type HybridizationType;

        pub type MolSanitizeException;
        pub type MolSanitizeExceptionUniquePtr; //  = UniquePtr<MolSanitizeException>;

        pub fn copy_mol(mol: &SharedPtr<ROMol>) -> SharedPtr<ROMol>;

        pub fn smiles_to_mol(smi: &CxxString) -> Result<SharedPtr<ROMol>>;

        pub fn smiles_to_mol_with_params(
            smi: &CxxString,
            params: &SharedPtr<SmilesParserParams>,
        ) -> Result<SharedPtr<ROMol>>;
        pub fn new_smiles_parser_params() -> SharedPtr<SmilesParserParams>;
        pub fn smiles_parser_params_set_sanitize(
            ptr: &SharedPtr<SmilesParserParams>,
            sanitize: bool,
        );
        pub fn smiles_parser_params_get_sanitize(ptr: &SharedPtr<SmilesParserParams>) -> bool;

        pub fn mol_to_smiles(mol: &SharedPtr<ROMol>) -> String;

        pub fn mol_to_molblock(mol: &SharedPtr<ROMol>) -> String;

        pub fn detect_chemistry_problems(
            mol: &SharedPtr<ROMol>,
        ) -> UniquePtr<CxxVector<MolSanitizeExceptionUniquePtr>>;

        pub fn mol_sanitize_exception_type(
            mol_sanitize_exception: &MolSanitizeExceptionUniquePtr,
        ) -> String;

        pub fn atom_sanitize_exception_get_atom_idx(
            mol_sanitize_exception: &MolSanitizeExceptionUniquePtr,
        ) -> u32;

        pub fn get_num_atoms(mol: &SharedPtr<ROMol>, onlyExplicit: bool) -> u32;
        pub fn get_atom_with_idx(mol: &mut SharedPtr<ROMol>, idx: u32) -> Pin<&mut Atom>;
        pub fn get_symbol(atom: Pin<&Atom>) -> String;
        pub fn get_is_aromatic(atom: Pin<&Atom>) -> bool;
        pub fn get_atomic_num(atom: Pin<&Atom>) -> i32;
        pub fn get_formal_charge(atom: Pin<&Atom>) -> i32;
        pub fn get_total_num_hs(atom: Pin<&Atom>) -> u32;
        pub fn get_total_valence(atom: Pin<&Atom>) -> u32;
        pub fn set_formal_charge(atom: Pin<&mut Atom>, what: i32);
        pub fn set_num_explicit_hs(atom: Pin<&mut Atom>, what: i32);
        pub fn atom_update_property_cache(atom: Pin<&mut Atom>, strict: bool) -> Result<()>;
        pub fn atom_set_hybridization(atom: Pin<&mut Atom>, what: HybridizationType);
        pub fn atom_get_hybridization(atom: Pin<&Atom>) -> HybridizationType;

        pub fn ro_mol_update_property_cache(atom: &mut SharedPtr<ROMol>, strict: bool);

        pub fn set_int_prop(atom: Pin<&mut Atom>, key: &CxxString, value: i32);
        pub fn get_int_prop(atom: Pin<&Atom>, key: &CxxString) -> Result<i32>;
        pub fn set_float_prop(atom: Pin<&mut Atom>, key: &CxxString, value: f64);
        pub fn get_float_prop(atom: Pin<&Atom>, key: &CxxString) -> Result<f64>;
        pub fn set_bool_prop(atom: Pin<&mut Atom>, key: &CxxString, value: bool);
        pub fn get_bool_prop(atom: Pin<&Atom>, key: &CxxString) -> Result<bool>;
        pub fn set_prop(atom: Pin<&mut Atom>, key: &CxxString, value: &CxxString);
        pub fn get_prop(atom: Pin<&Atom>, key: &CxxString) -> Result<String>;
        pub fn get_num_radical_electrons(atom: Pin<&Atom>) -> u32;

        //Returns the degree of the atom in the molecule.
        pub fn get_degree(atom: Pin<&Atom>) -> u32;
    }
}
