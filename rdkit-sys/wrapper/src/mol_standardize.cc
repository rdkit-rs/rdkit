#include <vector>

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolStandardize/Charge.h>
#include <GraphMol/MolStandardize/Tautomer.h>

namespace RDKit {
using ROMol                    = RDKit::ROMol;
using TautomerCatalogParams    = MolStandardize::TautomerCatalogParams;
using TautomerCatalog          = MolStandardize::TautomerCatalog;
using TautomerEnumerator       = MolStandardize::TautomerEnumerator;
using TautomerEnumeratorResult = MolStandardize::TautomerEnumeratorResult;
using CleanupParameters        = MolStandardize::CleanupParameters;
using Uncharger                = MolStandardize::Uncharger;

std::shared_ptr<TautomerEnumerator> tautomer_enumerator() {
	TautomerEnumerator *enumerator = new MolStandardize::TautomerEnumerator();
	return std::shared_ptr<TautomerEnumerator>(enumerator);
}

std::shared_ptr<TautomerEnumeratorResult> tautomer_enumerate(const std::shared_ptr<TautomerEnumerator> &enumerator,
                                                             const std::shared_ptr<ROMol> &mol) {
	TautomerEnumeratorResult stacked_enumerator = enumerator->enumerate(*mol);
	TautomerEnumeratorResult *heaped_enumerator = new TautomerEnumeratorResult(stacked_enumerator);
	return std::shared_ptr<TautomerEnumeratorResult>(heaped_enumerator);
}

std::shared_ptr<ROMol> tautomer_enumerator_canonicalize(const std::shared_ptr<TautomerEnumerator> &enumerator,
                                                        const std::shared_ptr<ROMol> &mol) {
	ROMol *canonical_mol = enumerator->canonicalize(*mol);
	return std::shared_ptr<ROMol>(canonical_mol);
}

int tautomer_enumerator_result_tautomers_size(const std::shared_ptr<TautomerEnumeratorResult> &enumerator_result) {
	return enumerator_result->size();
}

std::shared_ptr<ROMol>
tautomer_enumerator_result_tautomers_at(const std::shared_ptr<TautomerEnumeratorResult> &enumerator_result, size_t at) {
	RDKit::ROMOL_SPTR sptr = enumerator_result->at(at);
	return std::shared_ptr<ROMol>(new ROMol(*sptr));
}

std::shared_ptr<CleanupParameters> default_cleanup_parameters() {
	CleanupParameters *heap_cp = new CleanupParameters(MolStandardize::defaultCleanupParameters);
	return std::shared_ptr<CleanupParameters>(heap_cp);
}

std::shared_ptr<Uncharger> new_uncharger(bool canonical) {
	return std::shared_ptr<Uncharger>(new Uncharger(canonical));
}

std::shared_ptr<ROMol> uncharger_uncharge(const std::shared_ptr<Uncharger> &uncharger,
                                          const std::shared_ptr<ROMol> &mol) {
	ROMol *return_mol = uncharger->uncharge(*mol);
	return std::shared_ptr<ROMol>(return_mol);
}

std::shared_ptr<RWMol> fragment_parent(const std::shared_ptr<RWMol> &rw_mol,
                                       const std::shared_ptr<CleanupParameters> &cleanup_params,
                                       bool skip_standardize) {
	RWMol *parent = RDKit::MolStandardize::fragmentParent(*rw_mol, *cleanup_params, skip_standardize);
	return std::shared_ptr<RWMol>(parent);
}

std::shared_ptr<RWMol> normalize(const std::shared_ptr<RWMol> &mol,
                                 const std::shared_ptr<CleanupParameters> &cleanup_params) {
	RWMol *normalized = RDKit::MolStandardize::normalize(&*mol, *cleanup_params);
	return std::shared_ptr<RWMol>(normalized);
}
} // namespace RDKit