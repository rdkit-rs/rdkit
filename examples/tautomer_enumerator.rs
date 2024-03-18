use rdkit::{ROMol, TautomerEnumerator};

fn main() {
    let mol = ROMol::from_smiles("c1ccccc1C(=O)NC").unwrap();
    let enumerator = TautomerEnumerator::new();
    let enumerator_result = enumerator.enumerate(&mol);

    for t in enumerator_result {
        println!("{}", t.as_smiles().unwrap());
    }

    let canonical_mol = enumerator.canonicalize(&mol);
    println!("{}", canonical_mol.as_smiles().unwrap());
}
