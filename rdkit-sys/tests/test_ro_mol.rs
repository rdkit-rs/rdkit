#[test]
fn test_ro_mol() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let mut romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();
    assert!(!romol.is_null());

    rdkit_sys::ro_mol_ffi::ro_mol_update_property_cache(&mut romol, true);
}

#[test]
fn bad_mol_test() {
    cxx::let_cxx_string!(smile = "F(C)(C)(C)(C)(C)");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile);

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
    cxx::let_cxx_string!(smile = "N#[N]c1ccc(cc1)N(C)CN(C)(C)(C)");

    let params = rdkit_sys::ro_mol_ffi::new_smiles_parser_params();

    rdkit_sys::ro_mol_ffi::smiles_parser_params_set_sanitize(&params, true);
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol_with_params(&smile, &params);

    assert!(romol.is_err());

    rdkit_sys::ro_mol_ffi::smiles_parser_params_set_sanitize(&params, false);
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol_with_params(&smile, &params);

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
