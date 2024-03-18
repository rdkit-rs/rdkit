use rdkit::{
    detect_chemistry_problems, fragment_parent, substruct_match, CleanupParameters,
    MolSanitizeException, ROMol, ROMolError, RWMol, SmilesParserParams, SmilesWriteParams,
    SubstructMatchParameters, TautomerEnumerator, Uncharger,
};

#[test]
fn test_rdmol() {
    let _ = ROMol::from_smiles("c1ccccc1C(=O)NC").unwrap();
}

#[test]
fn test_neutralize() {
    let smiles = "CCOC(=O)C(C)(C)OC1=CC=C(C=C1)Cl.CO.C1=CC(=CC=C1C(=O)N[C@@H](CCC(=O)O)C(=O)O)NCC2=CN=C3C(=N2)C(=O)NC(=N3)N";
    let romol = ROMol::from_smiles(smiles).unwrap();
    let uncharger = Uncharger::new(false);
    let uncharged_mol = uncharger.uncharge(&romol);
    assert_eq!("CCOC(=O)C(C)(C)Oc1ccc(Cl)cc1.CO.Nc1nc2ncc(CNc3ccc(C(=O)N[C@@H](CCC(=O)O)C(=O)O)cc3)nc2c(=O)[nH]1", uncharged_mol.as_smiles().unwrap());
}

#[test]
fn test_fragment_parent() {
    let smiles = "CCOC(=O)C(C)(C)OC1=CC=C(C=C1)Cl.CO.C1=CC(=CC=C1C(=O)N[C@@H](CCC(=O)O)C(=O)O)NCC2=CN=C3C(=N2)C(=O)NC(=N3)N";
    let romol = ROMol::from_smiles(smiles).unwrap();
    let rwmol = romol.as_rw_mol(false, 1);
    let cleanup_params = CleanupParameters::default();
    let parent_rwmol = fragment_parent(&rwmol, &cleanup_params, true);
    assert_eq!(
        "Nc1nc2ncc(CNc3ccc(C(=O)N[C@@H](CCC(=O)O)C(=O)O)cc3)nc2c(=O)[nH]1",
        parent_rwmol.as_smiles().unwrap()
    );
    assert_eq!("CCOC(=O)C(C)(C)Oc1ccc(Cl)cc1.CO.Nc1nc2ncc(CNc3ccc(C(=O)N[C@@H](CCC(=O)O)C(=O)O)cc3)nc2c(=O)[nH]1", rwmol.as_smiles().unwrap());
}

#[test]
fn test_ro_mol_conversion_with_unknown_error() {
    let smiles = "string";
    let romol = ROMol::from_smiles(smiles);
    assert_eq!(romol.err(), Some(ROMolError::UnknownConversionError))
}

#[test]
fn test_ro_mol_conversion_with_conversion_exception() {
    let smiles = "F(C)(C)(C)(C)(C)";
    let romol = ROMol::from_smiles(smiles);
    assert_eq!(
        romol.err(),
        Some(ROMolError::ConversionException(
            "Explicit valence for atom # 0 F, 5, is greater than permitted".to_string()
        ))
    )
}

#[test]
fn test_enumerate_tautomer() {
    let smiles = "Oc1c(cccc3)c3nc2ccncc12";
    let romol = ROMol::from_smiles(smiles).unwrap();
    let te = TautomerEnumerator::new();
    let ter = te.enumerate(&romol);
    let ts = ter.collect::<Vec<_>>();
    assert_eq!(ts.len(), 3);
}

#[test]
fn test_stdz() {
    let smiles = "CC.Oc1c(cccc3CC(C(=O)[O-]))c3nc2c(C[NH+])cncc12.[Cl-]";
    let romol = ROMol::from_smiles(smiles).unwrap();
    let rwmol = romol.as_rw_mol(false, 1);

    let cleanup_params = CleanupParameters::default();
    let parent_rwmol = fragment_parent(&rwmol, &cleanup_params, true);

    let uncharger = Uncharger::new(false);
    let uncharged_mol = uncharger.uncharge(&parent_rwmol.to_ro_mol());

    let te = TautomerEnumerator::new();
    let canon_taut = te.canonicalize(&uncharged_mol);
    assert_eq!(
        "[N]Cc1cncc2c(=O)c3cccc(CCC(=O)O)c3[nH]c12",
        canon_taut.as_smiles().unwrap()
    );
}

