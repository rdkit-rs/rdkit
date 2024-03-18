use rdkit_sys::ro_mol_ffi::mol_to_smiles;

#[test]
fn test_ro_mol() {
    cxx::let_cxx_string!(smiles = "c1ccccc1CCCCCCCC");
    let mut romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    assert!(!romol.is_null());

    rdkit_sys::ro_mol_ffi::ro_mol_update_property_cache(&mut romol, true);
}

#[test]
fn bad_mol_test() {
    cxx::let_cxx_string!(smiles = "F(C)(C)(C)(C)(C)");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles);

    if let Err(e) = romol {
        assert_eq!(
            e.what(),
            "Explicit valence for atom # 0 F, 5, is greater than permitted"
        )
    } else {
        panic!("expected err variant")
    }
}

#[test]
fn parse_without_sanitize_test() {
    cxx::let_cxx_string!(smiles = "N#[N]c1ccc(cc1)N(C)CN(C)(C)(C)");

    let params = rdkit_sys::ro_mol_ffi::new_smiles_parser_params();

    rdkit_sys::ro_mol_ffi::smiles_parser_params_set_sanitize(&params, true);
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol_with_params(&smiles, &params);

    assert!(romol.is_err());

    rdkit_sys::ro_mol_ffi::smiles_parser_params_set_sanitize(&params, false);
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol_with_params(&smiles, &params);

    assert!(romol.is_ok());

    let mut romol = romol.unwrap();
    let problems = rdkit_sys::ro_mol_ffi::detect_chemistry_problems(&romol);
    assert_eq!(problems.len(), 2);

    let types = problems
        .iter()
        .map(|p| rdkit_sys::ro_mol_ffi::mol_sanitize_exception_type(p))
        .collect::<Vec<_>>();
    assert_eq!(&types, &["AtomValenceException", "AtomValenceException"]);

    let atom_idxs = problems
        .iter()
        .map(|p| rdkit_sys::ro_mol_ffi::atom_sanitize_exception_get_atom_idx(p))
        .collect::<Vec<_>>();
    assert_eq!(&atom_idxs, &[1, 11]);

    let atoms = atom_idxs
        .into_iter()
        .map(|idx| {
            let atom = rdkit_sys::ro_mol_ffi::get_atom_with_idx(&mut romol, idx);
            rdkit_sys::ro_mol_ffi::get_symbol(atom.as_ref())
        })
        .collect::<Vec<_>>();
    assert_eq!(atoms, &["N", "N"]);
}

#[test]
fn mol_to_smiles_with_params_diff_atom_root_test() {
    use std::collections::HashSet;

    cxx::let_cxx_string!(smiles = "C(I)(Br)(F)");
    let ro_mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let params = rdkit_sys::ro_mol_ffi::new_smiles_write_params();

    let mut smiles_set: HashSet<String> = HashSet::new();

    for at in 0..4 {
        rdkit_sys::ro_mol_ffi::smiles_write_params_set_rooted_at_atom(&params, at);
        let aug_smiles =
            rdkit_sys::ro_mol_ffi::mol_to_smiles_with_params(&ro_mol, &params).unwrap();
        smiles_set.insert(aug_smiles);
    }

    assert_eq!(smiles_set.len(), 4)
}

#[test]
fn mol_to_smiles_rooted_at_atom_error() {
    cxx::let_cxx_string!(smiles = "C(I)(Br)(F)");
    let ro_mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let params = rdkit_sys::ro_mol_ffi::new_smiles_write_params();

    rdkit_sys::ro_mol_ffi::smiles_write_params_set_rooted_at_atom(&params, 4);
    let aug_smiles = rdkit_sys::ro_mol_ffi::mol_to_smiles_with_params(&ro_mol, &params);

    if let Err(e) = aug_smiles {
        assert_eq!(
            e.what(),
            // a small typo in rdkit here.
            "rootedAtomAtom must be less than the number of atoms"
        )
    } else {
        panic!("expected err variant")
    }
}

#[test]
fn mol_to_smiles_with_params_random_smiles_test() {
    use std::collections::HashSet;

    cxx::let_cxx_string!(smiles = "CC(=O)Nc1ccc(cc1)O");
    let ro_mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let params = rdkit_sys::ro_mol_ffi::new_smiles_write_params();
    rdkit_sys::ro_mol_ffi::smiles_write_params_set_do_random(&params, true);

    let mut smiles_set: HashSet<String> = HashSet::new();
    for _ in 0..20 {
        let aug_smiles =
            rdkit_sys::ro_mol_ffi::mol_to_smiles_with_params(&ro_mol, &params).unwrap();
        smiles_set.insert(aug_smiles);
    }

    assert!(smiles_set.len() > 1)
}

#[test]
fn mol_to_smiles_failure_test() {
    // This failing case was replicated on:
    // - python 3.11.5, version 2022.09.1
    // - debian bookworm, librdkit-dev 202209.3-1
    cxx::let_cxx_string!(smiles = r"CCOC(=O)/C=S(/c1ccc(C(F)(F)F)cc1)=C1/C=C(\C)CCC1=O");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    assert!(mol_to_smiles(&romol).is_err())
}
