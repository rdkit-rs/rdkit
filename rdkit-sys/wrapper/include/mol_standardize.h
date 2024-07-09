#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolStandardize/Charge.h>
#include <GraphMol/MolStandardize/Tautomer.h>

namespace RDKit {
using TautomerEnumerator       = RDKit::MolStandardize::TautomerEnumerator;
using TautomerEnumeratorResult = RDKit::MolStandardize::TautomerEnumeratorResult;
using CleanupParameters        = MolStandardize::CleanupParameters;
using Uncharger                = MolStandardize::Uncharger;

std::shared_ptr<TautomerEnumerator> tautomer_enumerator();
std::shared_ptr<TautomerEnumeratorResult> tautomer_enumerate(const std::shared_ptr<TautomerEnumerator> &enumerator,
                                                             const std::shared_ptr<ROMol> &mol);
std::shared_ptr<ROMol> tautomer_enumerator_canonicalize(const std::shared_ptr<TautomerEnumerator> &enumerator,
                                                        const std::shared_ptr<ROMol> &mol);

// I couldn't figure out a better way to return a vec of rolmols, check the backlog.md for more info
int tautomer_enumerator_result_tautomers_size(const std::shared_ptr<TautomerEnumeratorResult> &enumerator_result);
std::shared_ptr<ROMol>
tautomer_enumerator_result_tautomers_at(const std::shared_ptr<TautomerEnumeratorResult> &enumerator_result, size_t at);

// rdkit-Release_2022_03_1/Code/MinimalLib/common.h
std::shared_ptr<CleanupParameters> default_cleanup_parameters();

std::shared_ptr<Uncharger> new_uncharger(bool canonical);
std::shared_ptr<ROMol> uncharger_uncharge(const std::shared_ptr<Uncharger> &uncharger,
                                          const std::shared_ptr<ROMol> &mol);

std::shared_ptr<RWMol> fragment_parent(const std::shared_ptr<RWMol> &rw_mol,
                                       const std::shared_ptr<CleanupParameters> &cleanup_params, bool skip_standardize);

std::shared_ptr<RWMol> normalize(const std::shared_ptr<RWMol> &mol,
                                 const std::shared_ptr<CleanupParameters> &cleanup_params);
} // namespace RDKit