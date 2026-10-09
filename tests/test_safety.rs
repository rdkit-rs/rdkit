use bitvec::prelude::*;
use rdkit::*;
use static_assertions::{assert_impl_all, assert_not_impl_any};
use std::io::{self, BufRead, Cursor, Read};

assert_impl_all!(ROMol: Send);
assert_impl_all!(RWMol: Send);
assert_not_impl_any!(ROMol: Sync);
assert_not_impl_any!(RWMol: Sync);
assert_not_impl_any!(AtomRef<'static>: Send, Sync);
assert_not_impl_any!(Atom<'static>: Send, Sync);
assert_not_impl_any!(ConformerMut<'static>: Send, Sync);
assert_not_impl_any!(Bond<'static>: Send, Sync);
assert_not_impl_any!(Conformer<'static>: Send, Sync);
assert_not_impl_any!(rdkit_sys::ro_mol_ffi::ROMol: Send, Sync);
assert_not_impl_any!(rdkit_sys::rw_mol_ffi::RWMol: Send, Sync);

#[test]
fn parsers_reject_invalid_input_and_preserve_empty_molecules() {
    assert!(ROMol::from_smiles("C1").is_err());
    assert!(RWMol::from_smarts("[C").is_err());
    assert!(RWMol::try_from_mol_block("invalid", true, true, true).is_err());
    assert!(RWMol::from_mol_block("invalid", true, true, true).is_none());
    assert_eq!(ROMol::from_smiles("").unwrap().num_atoms(true), 0);
    assert_eq!(RWMol::from_smarts("").unwrap().num_atoms(true), 0);
}

#[test]
fn borrowed_atoms_coexist_and_clones_are_independent() {
    let original = ROMol::from_smiles("CO").unwrap();
    let carbon = original.atom_ref(0).unwrap();
    let oxygen = original.atom_ref(1).unwrap();
    assert_eq!((carbon.symbol(), oxygen.symbol()), ("C".into(), "O".into()));
    assert_eq!(original.atoms().len(), 2);
    let mut cloned = original.clone();
    cloned.atom_with_idx(1).set_formal_charge(-1);
    assert_eq!(oxygen.get_formal_charge(), 0);
    assert_eq!(cloned.atom_ref(1).unwrap().get_formal_charge(), -1);
    drop(original);
    assert_eq!(cloned.atom_ref(0).unwrap().symbol(), "C");
}

#[test]
fn graph_bounds_and_editing_are_checked() {
    let mut mol = RWMol::new();
    assert!(mol.add_atom(119).is_err());
    let carbon = mol.add_atom(6).unwrap();
    let oxygen = mol.add_atom(8).unwrap();
    mol.add_bond(carbon, oxygen, BondType::SINGLE).unwrap();
    assert!(mol.add_bond(carbon, oxygen, BondType::SINGLE).is_err());
    assert!(mol.add_bond(carbon, carbon, BondType::SINGLE).is_err());
    assert!(mol.add_bond(carbon, 42, BondType::SINGLE).is_err());
    assert!(mol
        .add_bond(carbon, oxygen, BondType { repr: i32::MAX })
        .is_err());
    assert!(mol.atom_ref(2).is_err());
    assert!(mol.try_atom_with_idx(u32::MAX).is_err());
    assert!(mol.bond(1).is_err());
    mol.sanitize().unwrap();
    assert_eq!(mol.as_smiles(), "CO");
    let bond = mol.bond(0).unwrap();
    assert_eq!(bond.bond_type(), BondType::SINGLE);
    assert_eq!(bond.other_atom_idx(carbon), oxygen);
    assert!(bond.try_other_atom_idx(42).is_err());
    assert!(mol.remove_atom(8).is_err());
    assert!(mol.remove_bond(0, 8).is_err());
    mol.remove_bond(carbon, oxygen).unwrap();
    assert!(mol.remove_bond(carbon, oxygen).is_err());
    mol.remove_atom(oxygen).unwrap();
    mol.sanitize().unwrap();
    assert_eq!(mol.to_ro_mol().as_smiles(), "C");
}

#[test]
fn invalid_enum_and_hydrogen_counts_return_errors() {
    let mut mol = ROMol::from_smiles("C").unwrap();
    let mut atom = mol.atom_with_idx(0);
    assert!(atom.try_set_formal_charge(128).is_err());
    assert!(atom.try_set_formal_charge(-129).is_err());
    assert!(atom.try_set_num_explicit_hs(-1).is_err());
    assert!(atom.try_set_num_explicit_hs(256).is_err());
    assert!(atom
        .try_set_hybridization_type(HybridizationType { repr: i32::MAX })
        .is_err());
    assert_eq!(atom.get_total_num_hs(), 4);
}

#[test]
fn chemistry_exceptions_reach_rust() {
    let mut params = SmilesParserParams::default();
    params.set_sanitize(false);
    let mut mol = ROMol::from_smiles_with_params("F(C)(C)(C)(C)(C)", &params).unwrap();
    assert!(!try_detect_chemistry_problems(&mol).unwrap().is_empty());
    assert!(mol.try_update_property_cache(true).is_err());
    assert!(mol.atom_with_idx(0).update_property_cache(true).is_err());
}

#[test]
fn low_level_exception_downcast_is_checked() {
    use rdkit_sys::ro_mol_ffi as ffi;
    let mut params = ffi::new_smiles_parser_params().unwrap();
    ffi::smiles_parser_params_set_sanitize(params.pin_mut(), false).unwrap();
    cxx::let_cxx_string!(smiles = "c1cccc1");
    let mol = ffi::smiles_to_mol_with_params(&smiles, &params).unwrap();
    let problems = ffi::detect_chemistry_problems(&mol).unwrap();
    let problem = problems
        .iter()
        .find(|p| ffi::mol_sanitize_exception_type(p).unwrap() == "KekulizeException")
        .unwrap();
    assert!(ffi::atom_sanitize_exception_get_atom_idx(problem).is_err());
}

#[test]
fn bond_lookup_distinguishes_missing_from_invalid() {
    let mol = ROMol::from_smiles("CC.O").unwrap();
    assert!(mol.bond_between_atoms(0, 1).unwrap().is_some());
    assert!(mol.bond_between_atoms(0, 2).unwrap().is_none());
    assert!(mol.bond_between_atoms(0, 3).is_err());
}

#[test]
fn conformer_coordinates_are_owned_and_checked() {
    let mut mol = ROMol::from_smiles("CCO").unwrap();
    assert!(mol.conformer(-1).is_err());
    let id = mol.compute_2d_coords().unwrap();
    assert_eq!(mol.num_conformers(), 1);
    let conf = mol.conformer(id as i32).unwrap();
    assert_eq!(conf.id(), id);
    assert_eq!(conf.num_atoms(), 3);
    assert!(!conf.is_3d());
    assert!(conf.position(3).is_err());
    let original = conf.position(0).unwrap();
    let mut cloned = mol.clone();
    let position = Point3D {
        x: 1.5,
        y: -2.0,
        z: 0.25,
    };
    cloned
        .conformer_mut(id as i32)
        .unwrap()
        .set_position(0, position)
        .unwrap();
    assert_eq!(
        cloned.conformer(id as i32).unwrap().position(0).unwrap(),
        position
    );
    assert_eq!(
        mol.conformer(id as i32).unwrap().position(0).unwrap(),
        original
    );
    let mut conf = mol.conformer_mut(id as i32).unwrap();
    assert!(conf.set_position(3, position).is_err());
    assert!(conf
        .set_position(
            0,
            Point3D {
                x: f64::NAN,
                y: 0.0,
                z: 0.0
            }
        )
        .is_err());
}

#[test]
fn fingerprint_bit_lengths_and_similarity_are_exact() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    for bits in [1, 63, 64, 65, 167, 2048] {
        let fp = mol.morgan_fingerprint_with_params(2, bits).unwrap();
        assert_eq!(fp.0.len(), bits as usize);
        assert_eq!(fp.tanimoto_similarity(&fp), 1.0);
    }
    assert_eq!(mol.maccs_fingerprint().unwrap().0.len(), 167);
    assert!(mol.morgan_fingerprint_with_params(2, 0).is_err());
    assert!(mol.pattern_fingerprint_with_params(0).is_err());
    assert!(mol.rdk_fingerprint_with_params(5, 2, 64).is_err());
    let empty = Fingerprint(BitVec::new());
    assert_eq!(empty.tanimoto_similarity(&empty), 1.0);
    let zeros = Fingerprint(bitvec![u8, Lsb0; 0; 64]);
    assert_eq!(zeros.tanimoto_distance(&zeros), 1.0);
    let a = Fingerprint(bitvec![u8, Lsb0; 1, 1, 0]);
    let b = Fingerprint(bitvec![u8, Lsb0; 0, 1, 1]);
    assert_eq!(a.tanimoto_similarity(&b), 1.0 / 3.0);
    assert!(std::panic::catch_unwind(|| empty.tanimoto_similarity(&zeros)).is_err());
}

#[test]
fn tautomer_iterator_reports_remaining_items() {
    let mol = ROMol::from_smiles("CC(=O)C").unwrap();
    let mut tautomers = TautomerEnumerator::new().try_enumerate(&mol).unwrap();
    let total = tautomers.len();
    assert!(total > 1);
    assert!(tautomers.get(total).is_err());
    let first = tautomers.try_next().unwrap().unwrap();
    assert_eq!(tautomers.len(), total - 1);
    assert_eq!(tautomers.size_hint(), (total - 1, Some(total - 1)));
    while tautomers.next().is_some() {}
    assert_eq!(tautomers.size_hint(), (0, Some(0)));
    assert!(tautomers.next().is_none());
    drop(tautomers);
    assert!(!first.as_smiles().is_empty());
}

#[test]
fn periodic_table_bad_inputs_return_errors() {
    assert!(PeriodicTable::try_get_atomic_weight(119).is_err());
    assert!(PeriodicTable::try_get_element_symbol(u32::MAX).is_err());
    assert!(PeriodicTable::try_get_atomic_number("not-an-element").is_err());
    assert!(PeriodicTable::try_get_valence_list(119).is_err());
    assert_eq!(PeriodicTable::get_max_atomic_number(), 118);
    let row_ends = [0, 2, 10, 18, 36, 54, 86, 118];
    for number in 0..=118 {
        let expected = row_ends.iter().position(|end| number <= *end).unwrap() as u32;
        assert_eq!(PeriodicTable::try_get_row(number).unwrap(), expected);
    }
    assert!(PeriodicTable::try_get_row(119).is_err());
}

#[test]
fn sdf_reader_preserves_dollars_and_empty_title() {
    let mol = ROMol::from_smiles("CO").unwrap();
    let block = mol.to_molblock();
    let record = format!("{block}>  <VALUE>\nprice $5\n\n$$$$\n{block}$$$$\n");
    let mut records = VerboseMolBlockIter::new(Cursor::new(record), true, true, true);
    let (parsed, text) = records.next().unwrap().unwrap();
    assert_eq!(parsed.as_smiles(), "CO");
    assert!(text.contains("price $5"));
    assert!(text.starts_with('\n'));
    assert!(records.next().unwrap().is_ok());
    assert!(records.next().is_none());
}

struct BrokenReader;
impl Read for BrokenReader {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("broken reader"))
    }
}
impl BufRead for BrokenReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::other("broken reader"))
    }
    fn consume(&mut self, _: usize) {}
}

