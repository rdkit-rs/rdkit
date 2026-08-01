use rdkit::{
    scaffold_network_for_mol, scaffold_network_for_mols, ROMol, ScaffoldNetworkEdgeType,
    ScaffoldNetworkError, ScaffoldNetworkParams,
};

/// 2-phenylquinoline: one fused bicycle plus a pendant ring, so the network has
/// something to actually fragment.
const PHENYLQUINOLINE: &str = "c1ccc(-c2ccc3ncccc3c2)cc1";

/// The stock parameters minus the dummy-atom scaffolds, which keeps the
/// assertions below readable.
fn params_without_generics() -> ScaffoldNetworkParams {
    let mut params = ScaffoldNetworkParams::default();
    params.set_include_generic_scaffolds(false);
    params.set_include_generic_bond_scaffolds(false);
    params
}

fn scaffold_smiles(network: &rdkit::ScaffoldNetwork) -> Vec<&str> {
    network
        .nodes
        .iter()
        .map(|n| n.scaffold_smiles.as_str())
        .collect()
}

#[test]
fn test_scaffold_network_for_mol() {
    let mol = ROMol::from_smiles(PHENYLQUINOLINE).unwrap();
    let network = scaffold_network_for_mol(&mol, &params_without_generics()).unwrap();

    assert_eq!(
        scaffold_smiles(&network),
        [
            PHENYLQUINOLINE,
            "*c1ccccc1",
            "c1ccccc1",
            "*c1ccc2ncccc2c1",
            "c1ccc2ncccc2c1",
        ]
    );
    assert!(network.nodes.iter().all(|n| n.count == 1));
    assert!(network.nodes.iter().all(|n| n.mol_count == Some(1)));

    // every edge runs from a more specific scaffold down to a more general one
    let hierarchy = network
        .edges
        .iter()
        .map(|e| {
            (
                network.nodes[e.child_idx()].scaffold_smiles.as_str(),
                network.nodes[e.parent_idx()].scaffold_smiles.as_str(),
                e.edge_type,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        hierarchy,
        [
            (
                PHENYLQUINOLINE,
                "*c1ccccc1",
                ScaffoldNetworkEdgeType::Fragment
            ),
            (
                "*c1ccccc1",
                "c1ccccc1",
                ScaffoldNetworkEdgeType::RemoveAttachment
            ),
            (
                PHENYLQUINOLINE,
                "*c1ccc2ncccc2c1",
                ScaffoldNetworkEdgeType::Fragment
            ),
            (
                "*c1ccc2ncccc2c1",
                "c1ccc2ncccc2c1",
                ScaffoldNetworkEdgeType::RemoveAttachment
            ),
        ]
    );
}

#[test]
fn test_scaffold_network_default_params_include_generics() {
    let mol = ROMol::from_smiles(PHENYLQUINOLINE).unwrap();
    let network = scaffold_network_for_mol(&mol, &ScaffoldNetworkParams::default()).unwrap();

    // the dummy-atom scaffolds are on by default, and hang off their concrete
    // counterparts
    assert!(scaffold_smiles(&network).contains(&"*1:*:*:*:*:*:1"));
    assert!(network
        .edges
        .iter()
        .any(|e| e.edge_type == ScaffoldNetworkEdgeType::Generic));
}

#[test]
fn test_scaffold_network_single_ring() {
    let mol = ROMol::from_smiles("c1ccccc1").unwrap();
    let network = scaffold_network_for_mol(&mol, &params_without_generics()).unwrap();

    // nothing to strip: the molecule is already its own scaffold
    assert_eq!(scaffold_smiles(&network), ["c1ccccc1"]);
    assert!(network.edges.is_empty());
}

#[test]
fn test_scaffold_network_no_rings() {
    let mol = ROMol::from_smiles("CCO").unwrap();
    let network = scaffold_network_for_mol(&mol, &params_without_generics()).unwrap();

    // RDKit still emits a node for a ring-free molecule, it is just the empty
    // SMILES
    assert_eq!(scaffold_smiles(&network), ["CCO", ""]);
    assert_eq!(
        network.edges[0].edge_type,
        ScaffoldNetworkEdgeType::Initialize
    );
}

#[test]
fn test_scaffold_network_for_mols_shares_nodes() {
    let mols = [
        ROMol::from_smiles(PHENYLQUINOLINE).unwrap(),
        ROMol::from_smiles("c1ccc(-c2ccccc2)cc1").unwrap(),
    ];
    let network = scaffold_network_for_mols(&mols, &params_without_generics()).unwrap();

    // benzene falls out of both molecules, so it is one node reached from two
    // molecules
    let benzene = network
        .nodes
        .iter()
        .find(|n| n.scaffold_smiles == "c1ccccc1")
        .unwrap();
    assert_eq!(benzene.mol_count, Some(2));
    assert!(benzene.count > 1);
}

#[test]
fn test_scaffold_network_without_mol_counts() {
    let mut params = params_without_generics();
    params.set_collect_mol_counts(false);

    let mol = ROMol::from_smiles(PHENYLQUINOLINE).unwrap();
    let network = scaffold_network_for_mol(&mol, &params).unwrap();

    assert!(network.nodes.iter().all(|n| n.mol_count.is_none()));
    assert!(network.nodes.iter().all(|n| n.count == 1));
}

/// An empty bond breaker list leaves RDKit nothing to fragment on, so the whole
/// molecule comes back as a single node.
#[test]
fn test_scaffold_network_with_bond_breakers() {
    let mol = ROMol::from_smiles(PHENYLQUINOLINE).unwrap();

    let params = ScaffoldNetworkParams::with_bond_breakers(&[]);
    let network = scaffold_network_for_mol(&mol, &params).unwrap();
    assert_eq!(scaffold_smiles(&network), [PHENYLQUINOLINE]);

    let params = ScaffoldNetworkParams::with_bond_breakers(&[
        "[!#0;R:1]-!@[!#0:2]>>[*:1]-[#0].[#0]-[*:2]".to_string(),
    ]);
    let network = scaffold_network_for_mol(&mol, &params).unwrap();
    assert!(network.nodes.len() > 1);
}

/// RDKit throws when asked for neither flavor of scaffold; that surfaces as an
/// `Err` rather than unwinding through Rust.
#[test]
fn test_scaffold_network_bad_params() {
    let mut params = ScaffoldNetworkParams::default();
    params.set_include_scaffolds_with_attachments(false);
    params.set_include_scaffolds_without_attachments(false);

    let mol = ROMol::from_smiles(PHENYLQUINOLINE).unwrap();
    assert_eq!(
        scaffold_network_for_mol(&mol, &params),
        Err(ScaffoldNetworkError::CreationException(
            "must include at least one of scaffolds with attachments or scaffolds without attachments".to_string()
        ))
    );
}
