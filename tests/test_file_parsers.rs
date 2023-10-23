use std::path::PathBuf;

#[test]
fn test_mol_block() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = PathBuf::from(root);
    let compounds_gz = root.join("tests/fixtures/Compound_000000001_000000010.sdf.gz");

    let mol_block_iter =
        rdkit::MolBlockIter::from_gz_file(compounds_gz, false, false, false).unwrap();

    let mols = mol_block_iter.collect::<Vec<_>>();
    assert_eq!(mols.len(), 10);
}
