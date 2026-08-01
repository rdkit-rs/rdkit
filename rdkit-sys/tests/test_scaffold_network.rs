use rdkit_sys::scaffold_network_ffi::*;

/// 2-phenylquinoline: one fused bicycle plus a pendant ring, so the network has
/// something to actually fragment.
const PHENYLQUINOLINE: &str = "c1ccc(-c2ccc3ncccc3c2)cc1";

fn mol(smiles: &str) -> cxx::SharedPtr<rdkit_sys::ro_mol_ffi::ROMol> {
    cxx::let_cxx_string!(smiles_cxx_string = smiles);
    rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles_cxx_string).unwrap()
}

/// The stock parameters minus the dummy-atom scaffolds, which keeps the
/// assertions below readable.
fn params_without_generics() -> cxx::SharedPtr<ScaffoldNetworkParams> {
    let mut params = default_scaffold_network_params();
    set_include_generic_scaffolds(&mut params, false);
    include_generic_bond_scaffolds(&mut params, false);
    params
}

#[test]
fn test_scaffold_network() {
    default_scaffold_network_params();
    let mut params = new_scaffold_network_params(&vec![]);

    set_include_generic_scaffolds(&mut params, true);
    include_generic_bond_scaffolds(&mut params, true);
    include_scaffolds_without_attachments(&mut params, true);
    include_scaffolds_with_attachments(&mut params, true);
    keep_only_first_fragment(&mut params, true);
    prune_before_fragmenting(&mut params, true);
    flatten_isotopes(&mut params, true);
    flatten_chirality(&mut params, true);
    flatten_keep_largest(&mut params, true);
    collect_mol_counts(&mut params, true);

    let _scaffold_network = default_scaffold_network();
}

#[test]
fn test_create_scaffold_network() {
    let params = params_without_generics();
    let network = create_scaffold_network(&mol(PHENYLQUINOLINE), &params).unwrap();

    let nodes = scaffold_network_nodes(&network);
    assert_eq!(
        nodes,
        [
            PHENYLQUINOLINE,
            "*c1ccccc1",
            "c1ccccc1",
            "*c1ccc2ncccc2c1",
            "c1ccc2ncccc2c1",
        ]
    );

    // one entry per node, so the downstream can zip them against `nodes`
    assert_eq!(scaffold_network_counts(&network).len(), nodes.len());
    assert_eq!(scaffold_network_mol_counts(&network).len(), nodes.len());
    assert_eq!(scaffold_network_mol_counts(&network), [1, 1, 1, 1, 1]);

    // edges run from the more specific scaffold to the more general one
    let begin_indices = scaffold_network_edge_begin_indices(&network);
    let end_indices = scaffold_network_edge_end_indices(&network);
    let types = scaffold_network_edge_types(&network);
    assert_eq!(begin_indices, [0, 1, 0, 3]);
    assert_eq!(end_indices, [1, 2, 3, 4]);
    assert_eq!(
        types,
        [
            "Fragment",
            "RemoveAttachment",
            "Fragment",
            "RemoveAttachment"
        ]
    );
}

#[test]
fn test_create_scaffold_network_single_ring() {
    let params = params_without_generics();
    let network = create_scaffold_network(&mol("c1ccccc1"), &params).unwrap();

    // nothing to strip: the molecule is already its own scaffold
    assert_eq!(scaffold_network_nodes(&network), ["c1ccccc1"]);
    assert!(scaffold_network_edge_begin_indices(&network).is_empty());
}

#[test]
fn test_create_scaffold_network_no_rings() {
    let params = params_without_generics();
    let network = create_scaffold_network(&mol("CCO"), &params).unwrap();

    // RDKit still emits a node for a ring-free molecule, it is just the empty
    // SMILES
    assert_eq!(scaffold_network_nodes(&network), ["CCO", ""]);
    assert_eq!(scaffold_network_edge_types(&network), ["Initialize"]);
}

#[test]
fn test_update_scaffold_network() {
    let params = params_without_generics();
    let mut network = create_scaffold_network(&mol("c1ccccc1"), &params).unwrap();
    update_scaffold_network(&mut network, &mol("c1ccc2ncccc2c1"), &params).unwrap();

    assert_eq!(
        scaffold_network_nodes(&network),
        ["c1ccccc1", "c1ccc2ncccc2c1"]
    );
    assert_eq!(scaffold_network_mol_counts(&network), [1, 1]);
}

/// `new_scaffold_network_params` hands its argument straight to the
/// `ScaffoldNetworkParams(std::vector<std::string>)` constructor, so an empty
/// vec leaves RDKit with no bond breaker reactions and nothing ever fragments.
/// Callers who want the stock fragmentation want
/// `default_scaffold_network_params`.
#[test]
fn test_empty_bond_breakers_do_not_fragment() {
    let params = new_scaffold_network_params(&vec![]);
    let network = create_scaffold_network(&mol(PHENYLQUINOLINE), &params).unwrap();
    assert_eq!(scaffold_network_nodes(&network), [PHENYLQUINOLINE]);

    let params =
        new_scaffold_network_params(&vec!["[!#0;R:1]-!@[!#0:2]>>[*:1]-[#0].[#0]-[*:2]".into()]);
    let network = create_scaffold_network(&mol(PHENYLQUINOLINE), &params).unwrap();
    assert!(scaffold_network_nodes(&network).len() > 1);
}

/// RDKit throws when asked for neither flavor of scaffold; the bridge turns
/// that into an `Err` rather than letting it unwind through Rust.
#[test]
fn test_create_scaffold_network_bad_params() {
    let mut params = default_scaffold_network_params();
    include_scaffolds_with_attachments(&mut params, false);
    include_scaffolds_without_attachments(&mut params, false);

    match create_scaffold_network(&mol(PHENYLQUINOLINE), &params) {
        Err(e) => assert_eq!(
            e.what(),
            "must include at least one of scaffolds with attachments or scaffolds without attachments"
        ),
        Ok(_) => panic!("expected err variant"),
    }
}
