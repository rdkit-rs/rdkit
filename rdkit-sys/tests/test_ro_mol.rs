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
fn mol_to_molblock_test() {
    cxx::let_cxx_string!(smiles = "CC");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    let molblock = rdkit_sys::ro_mol_ffi::mol_to_molblock(&romol);
    assert_eq!(molblock, "\n     RDKit          2D\n\n  2  1  0  0  0  0  0  0  0  0999 V2000\n    0.0000    0.0000    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n    1.2990    0.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n  1  2  1  0\nM  END\n");
}

#[test]
fn set_and_get_bool_property_test() {
    cxx::let_cxx_string!(smiles = "CC");
    cxx::let_cxx_string!(key = "foo");
    let mut romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    let atom = rdkit_sys::ro_mol_ffi::get_atom_with_idx(&mut romol, 0);
    rdkit_sys::ro_mol_ffi::set_bool_prop(atom, &key, true);

    // now we create an other variable that is the same atom.
    // if it works, it should
    let atom_2 = rdkit_sys::ro_mol_ffi::get_atom_with_idx(&mut romol, 0);
    let value = rdkit_sys::ro_mol_ffi::get_bool_prop(atom_2, &key);
    assert!(value.is_ok());
    assert_eq!(value.unwrap(), true);
}

#[test]
fn set_and_get_int_property_test() {
    cxx::let_cxx_string!(smiles = "CC");
    cxx::let_cxx_string!(key = "foo");
    let mut romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    let atom = rdkit_sys::ro_mol_ffi::get_atom_with_idx(&mut romol, 0);
    rdkit_sys::ro_mol_ffi::set_int_prop(atom, &key, 42);

    // now we create an other variable that is the same atom.
    // if it works, it should
    let atom_2 = rdkit_sys::ro_mol_ffi::get_atom_with_idx(&mut romol, 0);
    let value = rdkit_sys::ro_mol_ffi::get_int_prop(atom_2, &key);
    assert!(value.is_ok());
    assert_eq!(value.unwrap(), 42);

    // get the "bar" property that does not exist, should return an error
    cxx::let_cxx_string!(error_key = "bar");
    let atom_2 = rdkit_sys::ro_mol_ffi::get_atom_with_idx(&mut romol, 0);
    let error_value = rdkit_sys::ro_mol_ffi::get_int_prop(atom_2, &error_key);
    assert!(error_value.is_err());
}