#[test]
fn sdf_io_and_utf8_errors_are_reported_once() {
    let mut reader = MolBlockIter::new(BrokenReader, true, true, true);
    assert!(reader
        .next()
        .unwrap()
        .unwrap_err()
        .contains("broken reader"));
    assert!(reader.next().is_none());
    let mut reader = MolBlockIter::new(Cursor::new(vec![0xff]), true, true, true);
    assert!(reader.next().unwrap().unwrap_err().contains("UTF-8"));
    assert!(reader.next().is_none());
    assert!(
        MolBlockIter::from_gz_file("/path/that/does/not/exist.sdf.gz", true, true, true).is_err()
    );
}

#[test]
fn unique_molecule_can_move_between_threads() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    let mol = std::thread::spawn(move || {
        let mut copy = mol.clone();
        copy.atom_with_idx(2).set_formal_charge(-1);
        (mol.as_smiles(), copy.as_smiles())
    })
    .join()
    .unwrap();
    assert_eq!(mol.0, "CCO");
    assert_eq!(mol.1, "CC[O-]");
}

#[test]
fn smiles_error_display_remains_compatible_and_retains_native_details() {
    let error = ROMol::from_smiles("F(C)(C)(C)(C)(C)").unwrap_err();
    assert_eq!(
        error.to_string(),
        "could not convert smiles to romol (exception)"
    );
    match error {
        ROMolError::ConversionException(message) => assert!(message.contains("valence")),
        other => panic!("unexpected error: {other:?}"),
    }
    assert_eq!(
        ROMolError::UnknownConversionError.to_string(),
        "could not convert smiles to romol (nullptr)"
    );
}

