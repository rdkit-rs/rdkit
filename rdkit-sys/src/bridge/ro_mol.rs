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

        pub type MolSanitizeException;
        pub type MolSanitizeExceptionUniquePtr; //  = UniquePtr<MolSanitizeException>;

        pub fn copy_mol(mol: &ROMol) -> Result<UniquePtr<ROMol>>;

        pub fn smiles_to_mol(smi: &CxxString) -> Result<UniquePtr<ROMol>>;

        pub fn smiles_to_mol_with_params(
            smi: &CxxString,
            params: &SmilesParserParams,
        ) -> Result<UniquePtr<ROMol>>;
        pub fn new_smiles_parser_params() -> Result<UniquePtr<SmilesParserParams>>;
        pub fn smiles_parser_params_set_sanitize(
            ptr: Pin<&mut SmilesParserParams>,
            sanitize: bool,
        ) -> Result<()>;
        pub fn smiles_parser_params_get_sanitize(ptr: &SmilesParserParams) -> Result<bool>;

        pub fn mol_to_smiles(mol: &ROMol) -> Result<String>;

        pub fn mol_to_molblock(mol: &ROMol) -> Result<String>;

        pub fn detect_chemistry_problems(
            mol: &ROMol,
        ) -> Result<UniquePtr<CxxVector<MolSanitizeExceptionUniquePtr>>>;

        pub fn mol_sanitize_exception_type(
            mol_sanitize_exception: &MolSanitizeExceptionUniquePtr,
        ) -> Result<String>;

        pub fn atom_sanitize_exception_get_atom_idx(
            mol_sanitize_exception: &MolSanitizeExceptionUniquePtr,
        ) -> Result<u32>;

        pub fn get_num_atoms(mol: &ROMol, onlyExplicit: bool) -> Result<u32>;
        pub fn get_atom_ref(mol: &ROMol, idx: u32) -> Result<&Atom>;
        pub fn get_atom_with_idx(mol: Pin<&mut ROMol>, idx: u32) -> Result<Pin<&mut Atom>>;
        pub fn get_symbol(atom: &Atom) -> Result<String>;
        pub fn get_is_aromatic(atom: &Atom) -> Result<bool>;
        pub fn get_atomic_num(atom: &Atom) -> Result<i32>;
        pub fn get_formal_charge(atom: &Atom) -> Result<i32>;
        pub fn get_total_num_hs(atom: &Atom) -> Result<u32>;
        pub fn get_total_valence(atom: &Atom) -> Result<u32>;
        pub fn set_formal_charge(atom: Pin<&mut Atom>, what: i32) -> Result<()>;
        pub fn set_num_explicit_hs(atom: Pin<&mut Atom>, what: i32) -> Result<()>;
        pub fn atom_update_property_cache(atom: Pin<&mut Atom>, strict: bool) -> Result<()>;
        pub fn atom_set_hybridization(atom: Pin<&mut Atom>, what: i32) -> Result<()>;
        pub fn atom_get_hybridization(atom: &Atom) -> Result<HybridizationType>;

        pub fn ro_mol_update_property_cache(atom: Pin<&mut ROMol>, strict: bool) -> Result<()>;

        pub fn set_int_prop(atom: Pin<&mut Atom>, key: &CxxString, value: i32) -> Result<()>;
        pub fn get_int_prop(atom: &Atom, key: &CxxString) -> Result<i32>;
        pub fn set_float_prop(atom: Pin<&mut Atom>, key: &CxxString, value: f64) -> Result<()>;
        pub fn get_float_prop(atom: &Atom, key: &CxxString) -> Result<f64>;
        pub fn set_bool_prop(atom: Pin<&mut Atom>, key: &CxxString, value: bool) -> Result<()>;
        pub fn get_bool_prop(atom: &Atom, key: &CxxString) -> Result<bool>;
        pub fn set_prop(atom: Pin<&mut Atom>, key: &CxxString, value: &CxxString) -> Result<()>;
        pub fn get_prop(atom: &Atom, key: &CxxString) -> Result<String>;
        pub fn get_num_radical_electrons(atom: &Atom) -> Result<u32>;

        //Returns the degree of the atom in the molecule.
        pub fn get_degree(atom: &Atom) -> Result<u32>;
    }
}
