use rdkit::{graphmol::ro_mol::ROMol, mol_standardize::TautomerEnumerator};

fn main() {
    let mol = ROMol::from_smiles("c1ccccc1C(=O)NC").unwrap();
    let enumerator = TautomerEnumerator::new();
    let enumerator_result = enumerator.enumerate(&mol);

    for t in enumerator_result {
        println!("{}", t.as_smiles());
    }

    let canonical_mol = enumerator.canonicalize(&mol).unwrap();
    println!("{}", canonical_mol.as_smiles());
}
