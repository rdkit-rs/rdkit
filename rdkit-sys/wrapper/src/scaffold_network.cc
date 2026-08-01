#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/ScaffoldNetwork/ScaffoldNetwork.h>

namespace RDKit {
using ScaffoldNetworkParams = ScaffoldNetwork::ScaffoldNetworkParams;

std::shared_ptr<ScaffoldNetworkParams> default_scaffold_network_params() {
	ScaffoldNetworkParams *network_params = new ScaffoldNetworkParams();
	return std::shared_ptr<ScaffoldNetworkParams>(network_params);
}

std::shared_ptr<ScaffoldNetworkParams> new_scaffold_network_params(const rust::Vec<rust::String> &bondBreakersSmarts) {
	std::vector<std::string> cc_vec;

	for (auto smarts : bondBreakersSmarts) { cc_vec.push_back(std::string(smarts)); }

	ScaffoldNetworkParams *network_params = new ScaffoldNetworkParams(cc_vec);
	return std::shared_ptr<ScaffoldNetworkParams>(network_params);
}

void set_include_generic_scaffolds(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
	params->includeGenericScaffolds = input;
}

void include_generic_bond_scaffolds(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
	params->includeGenericBondScaffolds = input;
}

void include_scaffolds_without_attachments(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
	params->includeScaffoldsWithoutAttachments = input;
}

void include_scaffolds_with_attachments(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
	params->includeScaffoldsWithAttachments = input;
}

void keep_only_first_fragment(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
	params->keepOnlyFirstFragment = input;
}

void prune_before_fragmenting(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
	params->pruneBeforeFragmenting = input;
}

void flatten_isotopes(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) { params->flattenIsotopes = input; }

void flatten_chirality(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) { params->flattenChirality = input; }

void flatten_keep_largest(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
	params->flattenKeepLargest = input;
}

void collect_mol_counts(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
	params->collectMolCounts = input;
}

using ScaffoldNetworkClass = ScaffoldNetwork::ScaffoldNetwork;

std::shared_ptr<ScaffoldNetworkClass> default_scaffold_network() {
	ScaffoldNetworkClass *scaffold_network = new ScaffoldNetworkClass();
	return std::shared_ptr<ScaffoldNetworkClass>(scaffold_network);
}

std::shared_ptr<ScaffoldNetworkClass> create_scaffold_network(const std::shared_ptr<ROMol> &mol,
                                                              const std::shared_ptr<ScaffoldNetworkParams> &params) {
	std::vector<std::shared_ptr<ROMol>> mols{mol};
	ScaffoldNetworkClass *scaffold_network =
	    new ScaffoldNetworkClass(ScaffoldNetwork::createScaffoldNetwork(mols, *params));
	return std::shared_ptr<ScaffoldNetworkClass>(scaffold_network);
}

void update_scaffold_network(std::shared_ptr<ScaffoldNetworkClass> &network, const std::shared_ptr<ROMol> &mol,
                             const std::shared_ptr<ScaffoldNetworkParams> &params) {
	std::vector<std::shared_ptr<ROMol>> mols{mol};
	ScaffoldNetwork::updateScaffoldNetwork(mols, *network, *params);
}

rust::Vec<rust::String> scaffold_network_nodes(const std::shared_ptr<ScaffoldNetworkClass> &network) {
	rust::Vec<rust::String> nodes;
	nodes.reserve(network->nodes.size());

	for (const auto &node : network->nodes) { nodes.push_back(rust::String(node)); }

	return nodes;
}

rust::Vec<std::uint32_t> scaffold_network_counts(const std::shared_ptr<ScaffoldNetworkClass> &network) {
	rust::Vec<std::uint32_t> counts;
	counts.reserve(network->counts.size());

	for (const auto count : network->counts) { counts.push_back(count); }

	return counts;
}

rust::Vec<std::uint32_t> scaffold_network_mol_counts(const std::shared_ptr<ScaffoldNetworkClass> &network) {
	rust::Vec<std::uint32_t> mol_counts;
	mol_counts.reserve(network->molCounts.size());

	for (const auto mol_count : network->molCounts) { mol_counts.push_back(mol_count); }

	return mol_counts;
}

rust::Vec<std::size_t> scaffold_network_edge_begin_indices(const std::shared_ptr<ScaffoldNetworkClass> &network) {
	rust::Vec<std::size_t> begin_indices;
	begin_indices.reserve(network->edges.size());

	for (const auto &edge : network->edges) { begin_indices.push_back(edge.beginIdx); }

	return begin_indices;
}

rust::Vec<std::size_t> scaffold_network_edge_end_indices(const std::shared_ptr<ScaffoldNetworkClass> &network) {
	rust::Vec<std::size_t> end_indices;
	end_indices.reserve(network->edges.size());

	for (const auto &edge : network->edges) { end_indices.push_back(edge.endIdx); }

	return end_indices;
}

rust::Vec<rust::String> scaffold_network_edge_types(const std::shared_ptr<ScaffoldNetworkClass> &network) {
	rust::Vec<rust::String> types;
	types.reserve(network->edges.size());

	// RDKit ships an operator<< for EdgeType; going through it keeps these strings in step with
	// upstream rather than duplicating the enum names on our side.
	std::ostringstream type_name;
	for (const auto &edge : network->edges) {
		type_name.str("");
		type_name << edge.type;
		types.push_back(rust::String(type_name.str()));
	}

	return types;
}

} // namespace RDKit