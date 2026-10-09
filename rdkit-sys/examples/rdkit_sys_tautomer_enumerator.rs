use cxx::let_cxx_string;

fn main() {
    let_cxx_string!(smiles = "c1ccccc1C(=O)NC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    let tautomer_enumerator =
        rdkit_sys::mol_standardize_ffi::tautomer_enumerator().expect("RDKit operation failed");
    let tautomer_enumerator_result =
        rdkit_sys::mol_standardize_ffi::tautomer_enumerate(&tautomer_enumerator, &mol)
            .expect("RDKit operation failed");
    let size = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_size(
        &tautomer_enumerator_result,
    )
    .expect("RDKit operation failed");
    println!("size: {}", size);

    let first = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(
        &tautomer_enumerator_result,
        0,
    )
    .expect("RDKit operation failed");
    let first_smiles =
        rdkit_sys::ro_mol_ffi::mol_to_smiles(&first).expect("RDKit operation failed");
    println!("first smiles: {}", first_smiles);

    let second = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(
        &tautomer_enumerator_result,
        1,
    )
    .expect("RDKit operation failed");
    let second_smiles =
        rdkit_sys::ro_mol_ffi::mol_to_smiles(&second).expect("RDKit operation failed");
    println!("second smiles: {}", second_smiles);

    let canonical_mol = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_canonicalize(
        &tautomer_enumerator,
        &mol,
    )
    .unwrap();
    let canonical_smiles =
        rdkit_sys::ro_mol_ffi::mol_to_smiles(&canonical_mol).expect("RDKit operation failed");
    println!("canonical: {}", canonical_smiles);
}
