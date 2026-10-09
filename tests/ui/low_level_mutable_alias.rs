use rdkit_sys::ro_mol_ffi as ffi;
fn main() {
    cxx::let_cxx_string!(smiles = "CC");
    let mut mol = ffi::smiles_to_mol(&smiles).unwrap();
    let first = ffi::get_atom_with_idx(mol.pin_mut(), 0).unwrap();
    let second = ffi::get_atom_with_idx(mol.pin_mut(), 0).unwrap();
    ffi::set_formal_charge(first, 1).unwrap();
    ffi::set_formal_charge(second, 2).unwrap();
}
