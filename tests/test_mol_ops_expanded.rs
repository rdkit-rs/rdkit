use rdkit::{ROMol, assign_stereochemistry, get_formal_charge, get_mol_frags, set_aromaticity};

#[test]
fn test_set_aromaticity() {
    let romol = ROMol::from_smiles("C1=CC=CC=C1").unwrap();
    let mut rwmol = romol.as_rw_mol(false, -1);
    set_aromaticity(&mut rwmol);
}

#[test]
fn test_assign_stereochemistry() {
    let mut mol = ROMol::from_smiles("C/C=C/C").unwrap();
    assign_stereochemistry(&mut mol);
}

#[test]
fn test_get_formal_charge() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    assert_eq!(get_formal_charge(&mol), 0);
    let mol = ROMol::from_smiles("[NH4+]").unwrap();
    assert_eq!(get_formal_charge(&mol), 1);
    let mol = ROMol::from_smiles("CC(=O)[O-]").unwrap();
    assert_eq!(get_formal_charge(&mol), -1);
}

#[test]
fn test_get_mol_frags_single() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    let frags = get_mol_frags(&mol);
    assert_eq!(frags.len(), 1);
}

#[test]
fn test_get_mol_frags_multiple() {
    let mol = ROMol::from_smiles("CCO.CC").unwrap();
    let frags = get_mol_frags(&mol);
    assert_eq!(frags.len(), 2);
}

#[test]
fn test_get_mol_frags_salt() {
    let mol = ROMol::from_smiles("[Na+].[Cl-]").unwrap();
    let frags = get_mol_frags(&mol);
    assert_eq!(frags.len(), 2);
}
