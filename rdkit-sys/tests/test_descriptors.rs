use cxx::let_cxx_string;

#[test]
fn test_descriptors() {
    let_cxx_string!(smiles = "c1ccccc1C(=O)NC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let properties = rdkit_sys::descriptors_ffi::new_properties();

    let names = rdkit_sys::descriptors_ffi::get_property_names(&properties);
    let names = names
        .into_iter()
        .map(|stringy| stringy.to_string())
        .collect::<Vec<_>>();
    let computed = rdkit_sys::descriptors_ffi::compute_properties(&properties, &mol);
    let computed = computed
        .into_iter()
        .map(|floaty| *floaty)
        .collect::<Vec<f64>>();

    let expected_names = vec![
        "exactmw",
        "amw",
        "lipinskiHBA",
        "lipinskiHBD",
        "NumRotatableBonds",
        "NumHBD",
        "NumHBA",
        "NumHeavyAtoms",
        "NumAtoms",
        "NumHeteroatoms",
        "NumAmideBonds",
        "FractionCSP3",
        "NumRings",
        "NumAromaticRings",
        "NumAliphaticRings",
        "NumSaturatedRings",
        "NumHeterocycles",
        "NumAromaticHeterocycles",
        "NumSaturatedHeterocycles",
        "NumAliphaticHeterocycles",
        "NumSpiroAtoms",
        "NumBridgeheadAtoms",
        "NumAtomStereoCenters",
        "NumUnspecifiedAtomStereoCenters",
        "labuteASA",
        "tpsa",
        "CrippenClogP",
        "CrippenMR",
        "chi0v",
        "chi1v",
        "chi2v",
        "chi3v",
        "chi4v",
        "chi0n",
        "chi1n",
        "chi2n",
        "chi3n",
        "chi4n",
        "hallKierAlpha",
        "kappa1",
        "kappa2",
        "kappa3",
        "Phi",
    ];
    let expected_computed = vec![
        135.068413908,
        135.166,
        2.0,
        1.0,
        1.0,
        1.0,
        1.0,
        10.0,
        19.0,
        2.0,
        1.0,
        0.125,
        1.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        59.872671631941984,
        29.1,
        1.0462,
        39.83220000000001,
        5.794999636411992,
        3.114807747754891,
        1.2630398384597699,
        1.2630398384597699,
        0.733865134927391,
        5.794999636411992,
        3.114807747754891,
        1.2630398384597699,
        1.2630398384597699,
        0.733865134927391,
        -1.31,
        6.805074798619103,
        3.011779444695785,
        1.34501586290279,
        2.049538439809832,
    ];

    assert_eq!(names, expected_names);
    assert_eq!(computed, expected_computed);
}
