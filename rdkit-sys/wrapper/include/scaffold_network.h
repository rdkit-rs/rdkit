#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/ScaffoldNetwork/ScaffoldNetwork.h>

namespace RDKit {
  using ScaffoldNetworkParamsLocal = ScaffoldNetwork::ScaffoldNetworkParams;

  std::shared_ptr<ScaffoldNetworkParamsLocal> default_scaffold_network_params();
  std::shared_ptr<ScaffoldNetworkParamsLocal> new_scaffold_network_params(const rust::Vec<rust::String> &bondBreakersSmarts);

  void set_include_generic_scaffolds(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_include_generic_bond_scaffolds(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_include_scaffolds_without_attachments(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_include_scaffolds_with_attachments(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_keep_only_first_fragment(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_prune_before_fragmenting(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_flatten_isotopes(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_flatten_chirality(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_flatten_keep_largest(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);
  void set_collect_mol_counts(std::shared_ptr<ScaffoldNetworkParamsLocal> &params, bool input);

  bool get_include_generic_scaffolds(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_include_generic_bond_scaffolds(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_include_scaffolds_without_attachments(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_include_scaffolds_with_attachments(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_keep_only_first_fragment(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_prune_before_fragmenting(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_flatten_isotopes(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_flatten_chirality(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_flatten_keep_largest(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);
  bool get_collect_mol_counts(const std::shared_ptr<ScaffoldNetworkParamsLocal> &params);

  using ScaffoldNetworkClass = ScaffoldNetwork::ScaffoldNetwork;

  std::shared_ptr<ScaffoldNetworkClass> default_scaffold_network();

//  std::shared_ptr<ScaffoldNetworkClass> create_scaffold_network(const std::vector<ROMolSharedPtr> &mols, const std::shared_ptr<ScaffoldNetworkParamsLocal> &scaffold_network_params) {
//    throw std::invalid_argument("sup");
//  }

//  pub fn create_scaffold_network(
//      mol: &CxxVector<ROMolSharedPtr>,
//      scaffold_network_params: &SharedPtr<ScaffoldNetworkParamsLocal>,
//  ) -> SharedPtr<ScaffoldNetworkClass>;
}