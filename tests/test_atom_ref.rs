/// Verify AtomRef returns identical results to Atom for all read-only methods.
#[test]
fn test_atom_ref_parity() {
    let mut romol = rdkit::ROMol::from_smiles("[NH4+]").unwrap();

    // Read via mutable Atom
    let atom = romol.atom_with_idx(0);
    let symbol = atom.symbol();
    let is_aromatic = atom.get_is_aromatic();
    let atomic_num = atom.get_atomic_num();
    let hybridization = atom.get_hybridization_type();
    let formal_charge = atom.get_formal_charge();
    let total_num_hs = atom.get_total_num_hs();
    let total_valence = atom.get_total_valence();
    let num_radical_electrons = atom.get_num_radical_electrons();
    let degree = atom.get_degree();

    // Read via immutable AtomRef — must match exactly
    let atom_ref = romol.atom_ref(0);
    assert_eq!(atom_ref.symbol(), symbol);
    assert_eq!(atom_ref.get_is_aromatic(), is_aromatic);
    assert_eq!(atom_ref.get_atomic_num(), atomic_num);
    assert_eq!(atom_ref.get_hybridization_type(), hybridization);
    assert_eq!(atom_ref.get_formal_charge(), formal_charge);
    assert_eq!(atom_ref.get_total_num_hs(), total_num_hs);
    assert_eq!(atom_ref.get_total_valence(), total_valence);
    assert_eq!(atom_ref.get_num_radical_electrons(), num_radical_electrons);
    assert_eq!(atom_ref.get_degree(), degree);
}

/// Verify AtomRef property getters work.
#[test]
fn test_atom_ref_properties() {
    let mut romol = rdkit::ROMol::from_smiles("CC").unwrap();

    // Set properties via mutable Atom
    {
        let mut carbon = romol.atom_with_idx(0);
        carbon.set_prop("int_key", 42);
        carbon.set_prop("float_key", 3.14);
        carbon.set_prop("bool_key", true);
        carbon.set_prop("str_key", "hello");
    }

    // Read back via immutable AtomRef
    let atom_ref = romol.atom_ref(0);
    assert_eq!(atom_ref.get_int_prop("int_key").unwrap(), 42);
    assert_eq!(atom_ref.get_float_prop("float_key").unwrap(), 3.14);
    assert_eq!(atom_ref.get_bool_prop("bool_key").unwrap(), true);
    assert_eq!(atom_ref.get_prop("str_key").unwrap(), "hello");
}

/// Verify multiple AtomRefs can coexist (no &mut self needed).
#[test]
fn test_atom_ref_no_clone_needed() {
    let romol = rdkit::ROMol::from_smiles("CCO").unwrap();

    // This would not compile with atom_with_idx since it needs &mut self.
    // With atom_ref, we can hold multiple references simultaneously.
    let c1 = romol.atom_ref(0);
    let c2 = romol.atom_ref(1);
    let o = romol.atom_ref(2);

    assert_eq!(c1.symbol(), "C");
    assert_eq!(c2.symbol(), "C");
    assert_eq!(o.symbol(), "O");
}

/// Verify AtomRef works across all atoms in a molecule.
#[test]
fn test_atom_ref_iteration() {
    let romol = rdkit::ROMol::from_smiles("c1ccccc1").unwrap();
    let n = romol.num_atoms(true);
    assert_eq!(n, 6);

    for i in 0..n {
        let atom = romol.atom_ref(i);
        assert_eq!(atom.symbol(), "C");
        assert!(atom.get_is_aromatic());
    }
}