#[test]
fn test_rw_mol_from_mol_block() {
    let mol_block = r#"1
  -OEChem-05172223082D

 31 30  0     1  0  0  0  0  0999 V2000
    2.8660    0.7500    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    2.8660   -2.2500    0.0000 O   0  5  0  0  0  0  0  0  0  0  0  0
    2.0000   -0.7500    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    3.7320    2.2500    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    5.4641    0.2500    0.0000 N   0  3  0  0  0  0  0  0  0  0  0  0
    4.5981    0.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    3.7320    0.2500    0.0000 C   0  0  3  0  0  0  0  0  0  0  0  0
    6.3301   -0.2500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    5.9641    1.1160    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    4.9641   -0.6160    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    3.7320   -0.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    2.8660   -1.2500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    2.8660    1.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    2.0000    2.2500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    4.9966    1.2250    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.1996    1.2250    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    3.7320    0.8700    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.0201   -0.7869    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.8671   -0.5600    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.6401    0.2869    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.5010    0.8060    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.2741    1.6530    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    5.4272    1.4260    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.4272   -0.3060    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.6541   -1.1530    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    5.5010   -0.9260    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    3.9441   -1.3326    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.3426   -0.6423    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    2.3100    2.7869    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    1.4631    2.5600    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    1.6900    1.7131    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
  1  7  1  0  0  0  0
  1 13  1  0  0  0  0
  2 12  1  0  0  0  0
  3 12  2  0  0  0  0
  4 13  2  0  0  0  0
  5  6  1  0  0  0  0
  5  8  1  0  0  0  0
  5  9  1  0  0  0  0
  5 10  1  0  0  0  0
  6  7  1  0  0  0  0
  6 15  1  0  0  0  0
  6 16  1  0  0  0  0
  7 11  1  0  0  0  0
  7 17  1  0  0  0  0
  8 18  1  0  0  0  0
  8 19  1  0  0  0  0
  8 20  1  0  0  0  0
  9 21  1  0  0  0  0
  9 22  1  0  0  0  0
  9 23  1  0  0  0  0
 10 24  1  0  0  0  0
 10 25  1  0  0  0  0
 10 26  1  0  0  0  0
 11 12  1  0  0  0  0
 11 27  1  0  0  0  0
 11 28  1  0  0  0  0
 13 14  1  0  0  0  0
 14 29  1  0  0  0  0
 14 30  1  0  0  0  0
 14 31  1  0  0  0  0
M  CHG  2   2  -1   5   1
M  END
> <PUBCHEM_COMPOUND_CID>
1

> <PUBCHEM_COMPOUND_CANONICALIZED>
1

> <PUBCHEM_CACTVS_COMPLEXITY>
214

> <PUBCHEM_CACTVS_HBOND_ACCEPTOR>
4

> <PUBCHEM_CACTVS_HBOND_DONOR>
0

> <PUBCHEM_CACTVS_ROTATABLE_BOND>
5

> <PUBCHEM_CACTVS_SUBSKEYS>
AAADceByOAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHgAAAAAACBThgAYCCAMABAAIAACQCAAAAAAAAAAAAAEIAAACABQAgAAHAAAFIAAQAAAkAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==

> <PUBCHEM_IUPAC_OPENEYE_NAME>
3-acetoxy-4-(trimethylammonio)butanoate

> <PUBCHEM_IUPAC_CAS_NAME>
3-acetyloxy-4-(trimethylammonio)butanoate

> <PUBCHEM_IUPAC_NAME_MARKUP>
3-acetyloxy-4-(trimethylazaniumyl)butanoate

> <PUBCHEM_IUPAC_NAME>
3-acetyloxy-4-(trimethylazaniumyl)butanoate

> <PUBCHEM_IUPAC_SYSTEMATIC_NAME>
3-acetyloxy-4-(trimethylazaniumyl)butanoate

> <PUBCHEM_IUPAC_TRADITIONAL_NAME>
3-acetoxy-4-(trimethylammonio)butyrate

> <PUBCHEM_IUPAC_INCHI>
InChI=1S/C9H17NO4/c1-7(11)14-8(5-9(12)13)6-10(2,3)4/h8H,5-6H2,1-4H3

> <PUBCHEM_IUPAC_INCHIKEY>
RDHQFKQIGNGIED-UHFFFAOYSA-N

> <PUBCHEM_XLOGP3_AA>
0.4

> <PUBCHEM_EXACT_MASS>
203.11575802

> <PUBCHEM_MOLECULAR_FORMULA>
C9H17NO4

> <PUBCHEM_MOLECULAR_WEIGHT>
203.24

> <PUBCHEM_OPENEYE_CAN_SMILES>
CC(=O)OC(CC(=O)[O-])C[N+](C)(C)C

> <PUBCHEM_OPENEYE_ISO_SMILES>
CC(=O)OC(CC(=O)[O-])C[N+](C)(C)C

> <PUBCHEM_CACTVS_TPSA>
66.4

> <PUBCHEM_MONOISOTOPIC_WEIGHT>
203.11575802

> <PUBCHEM_TOTAL_CHARGE>
0

> <PUBCHEM_HEAVY_ATOM_COUNT>
14

> <PUBCHEM_ATOM_DEF_STEREO_COUNT>
0

> <PUBCHEM_ATOM_UDEF_STEREO_COUNT>
1

> <PUBCHEM_BOND_DEF_STEREO_COUNT>
0

> <PUBCHEM_BOND_UDEF_STEREO_COUNT>
0

> <PUBCHEM_ISOTOPIC_ATOM_COUNT>
0

> <PUBCHEM_COMPONENT_COUNT>
1

> <PUBCHEM_CACTVS_TAUTO_COUNT>
1

> <PUBCHEM_COORDINATE_TYPE>
1
5
255

> <PUBCHEM_BONDANNOTATIONS>
7  11  3
"#;

    let rw_mol = RWMol::from_mol_block(mol_block, false, false, false).unwrap();
    assert_eq!("[H]C([H])([H])C(=O)OC([H])(C([H])([H])C(=O)[O-])C([H])([H])[N+](C([H])([H])[H])(C([H])([H])[H])C([H])([H])[H]", &rw_mol.as_smiles().unwrap());
}