#[test]
fn low_level_smiles_null_remains_explicitly_checkable() {
    cxx::let_cxx_string!(smiles = "C1");
    let ptr = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();
    assert!(ptr.is_null());
    assert_eq!(
        ROMol::from_smiles("C1").unwrap_err(),
        ROMolError::UnknownConversionError
    );
}

#[test]
fn periodic_valence_lists_own_their_storage() {
    use rdkit_sys::PeriodicTableOps;
    let surviving_table = rdkit_sys::periodic_table_ffi::get_periodic_table().unwrap();
    let list = {
        let table = rdkit_sys::periodic_table_ffi::get_periodic_table().unwrap();
        assert_eq!(table.getDefaultValence(6).unwrap(), 4);
        table.getValenceList(6).unwrap()
    };
    assert_eq!(list.as_slice(), &[4]);
    assert_eq!(surviving_table.getDefaultValence(6).unwrap(), 4);
    drop(surviving_table);
    assert_eq!(PeriodicTable::get_default_valence(6), 4);
    let mut first = PeriodicTable::get_valence_list(6);
    let second = PeriodicTable::get_valence_list(6);
    first.push(99);
    assert_eq!(second, vec![4]);
}

#[test]
fn bond_types_round_trip_through_native_graph_edits() {
    for kind in [
        BondType::UNSPECIFIED,
        BondType::SINGLE,
        BondType::DOUBLE,
        BondType::TRIPLE,
        BondType::QUADRUPLE,
        BondType::QUINTUPLE,
        BondType::HEXTUPLE,
        BondType::ONEANDAHALF,
        BondType::TWOANDAHALF,
        BondType::THREEANDAHALF,
        BondType::FOURANDAHALF,
        BondType::FIVEANDAHALF,
        BondType::AROMATIC,
        BondType::IONIC,
        BondType::HYDROGEN,
        BondType::THREECENTER,
        BondType::DATIVEONE,
        BondType::DATIVE,
        BondType::DATIVEL,
        BondType::DATIVER,
        BondType::OTHER,
        BondType::ZERO,
    ] {
        let mut mol = RWMol::new();
        mol.add_atom(6).unwrap();
        mol.add_atom(6).unwrap();
        let idx = mol.add_bond(0, 1, kind).unwrap();
        assert_eq!(mol.bond(idx).unwrap().bond_type(), kind);
    }
    for invalid in [-1, 22, 256, i32::MAX] {
        let mut mol = RWMol::new();
        mol.add_atom(6).unwrap();
        mol.add_atom(6).unwrap();
        assert!(mol.add_bond(0, 1, BondType { repr: invalid }).is_err());
        assert_eq!(mol.num_bonds(), 0);
    }
}

