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
} // namespace RDKit