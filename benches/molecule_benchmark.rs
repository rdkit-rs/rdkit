#![allow(soft_unstable)]
#![feature(test)]
extern crate test;

use rdkit_sys::molecule::Molecule;

#[bench]
fn bench_molecules(bencher: &mut test::bench::Bencher) {
    let smiles1 = "c1ccccc1CCCCCCCC";

    bencher.iter(|| {
        Molecule::from_smile(smiles1).unwrap();
    })
}

#[bench]
fn bench_fingerprint(bencher: &mut test::bench::Bencher) {
    let smiles1 = "c1ccccc1CCCCCCCC";
    let mol1 = Molecule::from_smile(smiles1).unwrap();

    bencher.iter(|| mol1.fingerprint())
}

#[bench]
fn bench_tanimoto(bencher: &mut test::bench::Bencher) {
    let smiles1 = "c1ccccc1CCCCCCCC";
    let mol1 = Molecule::from_smile(smiles1).unwrap();
    let smiles2 = "c1ccccc1CCCCCC";
    let mol2 = Molecule::from_smile(smiles2).unwrap();

    bencher.iter(|| {
        let mol1_fingerprint = mol1.fingerprint();

        let mol2_fingerprint = mol2.fingerprint();

        mol1_fingerprint.tanimoto_distance(&mol2_fingerprint)
    });
}
