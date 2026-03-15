#![allow(soft_unstable)]
#![feature(test)]
extern crate test;

use rdkit::ROMol;

/// Drug-like molecules of varying size for realistic benchmarking.
/// These cover common pharmaceutical scaffolds and natural products.
const SMILES_SET: &[&str] = &[
    // aspirin
    "CC(=O)Oc1ccccc1C(=O)O",
    // ibuprofen
    "CC(C)Cc1ccc(cc1)C(C)C(=O)O",
    // caffeine
    "Cn1c(=O)c2c(ncn2C)n(C)c1=O",
    // diazepam
    "O=C1CN=C(c2ccccc2)c2cc(Cl)ccc2N1C",
    // atorvastatin (lipitor)
    "CC(C)c1c(C(=O)Nc2ccccc2)c(-c2ccccc2)c(-c2ccc(F)cc2)n1CC[C@@H](O)C[C@@H](O)CC(=O)O",
    // taxol core
    "CC1=C2C(OC(=O)c3ccccc3)C(O)C4(OC(=O)C(O)(CC(OC(=O)c5ccccc5)C1O)C24C)C(=O)c1ccc(OC)cc1",
    // vancomycin fragment
    "OC1C(O)C(OC2C(O)C(O)C(O)C(CO)O2)OC(CO)C1NC(=O)C1CC(O)CN1C(=O)C(NC(=O)C1CC(=O)NC(=O)C1O)C(O)c1ccc(O)cc1",
];

/// Baseline: SMILES parsing cost.
#[bench]
fn bench_parse_smiles(b: &mut test::bench::Bencher) {
    b.iter(|| {
        for smiles in SMILES_SET {
            test::black_box(ROMol::from_smiles(smiles).unwrap());
        }
    });
}

/// Iterate all atoms via atom_ref (&self), read 7 properties per atom.
/// This is the realistic featurization workload.
#[bench]
fn bench_atom_ref_all_properties(b: &mut test::bench::Bencher) {
    let mols: Vec<ROMol> = SMILES_SET
        .iter()
        .map(|s| ROMol::from_smiles(s).unwrap())
        .collect();

    b.iter(|| {
        for mol in &mols {
            let n = mol.num_atoms(true);
            for i in 0..n {
                let atom = mol.atom_ref(i);
                test::black_box(atom.symbol());
                test::black_box(atom.get_atomic_num());
                test::black_box(atom.get_formal_charge());
                test::black_box(atom.get_is_aromatic());
                test::black_box(atom.get_hybridization_type());
                test::black_box(atom.get_degree());
                test::black_box(atom.get_total_num_hs());
            }
        }
    });
}

/// Same workload via atom_with_idx (&mut self).
/// Regression guard: should be the same speed as atom_ref.
#[bench]
fn bench_atom_mut_all_properties(b: &mut test::bench::Bencher) {
    let mut mols: Vec<ROMol> = SMILES_SET
        .iter()
        .map(|s| ROMol::from_smiles(s).unwrap())
        .collect();

    b.iter(|| {
        for mol in &mut mols {
            let n = mol.num_atoms(true);
            for i in 0..n {
                let atom = mol.atom_with_idx(i);
                test::black_box(atom.symbol());
                test::black_box(atom.get_atomic_num());
                test::black_box(atom.get_formal_charge());
                test::black_box(atom.get_is_aromatic());
                test::black_box(atom.get_hybridization_type());
                test::black_box(atom.get_degree());
                test::black_box(atom.get_total_num_hs());
            }
        }
    });
}

/// Clone cost alone. Useful for understanding the cost of cloning
/// molecules when only &ROMol is available but mutation is needed.
#[bench]
fn bench_clone_molecules(b: &mut test::bench::Bencher) {
    let mols: Vec<ROMol> = SMILES_SET
        .iter()
        .map(|s| ROMol::from_smiles(s).unwrap())
        .collect();

    b.iter(|| {
        for mol in &mols {
            test::black_box(mol.clone());
        }
    });
}
