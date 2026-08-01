//! RDKit's scaffold network (`rdScaffoldNetwork`): the hierarchy of scaffolds a
//! molecule reduces to, built by iteratively breaking its non-ring bonds.
//!
//! Prior art for this class of hierarchy, though not the algorithm implemented
//! here: Wilkens, Janes & Su, "HierS: hierarchical scaffold clustering using
//! topological chemical graphs", J. Med. Chem. 2005, 48(9), 3182-93,
//! doi:10.1021/jm049032d. HierS enumerates ring-delimited substructures
//! directly, where RDKit applies a bond breaking reaction and also emits
//! generic and attachment point scaffolds that HierS has no equivalent of.
//! RDKit's own sources cite no paper for the module.

use cxx::SharedPtr;
use rdkit_sys::scaffold_network_ffi;

use crate::ROMol;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ScaffoldNetworkError {
    #[error("could not build scaffold network (exception): {0}")]
    CreationException(String),
}

pub struct ScaffoldNetworkParams {
    pub ptr: SharedPtr<scaffold_network_ffi::ScaffoldNetworkParams>,
}

impl Default for ScaffoldNetworkParams {
    fn default() -> Self {
        ScaffoldNetworkParams::new()
    }
}

impl ScaffoldNetworkParams {
    /// RDKit's stock parameters, including its default bond breaker reaction.
    pub fn new() -> Self {
        let ptr = scaffold_network_ffi::default_scaffold_network_params();

        ScaffoldNetworkParams { ptr }
    }

    /// Stock parameters with the bond breaker reaction(s) replaced. Note that
    /// an empty `bond_breaker_smarts` leaves RDKit with nothing to fragment
    /// on, so every network comes back as a single node.
    pub fn with_bond_breakers(bond_breaker_smarts: &[String]) -> Self {
        let ptr = scaffold_network_ffi::new_scaffold_network_params(&bond_breaker_smarts.to_vec());

        ScaffoldNetworkParams { ptr }
    }

    /// Include scaffolds with every atom replaced by a dummy.
    pub fn set_include_generic_scaffolds(&mut self, what: bool) {
        scaffold_network_ffi::set_include_generic_scaffolds(&mut self.ptr, what)
    }

    /// Include scaffolds with every bond replaced by a single bond.
    pub fn set_include_generic_bond_scaffolds(&mut self, what: bool) {
        scaffold_network_ffi::include_generic_bond_scaffolds(&mut self.ptr, what)
    }

    pub fn set_include_scaffolds_without_attachments(&mut self, what: bool) {
        scaffold_network_ffi::include_scaffolds_without_attachments(&mut self.ptr, what)
    }

    pub fn set_include_scaffolds_with_attachments(&mut self, what: bool) {
        scaffold_network_ffi::include_scaffolds_with_attachments(&mut self.ptr, what)
    }

    pub fn set_keep_only_first_fragment(&mut self, what: bool) {
        scaffold_network_ffi::keep_only_first_fragment(&mut self.ptr, what)
    }

    pub fn set_prune_before_fragmenting(&mut self, what: bool) {
        scaffold_network_ffi::prune_before_fragmenting(&mut self.ptr, what)
    }

    pub fn set_flatten_isotopes(&mut self, what: bool) {
        scaffold_network_ffi::flatten_isotopes(&mut self.ptr, what)
    }

    pub fn set_flatten_chirality(&mut self, what: bool) {
        scaffold_network_ffi::flatten_chirality(&mut self.ptr, what)
    }

    pub fn set_flatten_keep_largest(&mut self, what: bool) {
        scaffold_network_ffi::flatten_keep_largest(&mut self.ptr, what)
    }

    /// Track how many input molecules each scaffold was reached from. With this
    /// off RDKit leaves `molCounts` empty and
    /// [`ScaffoldNetworkNode::mol_count`] is `None`.
    pub fn set_collect_mol_counts(&mut self, what: bool) {
        scaffold_network_ffi::collect_mol_counts(&mut self.ptr, what)
    }
}

/// Why RDKit drew an edge between two scaffolds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaffoldNetworkEdgeType {
    /// molecule -> fragment
    Fragment,
    /// molecule -> generic molecule (all atoms are dummies)
    Generic,
    /// molecule -> generic bond molecule (all bonds single)
    GenericBond,
    /// molecule -> molecule with no attachment points
    RemoveAttachment,
    /// molecule -> flattened molecule
    Initialize,
    /// an edge type this crate does not know about yet
    Unknown,
}

impl ScaffoldNetworkEdgeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScaffoldNetworkEdgeType::Fragment => "Fragment",
            ScaffoldNetworkEdgeType::Generic => "Generic",
            ScaffoldNetworkEdgeType::GenericBond => "GenericBond",
            ScaffoldNetworkEdgeType::RemoveAttachment => "RemoveAttachment",
            ScaffoldNetworkEdgeType::Initialize => "Initialize",
            ScaffoldNetworkEdgeType::Unknown => "UNKNOWN",
        }
    }
}

