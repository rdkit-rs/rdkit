use crate::ROMol;

pub fn detect_chemistry_problems(mol: &ROMol) -> Vec<String> {
    let problems = rdkit_sys::ro_mol_ffi::detect_chemistry_problems(mol.ptr.clone());
    problems
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}
