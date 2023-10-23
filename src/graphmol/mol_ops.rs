use rdkit_sys::ro_mol_ffi as ro_mol;

use crate::ROMol;

pub fn detect_chemistry_problems(mol: &ROMol) -> Vec<(String, Option<u32>)> {
    let problems = rdkit_sys::ro_mol_ffi::detect_chemistry_problems(&mol.ptr);
    problems
        .iter()
        .map(|p| {
            let type_ = ro_mol::mol_sanitize_exception_type(p);
            match type_.as_str() {
                "AtomValenceException" => {
                    let atom_idx = ro_mol::atom_sanitize_exception_get_atom_idx(p);
                    (type_, Some(atom_idx))
                }
                _ => (type_, None),
            }
        })
        .collect()
}
