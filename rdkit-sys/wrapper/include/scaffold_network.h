#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/ScaffoldNetwork/ScaffoldNetwork.h>

namespace RDKit {
using ScaffoldNetworkParams = ScaffoldNetwork::ScaffoldNetworkParams;

std::shared_ptr<ScaffoldNetworkParams> default_scaffold_network_params();
std::shared_ptr<ScaffoldNetworkParams> new_scaffold_network_params(const rust::Vec<rust::String> &bondBreakersSmarts);

void set_include_generic_scaffolds(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void include_generic_bond_scaffolds(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void include_scaffolds_without_attachments(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void include_scaffolds_with_attachments(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void keep_only_first_fragment(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void prune_before_fragmenting(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void flatten_isotopes(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void flatten_chirality(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void flatten_keep_largest(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);
void collect_mol_counts(std::shared_ptr<ScaffoldNetworkParams> &params, bool input);

using ScaffoldNetworkClass = ScaffoldNetwork::ScaffoldNetwork;

std::shared_ptr<ScaffoldNetworkClass> default_scaffold_network();

// RDKit::ScaffoldNetwork::createScaffoldNetwork is a template over a container of molecules, but only
// std::vector<ROMOL_SPTR> (boost::shared_ptr) and std::vector<std::shared_ptr<ROMol>> are explicitly
// instantiated. cxx hands us one std::shared_ptr<ROMol> at a time, so we wrap it in a one element
// vector here rather than fighting cxx over Vec<SharedPtr<ROMol>> (see BACKLOG.md). Callers who want
// a network spanning several molecules build it up with update_scaffold_network.
std::shared_ptr<ScaffoldNetworkClass> create_scaffold_network(const std::shared_ptr<ROMol> &mol,
                                                              const std::shared_ptr<ScaffoldNetworkParams> &params);
void update_scaffold_network(std::shared_ptr<ScaffoldNetworkClass> &network, const std::shared_ptr<ROMol> &mol,
                             const std::shared_ptr<ScaffoldNetworkParams> &params);

// The ScaffoldNetwork members come back as plain data, one parallel vector per member, so that no
// C++ type crosses the bridge. edges[i] runs from edge_begin_indices[i] to edge_end_indices[i], both
// of which index into the nodes vector.
rust::Vec<rust::String> scaffold_network_nodes(const std::shared_ptr<ScaffoldNetworkClass> &network);
rust::Vec<std::uint32_t> scaffold_network_counts(const std::shared_ptr<ScaffoldNetworkClass> &network);
rust::Vec<std::uint32_t> scaffold_network_mol_counts(const std::shared_ptr<ScaffoldNetworkClass> &network);
rust::Vec<std::size_t> scaffold_network_edge_begin_indices(const std::shared_ptr<ScaffoldNetworkClass> &network);
rust::Vec<std::size_t> scaffold_network_edge_end_indices(const std::shared_ptr<ScaffoldNetworkClass> &network);
rust::Vec<rust::String> scaffold_network_edge_types(const std::shared_ptr<ScaffoldNetworkClass> &network);
} // namespace RDKit