/// Example showing 2 possible methods for (limited) SMILES enumeration.
/// May be useful for NLP in data-limited settings.
use rdkit::{ROMol, SmilesWriteParams};

/// Enumerate some SMILES combinations by iterating through which atom the
/// SMILES is rooted at.
fn enumerate_smiles(mol: &ROMol) -> Vec<String> {
    let n_atoms = mol.num_atoms(true);
    let mut params = SmilesWriteParams::default();
    let mut smiles_vec = Vec::new();
    for atom_idx in 0..n_atoms {
        params.rooted_at_atom(atom_idx as i32);
        let aug_smi = mol.as_smiles_with_params(&params).unwrap();
        smiles_vec.push(aug_smi);
    }
    smiles_vec
}

fn main() {
    let input_smi = "COc1cc(OC)c2c(c1)OC1(c3ccc(OC)c(OC)c3)C(c3ccccc3)CC(O)C21O";
    let ro_mol = ROMol::from_smiles(input_smi).unwrap();

    let smiles_vec_0 = enumerate_smiles(&ro_mol);
    println!("generated {:} smiles for mol", smiles_vec_0.len());

    let smiles_vec_1 = ro_mol.as_random_smiles_vec(10);
    println!("generated {:} smiles for mol", smiles_vec_1.len());
}