impl From<&str> for ScaffoldNetworkEdgeType {
    fn from(value: &str) -> Self {
        match value {
            "Fragment" => ScaffoldNetworkEdgeType::Fragment,
            "Generic" => ScaffoldNetworkEdgeType::Generic,
            "GenericBond" => ScaffoldNetworkEdgeType::GenericBond,
            "RemoveAttachment" => ScaffoldNetworkEdgeType::RemoveAttachment,
            "Initialize" => ScaffoldNetworkEdgeType::Initialize,
            _ => ScaffoldNetworkEdgeType::Unknown,
        }
    }
}

impl std::fmt::Display for ScaffoldNetworkEdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldNetworkNode {
    /// Canonical SMILES for the scaffold. Molecules with no rings at all
    /// produce an empty scaffold, so this can legitimately be the empty
    /// string.
    pub scaffold_smiles: String,
    /// How many times this scaffold was encountered.
    pub count: u32,
    /// How many input molecules this scaffold was found in, when the network
    /// was built with [`ScaffoldNetworkParams::set_collect_mol_counts`]
    /// left on.
    pub mol_count: Option<u32>,
}

/// An edge in the scaffold hierarchy, pointing from the more specific scaffold
/// to the more general one. See [`ScaffoldNetworkEdge::child_idx`] and
/// [`ScaffoldNetworkEdge::parent_idx`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldNetworkEdge {
    /// Index into [`ScaffoldNetwork::nodes`] of the scaffold the edge starts
    /// at.
    pub begin_idx: usize,
    /// Index into [`ScaffoldNetwork::nodes`] of the scaffold the edge ends at.
    pub end_idx: usize,
    pub edge_type: ScaffoldNetworkEdgeType,
}

impl ScaffoldNetworkEdge {
    /// The more specific end of the edge: further down the hierarchy, more
    /// rings.
    pub fn child_idx(&self) -> usize {
        self.begin_idx
    }

    /// The more general end of the edge: further up the hierarchy, fewer rings.
    pub fn parent_idx(&self) -> usize {
        self.end_idx
    }
}

/// The scaffold hierarchy for one or more molecules, as a DAG of scaffold
/// SMILES.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScaffoldNetwork {
    pub nodes: Vec<ScaffoldNetworkNode>,
    pub edges: Vec<ScaffoldNetworkEdge>,
}

impl ScaffoldNetwork {
    fn from_ffi(ptr: &SharedPtr<scaffold_network_ffi::ScaffoldNetworkClass>) -> Self {
        let scaffold_smiles = scaffold_network_ffi::scaffold_network_nodes(ptr);
        let counts = scaffold_network_ffi::scaffold_network_counts(ptr);
        let mol_counts = scaffold_network_ffi::scaffold_network_mol_counts(ptr);

        // molCounts is only filled in when collectMolCounts was set, and RDKit leaves
        // it empty otherwise rather than zero filling it
        let mol_counts_match = mol_counts.len() == scaffold_smiles.len();

        let nodes = scaffold_smiles
            .into_iter()
            .enumerate()
            .map(|(idx, smiles)| ScaffoldNetworkNode {
                scaffold_smiles: smiles,
                count: counts.get(idx).copied().unwrap_or_default(),
                mol_count: match mol_counts_match {
                    true => Some(mol_counts[idx]),
                    false => None,
                },
            })
            .collect::<Vec<_>>();

        let begin_indices = scaffold_network_ffi::scaffold_network_edge_begin_indices(ptr);
        let end_indices = scaffold_network_ffi::scaffold_network_edge_end_indices(ptr);
        let edge_types = scaffold_network_ffi::scaffold_network_edge_types(ptr);

        let edges = begin_indices
            .into_iter()
            .zip(end_indices)
            .zip(edge_types)
            .map(|((begin_idx, end_idx), edge_type)| ScaffoldNetworkEdge {
                begin_idx,
                end_idx,
                edge_type: edge_type.as_str().into(),
            })
            .collect::<Vec<_>>();

        ScaffoldNetwork { nodes, edges }
    }
}

/// Build the scaffold hierarchy for a single molecule.
pub fn scaffold_network_for_mol(
    mol: &ROMol,
    params: &ScaffoldNetworkParams,
) -> Result<ScaffoldNetwork, ScaffoldNetworkError> {
    let ptr = scaffold_network_ffi::create_scaffold_network(&mol.ptr, &params.ptr)
        .map_err(|e| ScaffoldNetworkError::CreationException(e.what().to_owned()))?;

    Ok(ScaffoldNetwork::from_ffi(&ptr))
}

/// Build one scaffold hierarchy spanning several molecules, so that shared
/// scaffolds collapse onto a single node and the per-node counts mean
/// something.
pub fn scaffold_network_for_mols(
    mols: &[ROMol],
    params: &ScaffoldNetworkParams,
) -> Result<ScaffoldNetwork, ScaffoldNetworkError> {
    let mut ptr = scaffold_network_ffi::default_scaffold_network();

    for mol in mols {
        scaffold_network_ffi::update_scaffold_network(&mut ptr, &mol.ptr, &params.ptr)
            .map_err(|e| ScaffoldNetworkError::CreationException(e.what().to_owned()))?;
    }

    Ok(ScaffoldNetwork::from_ffi(&ptr))
}
