#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/ScaffoldNetwork/ScaffoldNetwork.h>

namespace RDKit {
using ScaffoldNetworkParams = ScaffoldNetwork::ScaffoldNetworkParams;

std::unique_ptr<ScaffoldNetworkParams> default_scaffold_network_params() {
	ScaffoldNetworkParams *network_params = new ScaffoldNetworkParams();
	return std::unique_ptr<ScaffoldNetworkParams>(network_params);
}

std::unique_ptr<ScaffoldNetworkParams> new_scaffold_network_params(const rust::Vec<rust::String> &bondBreakersSmarts) {
	std::vector<std::string> cc_vec;

	for (auto smarts : bondBreakersSmarts) { cc_vec.push_back(std::string(smarts)); }

	ScaffoldNetworkParams *network_params = new ScaffoldNetworkParams(cc_vec);
	return std::unique_ptr<ScaffoldNetworkParams>(network_params);
}

void set_include_generic_scaffolds(ScaffoldNetworkParams &params, bool input) {
	params.includeGenericScaffolds = input;
}

void include_generic_bond_scaffolds(ScaffoldNetworkParams &params, bool input) {
	params.includeGenericBondScaffolds = input;
}

void include_scaffolds_without_attachments(ScaffoldNetworkParams &params, bool input) {
	params.includeScaffoldsWithoutAttachments = input;
}

void include_scaffolds_with_attachments(ScaffoldNetworkParams &params, bool input) {
	params.includeScaffoldsWithAttachments = input;
}

void keep_only_first_fragment(ScaffoldNetworkParams &params, bool input) { params.keepOnlyFirstFragment = input; }

void prune_before_fragmenting(ScaffoldNetworkParams &params, bool input) { params.pruneBeforeFragmenting = input; }

void flatten_isotopes(ScaffoldNetworkParams &params, bool input) { params.flattenIsotopes = input; }

void flatten_chirality(ScaffoldNetworkParams &params, bool input) { params.flattenChirality = input; }

void flatten_keep_largest(ScaffoldNetworkParams &params, bool input) { params.flattenKeepLargest = input; }

void collect_mol_counts(ScaffoldNetworkParams &params, bool input) { params.collectMolCounts = input; }

using ScaffoldNetworkClass = ScaffoldNetwork::ScaffoldNetwork;

std::unique_ptr<ScaffoldNetworkClass> default_scaffold_network() {
	ScaffoldNetworkClass *scaffold_network = new ScaffoldNetworkClass();
	return std::unique_ptr<ScaffoldNetworkClass>(scaffold_network);
}

std::unique_ptr<ScaffoldNetworkClass> create_scaffold_network(const ROMol &mol, const ScaffoldNetworkParams &params) {
	std::vector<std::shared_ptr<ROMol>> mols{std::make_shared<ROMol>(mol)};
	ScaffoldNetworkClass *scaffold_network =
	    new ScaffoldNetworkClass(ScaffoldNetwork::createScaffoldNetwork(mols, params));
	return std::unique_ptr<ScaffoldNetworkClass>(scaffold_network);
}

void update_scaffold_network(ScaffoldNetworkClass &network, const ROMol &mol, const ScaffoldNetworkParams &params) {
	std::vector<std::shared_ptr<ROMol>> mols{std::make_shared<ROMol>(mol)};
	ScaffoldNetwork::updateScaffoldNetwork(mols, network, params);
}

rust::Vec<rust::String> scaffold_network_nodes(const ScaffoldNetworkClass &network) {
	rust::Vec<rust::String> nodes;
	nodes.reserve(network.nodes.size());

	for (const auto &node : network.nodes) { nodes.push_back(rust::String(node)); }

	return nodes;
}

rust::Vec<std::uint32_t> scaffold_network_counts(const ScaffoldNetworkClass &network) {
	rust::Vec<std::uint32_t> counts;
	counts.reserve(network.counts.size());

	for (const auto count : network.counts) { counts.push_back(count); }

	return counts;
}

rust::Vec<std::uint32_t> scaffold_network_mol_counts(const ScaffoldNetworkClass &network) {
	rust::Vec<std::uint32_t> mol_counts;
	mol_counts.reserve(network.molCounts.size());

	for (const auto mol_count : network.molCounts) { mol_counts.push_back(mol_count); }

	return mol_counts;
}

rust::Vec<std::size_t> scaffold_network_edge_begin_indices(const ScaffoldNetworkClass &network) {
	rust::Vec<std::size_t> begin_indices;
	begin_indices.reserve(network.edges.size());

	for (const auto &edge : network.edges) { begin_indices.push_back(edge.beginIdx); }

	return begin_indices;
}

rust::Vec<std::size_t> scaffold_network_edge_end_indices(const ScaffoldNetworkClass &network) {
	rust::Vec<std::size_t> end_indices;
	end_indices.reserve(network.edges.size());

	for (const auto &edge : network.edges) { end_indices.push_back(edge.endIdx); }

	return end_indices;
}

rust::Vec<rust::String> scaffold_network_edge_types(const ScaffoldNetworkClass &network) {
	rust::Vec<rust::String> types;
	types.reserve(network.edges.size());

	std::ostringstream type_name;
	for (const auto &edge : network.edges) {
		type_name.str("");
		type_name << edge.type;
		types.push_back(rust::String(type_name.str()));
	}

	return types;
}

}
