use cxx::UniquePtr;
use rdkit_sys::scaffold_network_ffi;

use crate::ROMol;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ScaffoldNetworkError {
    #[error("could not build scaffold network (exception): {0}")]
    CreationException(String),
}

pub struct ScaffoldNetworkParams {
    pub(crate) ptr: UniquePtr<scaffold_network_ffi::ScaffoldNetworkParams>,
}

impl Default for ScaffoldNetworkParams {
    fn default() -> Self {
        ScaffoldNetworkParams::new()
    }
}

impl ScaffoldNetworkParams {
    pub fn new() -> Self {
        let ptr = scaffold_network_ffi::default_scaffold_network_params()
            .expect("RDKit scaffold allocation failed");

        ScaffoldNetworkParams { ptr }
    }

    pub fn with_bond_breakers(bond_breaker_smarts: &[String]) -> Self {
        let ptr = scaffold_network_ffi::new_scaffold_network_params(&bond_breaker_smarts.to_vec())
            .expect("RDKit scaffold parameters failed");

        ScaffoldNetworkParams { ptr }
    }

    pub fn try_with_bond_breakers(bond_breaker_smarts: &[String]) -> Result<Self, cxx::Exception> {
        Ok(Self {
            ptr: scaffold_network_ffi::new_scaffold_network_params(&bond_breaker_smarts.to_vec())?,
        })
    }

    pub fn set_include_generic_scaffolds(&mut self, what: bool) {
        scaffold_network_ffi::set_include_generic_scaffolds(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_include_generic_bond_scaffolds(&mut self, what: bool) {
        scaffold_network_ffi::include_generic_bond_scaffolds(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_include_scaffolds_without_attachments(&mut self, what: bool) {
        scaffold_network_ffi::include_scaffolds_without_attachments(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_include_scaffolds_with_attachments(&mut self, what: bool) {
        scaffold_network_ffi::include_scaffolds_with_attachments(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_keep_only_first_fragment(&mut self, what: bool) {
        scaffold_network_ffi::keep_only_first_fragment(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_prune_before_fragmenting(&mut self, what: bool) {
        scaffold_network_ffi::prune_before_fragmenting(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_flatten_isotopes(&mut self, what: bool) {
        scaffold_network_ffi::flatten_isotopes(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_flatten_chirality(&mut self, what: bool) {
        scaffold_network_ffi::flatten_chirality(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_flatten_keep_largest(&mut self, what: bool) {
        scaffold_network_ffi::flatten_keep_largest(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }

    pub fn set_collect_mol_counts(&mut self, what: bool) {
        scaffold_network_ffi::collect_mol_counts(self.ptr.pin_mut(), what)
            .expect("RDKit scaffold parameters failed")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaffoldNetworkEdgeType {
    Fragment,
    Generic,
    GenericBond,
    RemoveAttachment,
    Initialize,
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
    pub scaffold_smiles: String,
    pub count: u32,
    pub mol_count: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldNetworkEdge {
    pub begin_idx: usize,
    pub end_idx: usize,
    pub edge_type: ScaffoldNetworkEdgeType,
}

impl ScaffoldNetworkEdge {
    pub fn child_idx(&self) -> usize {
        self.begin_idx
    }

    pub fn parent_idx(&self) -> usize {
        self.end_idx
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScaffoldNetwork {
    pub nodes: Vec<ScaffoldNetworkNode>,
    pub edges: Vec<ScaffoldNetworkEdge>,
}

impl ScaffoldNetwork {
    fn from_ffi(ptr: &scaffold_network_ffi::ScaffoldNetworkClass) -> Result<Self, cxx::Exception> {
        let scaffold_smiles = scaffold_network_ffi::scaffold_network_nodes(ptr)?;
        let counts = scaffold_network_ffi::scaffold_network_counts(ptr)?;
        let mol_counts = scaffold_network_ffi::scaffold_network_mol_counts(ptr)?;

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

        let begin_indices = scaffold_network_ffi::scaffold_network_edge_begin_indices(ptr)?;
        let end_indices = scaffold_network_ffi::scaffold_network_edge_end_indices(ptr)?;
        let edge_types = scaffold_network_ffi::scaffold_network_edge_types(ptr)?;

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

        Ok(ScaffoldNetwork { nodes, edges })
    }
}

pub fn scaffold_network_for_mol(
    mol: &ROMol,
    params: &ScaffoldNetworkParams,
) -> Result<ScaffoldNetwork, ScaffoldNetworkError> {
    let ptr = scaffold_network_ffi::create_scaffold_network(&mol.ptr, &params.ptr)
        .map_err(|e| ScaffoldNetworkError::CreationException(e.what().to_owned()))?;

    ScaffoldNetwork::from_ffi(&ptr)
        .map_err(|e| ScaffoldNetworkError::CreationException(e.to_string()))
}

pub fn scaffold_network_for_mols(
    mols: &[ROMol],
    params: &ScaffoldNetworkParams,
) -> Result<ScaffoldNetwork, ScaffoldNetworkError> {
    let mut ptr = scaffold_network_ffi::default_scaffold_network()
        .map_err(|e| ScaffoldNetworkError::CreationException(e.to_string()))?;

    for mol in mols {
        scaffold_network_ffi::update_scaffold_network(ptr.pin_mut(), &mol.ptr, &params.ptr)
            .map_err(|e| ScaffoldNetworkError::CreationException(e.what().to_owned()))?;
    }

    ScaffoldNetwork::from_ffi(&ptr)
        .map_err(|e| ScaffoldNetworkError::CreationException(e.to_string()))
}
