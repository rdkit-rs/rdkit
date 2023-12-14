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

    for (auto smarts : bondBreakersSmarts) {
      cc_vec.push_back(std::string(smarts));
    }

    ScaffoldNetworkParams *network_params = new ScaffoldNetworkParams(cc_vec);
    return std::shared_ptr<ScaffoldNetworkParams>(network_params);
  }

  void set_include_generic_scaffolds(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->includeGenericScaffolds = input;
  }

  void set_include_generic_bond_scaffolds(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->includeGenericBondScaffolds = input;
  }

  void set_include_scaffolds_without_attachments(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->includeScaffoldsWithoutAttachments = input;
  }

  void set_include_scaffolds_with_attachments(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->includeScaffoldsWithAttachments = input;
  }

  void set_keep_only_first_fragment(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->keepOnlyFirstFragment = input;
  }

  void set_prune_before_fragmenting(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->pruneBeforeFragmenting = input;
  }

  void set_flatten_isotopes(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->flattenIsotopes = input;
  }

  void set_flatten_chirality(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->flattenChirality = input;
  }

  void set_flatten_keep_largest(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->flattenKeepLargest = input;
  }

  void set_collect_mol_counts(std::shared_ptr<ScaffoldNetworkParams> &params, bool input) {
    params->collectMolCounts = input;
  }

  using ScaffoldNetworkClass = ScaffoldNetwork::ScaffoldNetwork;

  std::shared_ptr<ScaffoldNetworkClass> default_scaffold_network() {
    ScaffoldNetworkClass *scaffold_network = new ScaffoldNetworkClass();
    return std::shared_ptr<ScaffoldNetworkClass>(scaffold_network);
  }

  bool get_include_generic_scaffolds(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->includeGenericScaffolds;
  }
  bool get_include_generic_bond_scaffolds(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->includeGenericBondScaffolds;
  }
  bool get_include_scaffolds_without_attachments(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->includeScaffoldsWithoutAttachments;
  }
  bool get_include_scaffolds_with_attachments(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->includeScaffoldsWithAttachments;
  }
  bool get_keep_only_first_fragment(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->keepOnlyFirstFragment;
  }
  bool get_prune_before_fragmenting(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->pruneBeforeFragmenting;
  }
  bool get_flatten_isotopes(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->flattenIsotopes;
  }
  bool get_flatten_chirality(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->flattenChirality;
  }
  bool get_flatten_keep_largest(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->flattenKeepLargest;
  }
  bool get_collect_mol_counts(const std::shared_ptr<ScaffoldNetworkParams> &params) {
    return params->collectMolCounts;
  }

  void update_scaffold_network(const std::shared_ptr<ROMol> &mol, std::shared_ptr<ScaffoldNetworkClass> &scaffold_network, const std::shared_ptr<ScaffoldNetworkParams> &scaffold_network_params) {
    std::vector<ROMol> mols;
    mols.push_back(*mol);
    ScaffoldNetwork::updateScaffoldNetwork(&mols, *scaffold_network, *scaffold_network_params);
  }

}