#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {

    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");

        pub type ROMol;

        pub type ExplicitBitVect = crate::fingerprint_ffi::ExplicitBitVect;
        pub type SmilesParserParams;
        pub type Atom;

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

        pub fn mol_to_smiles(mol: &SharedPtr<ROMol>) -> String;

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
        pub fn get_atom_with_idx(mol: &SharedPtr<ROMol>, idx: u32) -> SharedPtr<Atom>;
        pub fn get_symbol(atom: &SharedPtr<Atom>) -> String;
        pub fn get_is_aromatic(atom: &SharedPtr<Atom>) -> bool;
        pub fn get_atomic_num(atom: &SharedPtr<Atom>) -> i32;
        pub fn get_formal_charge(atom: &SharedPtr<Atom>) -> i32;
        pub fn get_total_num_hs(atom: &SharedPtr<Atom>) -> u32;
        pub fn get_total_valence(atom: &SharedPtr<Atom>) -> u32;
        pub fn set_formal_charge(atom: &mut SharedPtr<Atom>, what: i32);
        pub fn set_num_explicit_hs(atom: &mut SharedPtr<Atom>, what: i32);
        pub fn atom_update_property_cache(atom: &mut SharedPtr<Atom>, strict: bool);

        pub fn ro_mol_update_property_cache(atom: &mut SharedPtr<ROMol>, strict: bool);
    }
}
