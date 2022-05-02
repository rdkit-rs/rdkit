use rdkit::Molecule;

fn main() {
    let smiles1 = "c1ccccc1CCCCCCCC";
    let mol1 = Molecule::from_smile(smiles1).unwrap();
    let smiles2 = "c1ccccc1CCCCCC";
    let mol2 = Molecule::from_smile(smiles2).unwrap();

    let mol1_fingerprint = mol1.fingerprint();

    let mol2_fingerprint = mol2.fingerprint();

    let distance = mol1_fingerprint.tanimoto_distance(&mol2_fingerprint);

    println!(
        "{:?} and {:?} got a tanimoto score of {:?}",
        mol1, mol2, distance
    );
}
