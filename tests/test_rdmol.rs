use rdkit::{ROMol, Uncharger, fragment_parent, CleanupParameters, TautomerEnumerator};

#[test]
fn test_rdmol() {
    let _ = ROMol::from_smile("c1ccccc1C(=O)NC").unwrap();
}

#[test]
fn test_neutralize() {
    let smiles = "CCOC(=O)C(C)(C)OC1=CC=C(C=C1)Cl.CO.C1=CC(=CC=C1C(=O)N[C@@H](CCC(=O)O)C(=O)O)NCC2=CN=C3C(=N2)C(=O)NC(=N3)N";
    let romol = ROMol::from_smile(smiles).unwrap();
    let uncharger = Uncharger::new(false);
    let uncharged_mol = uncharger.uncharge(&romol);
    println!("{:?}", uncharged_mol.as_smile());
}

#[test]
fn test_fragment_parent() {
    let smiles = "CCOC(=O)C(C)(C)OC1=CC=C(C=C1)Cl.CO.C1=CC(=CC=C1C(=O)N[C@@H](CCC(=O)O)C(=O)O)NCC2=CN=C3C(=N2)C(=O)NC(=N3)N";
    let romol = ROMol::from_smile(smiles).unwrap();
    let rwmol = romol.to_rw_mol(false, 1);
    let cleanup_params = CleanupParameters::default();
    let parent_rwmol = fragment_parent(&rwmol, &cleanup_params, true);
    println!("{:?}", parent_rwmol.as_smile());
    println!("{:?}", rwmol.as_smile());
}

#[test]
fn test_enumerate_tautomer() {
    let smiles = "Oc1c(cccc3)c3nc2ccncc12";
    let romol = ROMol::from_smile(smiles).unwrap();
    let te = TautomerEnumerator::new();
    let ter = te.enumerate(&romol);
    let ts = ter.collect::<Vec<_>>();
    assert_eq!(ts.len(), 3);
}

#[test]
fn test_stdz() {
    let smiles = "CC.Oc1c(cccc3CC(C(=O)[O-]))c3nc2c(C[NH+])cncc12.[Cl-]";
    let romol = ROMol::from_smile(smiles).unwrap();
    let rwmol = romol.to_rw_mol(false, 1);

    let cleanup_params = CleanupParameters::default();
    let parent_rwmol = fragment_parent(&rwmol, &cleanup_params, true);

    let uncharger = Uncharger::new(false);
    let uncharged_mol = uncharger.uncharge(&parent_rwmol.to_romol());

    let te = TautomerEnumerator::new();
    let canon_taut = te.canonicalize(&uncharged_mol);
    println!("{:?}", canon_taut.as_smile());
}