#[test]
fn test_detect_chemistry_problems() {
    let smile = "N#[N]c1ccc(cc1)N(C)CN(C)(C)(C)";
    let mut parser_params = SmilesParserParams::default();
    parser_params.sanitize(false);
    let mut mol = ROMol::from_smiles_with_params(smile, &parser_params).unwrap();

    let problems = detect_chemistry_problems(&mol);
    assert_eq!(
        &problems,
        &[
            MolSanitizeException::AtomValenceException { atom_idx: 1 },
            MolSanitizeException::AtomValenceException { atom_idx: 11 },
        ]
    );

    let problem_atom_one = mol.atom_with_idx(1);
    assert_eq!(format!("{}", problem_atom_one), "N");

    let problem_atom_two = mol.atom_with_idx(11);
    assert_eq!(format!("{}", problem_atom_two), "N");
}

#[test]
fn test_building_rwmol_from_smarts() {
    let smarts = "[+1!h0!$([*]~[-1,-2,-3,-4]),-1!$([*]~[+1,+2,+3,+4])]";
    let romol = ROMol::from_smiles("[NH4+]").unwrap();
    let rwmol = RWMol::from_smarts(smarts).unwrap();
    let query_mol = rwmol.to_ro_mol();
    let result = substruct_match(&romol, &query_mol, &SubstructMatchParameters::default());
    // println!("{:?}", result);
    assert_eq!(result.len(), 1);
}

#[test]
fn test_smiles_write_rooted_at_atom() {
    let input_smi = "COc1cc(OC)c2c(c1)OC1(c3ccc(OC)c(OC)c3)C(c3ccccc3)CC(O)C21O";
    let ro_mol = ROMol::from_smiles(input_smi).unwrap();
    let canonical_smi = ro_mol.as_smiles();

    let mut params = SmilesWriteParams::default();
    let another_can_smi = ro_mol.as_smiles_with_params(&params).unwrap();

    assert_eq!(canonical_smi, another_can_smi);
    params.rooted_at_atom(1);

    let non_canonical_smi = ro_mol.as_smiles_with_params(&params).unwrap();
    assert_ne!(canonical_smi, non_canonical_smi);
}

#[test]
fn test_smiles_out_of_bounds_rooted_at_atom() {
    let input_smi = "C";
    let ro_mol = ROMol::from_smiles(input_smi).unwrap();

    let mut params = SmilesWriteParams::default();
    params.rooted_at_atom(1);

    let result = ro_mol.as_smiles_with_params(&params);
    assert!(result.is_err())
}

#[test]
fn test_smiles_write_do_random() {
    use std::collections::HashSet;
    let input_smi = "COc1cc(OC)c2c(c1)OC1(c3ccc(OC)c(OC)c3)C(c3ccccc3)CC(O)C21O";
    let ro_mol = ROMol::from_smiles(input_smi).unwrap();

    let mut params = SmilesWriteParams::default();
    params.do_random(true);

    let mut smiles_set: HashSet<String> = HashSet::new();
    for _ in 0..20 {
        let rand_smi = ro_mol.as_smiles_with_params(&params).unwrap();
        smiles_set.insert(rand_smi);
    }
    assert!(smiles_set.len() > 1);
}

#[test]
fn test_as_random_smiles_vec() {
    use std::collections::HashSet;
    let input_smi = "COc1cc(OC)c2c(c1)OC1(c3ccc(OC)c(OC)c3)C(c3ccccc3)CC(O)C21O";
    let ro_mol = ROMol::from_smiles(input_smi).unwrap();

    let smiles_vec = ro_mol.as_random_smiles_vec(20);
    let smiles_set: HashSet<&String> = HashSet::from_iter(smiles_vec.iter());

    assert!(smiles_set.len() > 1);
}
