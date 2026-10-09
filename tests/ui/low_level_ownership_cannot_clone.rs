use rdkit_sys::ro_mol_ffi as ffi;
fn main() {
    cxx::let_cxx_string!(smiles = "C");
    let mol = ffi::smiles_to_mol(&smiles).unwrap();
    let alias = mol.clone();
    drop(alias);
}