#[test]
fn hybridization_round_trips_without_native_enum_layout_assumptions() {
    let mut mol = ROMol::from_smiles("C").unwrap();
    let mut atom = mol.atom_with_idx(0);
    for hybridization in [
        HybridizationType::UNSPECIFIED,
        HybridizationType::S,
        HybridizationType::SP,
        HybridizationType::SP2,
        HybridizationType::SP3,
        HybridizationType::SP2D,
        HybridizationType::SP3D,
        HybridizationType::SP3D2,
        HybridizationType::OTHER,
    ] {
        atom.try_set_hybridization_type(hybridization).unwrap();
        assert_eq!(atom.try_get_hybridization_type().unwrap(), hybridization);
    }
    for invalid in [-1, 9, 256, i32::MAX] {
        assert!(atom
            .try_set_hybridization_type(HybridizationType { repr: invalid })
            .is_err());
        assert_eq!(atom.get_hybridization_type(), HybridizationType::OTHER);
    }
}

#[test]
fn parsed_bond_stereo_and_directions_cross_native_enum_layouts() {
    let trans = ROMol::from_smiles("F/C=C/F").unwrap();
    let cis = ROMol::from_smiles("F/C=C\\F").unwrap();
    assert_eq!(trans.bond(1).unwrap().stereo(), BondStereo::STEREOE);
    assert_eq!(cis.bond(1).unwrap().stereo(), BondStereo::STEREOZ);
    assert_eq!(trans.bond(0).unwrap().bond_dir(), BondDir::ENDUPRIGHT);
    assert_eq!(cis.bond(2).unwrap().bond_dir(), BondDir::ENDDOWNRIGHT);
    assert_eq!(trans.bond(0).unwrap().stereo(), BondStereo::STEREONONE);
    assert_eq!(trans.bond(1).unwrap().bond_dir(), BondDir::NONE);
}
