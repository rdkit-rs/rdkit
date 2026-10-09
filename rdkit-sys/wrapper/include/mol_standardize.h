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

std::unique_ptr<TautomerEnumerator> tautomer_enumerator();
std::unique_ptr<TautomerEnumeratorResult> tautomer_enumerate(const TautomerEnumerator &enumerator, const ROMol &mol);
std::unique_ptr<ROMol> tautomer_enumerator_canonicalize(const TautomerEnumerator &enumerator, const ROMol &mol);

// I couldn't figure out a better way to return a vec of rolmols, check the backlog.md for more info
size_t tautomer_enumerator_result_tautomers_size(const TautomerEnumeratorResult &enumerator_result);
std::unique_ptr<ROMol> tautomer_enumerator_result_tautomers_at(const TautomerEnumeratorResult &enumerator_result,
                                                               size_t at);

// rdkit-Release_2022_03_1/Code/MinimalLib/common.h
std::unique_ptr<CleanupParameters> default_cleanup_parameters();

std::unique_ptr<Uncharger> new_uncharger(bool canonical);
std::unique_ptr<ROMol> uncharger_uncharge(Uncharger &uncharger, const ROMol &mol);

std::unique_ptr<RWMol> fragment_parent(const RWMol &rw_mol, const CleanupParameters &cleanup_params,
                                       bool skip_standardize);

std::unique_ptr<RWMol> normalize(const RWMol &mol, const CleanupParameters &cleanup_params);
} // namespace RDKit