use rdkit::graphmol::{atom::HybridizationType, ro_mol::ROMol};
#[test]
fn test_atom() {
    let mut romol = ROMol::from_smiles("[NH4+]").unwrap();
    let atom = romol.atom_with_idx(0);

    assert_eq!(atom.symbol(), "N");
    assert!(!atom.get_is_aromatic());
    assert_eq!(atom.get_atomic_num(), 7);
    assert_eq!(atom.get_hybridization_type(), HybridizationType::SP3);
    assert_eq!(atom.get_formal_charge(), 1);
    assert_eq!(atom.get_total_num_hs(), 4);
    assert_eq!(atom.get_total_valence(), 4);
}

#[test]
fn test_atom_update_property_cache_exception() {
    let mut romol = ROMol::from_smiles("C([H])([H])([H])([H])").unwrap();
    let mut carbon = romol.atom_with_idx(0);
    carbon.set_num_explicit_hs(5);

    assert_eq!(
        carbon.update_property_cache(true).err().unwrap().what(),
        "Explicit valence for atom # 0 C, 5, is greater than permitted"
    )
}

#[test]
fn test_set_and_get_properties() {
    let mut romol = ROMol::from_smiles("CC").unwrap();
    {
        let mut carbon = romol.atom_with_idx(0);
        carbon.set_prop("int", 42);
        let carbon_2 = romol.atom_with_idx(0);
        assert_eq!(carbon_2.get_int_prop("int").unwrap(), 42);
    }
    {
        let mut carbon = romol.atom_with_idx(0);
        carbon.set_prop("float", 3.14);
        let carbon_3 = romol.atom_with_idx(0);
        assert_eq!(carbon_3.get_float_prop("float").unwrap(), 3.14);
    }
    {
        let mut carbon = romol.atom_with_idx(0);
        carbon.set_prop("bool", true);
        let carbon_4 = romol.atom_with_idx(0);
        assert_eq!(carbon_4.get_bool_prop("bool").unwrap(), true);
    }
    {
        let mut carbon = romol.atom_with_idx(0);
        carbon.set_prop("string", "hello");
        let carbon_5 = romol.atom_with_idx(0);
        assert_eq!(carbon_5.get_prop("string").unwrap(), "hello");
    }
    let carbon = romol.atom_with_idx(0);
    assert_eq!(carbon.get_int_prop("int").unwrap(), 42);
    assert_eq!(carbon.get_float_prop("float").unwrap(), 3.14);
}

#[test]
fn test_number_of_radical_electrons() {
    let mut romol = ROMol::from_smiles("CC").unwrap();
    let carbon = romol.atom_with_idx(0);
    assert_eq!(carbon.get_num_radical_electrons(), 0);
}

#[test]
fn test_degree() {
    let mut romol = ROMol::from_smiles("CC").unwrap();
    let carbon = romol.atom_with_idx(0);
    assert_eq!(carbon.get_degree(), 1);
}
