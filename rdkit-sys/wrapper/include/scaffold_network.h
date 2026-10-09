#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/ScaffoldNetwork/ScaffoldNetwork.h>

namespace RDKit {
using ScaffoldNetworkParams = ScaffoldNetwork::ScaffoldNetworkParams;

std::unique_ptr<ScaffoldNetworkParams> default_scaffold_network_params();
std::unique_ptr<ScaffoldNetworkParams> new_scaffold_network_params(const rust::Vec<rust::String> &bondBreakersSmarts);

void set_include_generic_scaffolds(ScaffoldNetworkParams &params, bool input);
void include_generic_bond_scaffolds(ScaffoldNetworkParams &params, bool input);
void include_scaffolds_without_attachments(ScaffoldNetworkParams &params, bool input);
void include_scaffolds_with_attachments(ScaffoldNetworkParams &params, bool input);
void keep_only_first_fragment(ScaffoldNetworkParams &params, bool input);
void prune_before_fragmenting(ScaffoldNetworkParams &params, bool input);
void flatten_isotopes(ScaffoldNetworkParams &params, bool input);
void flatten_chirality(ScaffoldNetworkParams &params, bool input);
void flatten_keep_largest(ScaffoldNetworkParams &params, bool input);
void collect_mol_counts(ScaffoldNetworkParams &params, bool input);

using ScaffoldNetworkClass = ScaffoldNetwork::ScaffoldNetwork;

std::unique_ptr<ScaffoldNetworkClass> default_scaffold_network();

std::unique_ptr<ScaffoldNetworkClass> create_scaffold_network(const ROMol &mol, const ScaffoldNetworkParams &params);
void update_scaffold_network(ScaffoldNetworkClass &network, const ROMol &mol, const ScaffoldNetworkParams &params);

rust::Vec<rust::String> scaffold_network_nodes(const ScaffoldNetworkClass &network);
rust::Vec<std::uint32_t> scaffold_network_counts(const ScaffoldNetworkClass &network);
rust::Vec<std::uint32_t> scaffold_network_mol_counts(const ScaffoldNetworkClass &network);
rust::Vec<std::size_t> scaffold_network_edge_begin_indices(const ScaffoldNetworkClass &network);
rust::Vec<std::size_t> scaffold_network_edge_end_indices(const ScaffoldNetworkClass &network);
rust::Vec<rust::String> scaffold_network_edge_types(const ScaffoldNetworkClass &network);
}
