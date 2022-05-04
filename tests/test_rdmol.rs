use rdkit::{ROMol, Uncharger, fragment_parent, CleanupParameters};

#[test]
fn test_rdmol() {
    let _ = ROMol::from_smile("c1ccccc1C(=O)NC").unwrap();
}

#[test]
fn test_neutralize() {
    let smiles = "CCOC(=O)C(C)(C)OC1=CC=C(C=C1)Cl.CO.C1=CC(=CC=C1C(=O)N[C@@H](CCC(=O)O)C(=O)O)NCC2=CN=C3C(=N2)C(=O)NC(=N3)N";
    let romol = ROMol::from_smile(smiles).unwrap();
    let uncharger = Uncharger::new(false);
    let uncharged_mol = uncharger.uncharge(romol);
    println!("{:?}", uncharged_mol.as_smile());
}

#[test]
fn test_fragment_parent() {
    let smiles = "CCOC(=O)C(C)(C)OC1=CC=C(C=C1)Cl.CO.C1=CC(=CC=C1C(=O)N[C@@H](CCC(=O)O)C(=O)O)NCC2=CN=C3C(=N2)C(=O)NC(=N3)N";
    let romol = ROMol::from_smile(smiles).unwrap();
    let rwmol = romol.to_rw_mol(false, 1);
    let cleanup_params = CleanupParameters::default();
    fragment_parent(rwmol, cleanup_params, true);
}