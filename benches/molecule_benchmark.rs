use rdkit::ROMol;
use std::{hint::black_box, time::Instant};

fn measure<T>(name: &str, iterations: u32, mut operation: impl FnMut() -> T) {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(operation());
    }
    println!(
        "{name}: {:.0} ns/iteration ({iterations} iterations)",
        start.elapsed().as_nanos() as f64 / iterations as f64
    );
}

fn main() {
    let iterations = 1_000;
    let smiles = "c1ccccc1CCCCCCCC";
    measure("SMILES parsing", iterations, || {
        ROMol::from_smiles(black_box(smiles)).unwrap()
    });
    let mol = ROMol::from_smiles(smiles).unwrap();
    measure("RDK fingerprint", iterations, || {
        black_box(&mol).rdk_fingerprint()
    });
    let first = mol.rdk_fingerprint();
    let second = ROMol::from_smiles("c1ccccc1CCCCCC")
        .unwrap()
        .rdk_fingerprint();
    measure("Tanimoto similarity", iterations, || {
        black_box(&first).tanimoto_similarity(black_box(&second))
    });
}
