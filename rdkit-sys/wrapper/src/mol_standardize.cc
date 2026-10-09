#include "wrapper/include/owned.h"
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

std::unique_ptr<TautomerEnumerator> tautomer_enumerator() {
	TautomerEnumerator *enumerator = new MolStandardize::TautomerEnumerator();
	return std::unique_ptr<TautomerEnumerator>(enumerator);
}

std::unique_ptr<TautomerEnumeratorResult> tautomer_enumerate(const TautomerEnumerator &enumerator, const ROMol &mol) {
	TautomerEnumeratorResult stacked_enumerator = enumerator.enumerate(mol);
	TautomerEnumeratorResult *heaped_enumerator = new TautomerEnumeratorResult(stacked_enumerator);
	return std::unique_ptr<TautomerEnumeratorResult>(heaped_enumerator);
}

std::unique_ptr<ROMol> tautomer_enumerator_canonicalize(const TautomerEnumerator &enumerator, const ROMol &mol) {
	ROMol *canonical_mol = enumerator.canonicalize(mol);
	return checked_owned<ROMol>(canonical_mol);
}

size_t tautomer_enumerator_result_tautomers_size(const TautomerEnumeratorResult &enumerator_result) {
	return enumerator_result.size();
}

std::unique_ptr<ROMol> tautomer_enumerator_result_tautomers_at(const TautomerEnumeratorResult &enumerator_result,
                                                               size_t at) {
	if (at >= enumerator_result.size()) { throw std::out_of_range("tautomer index"); }
	RDKit::ROMOL_SPTR sptr = enumerator_result.at(at);
	if (!sptr) { throw std::runtime_error("missing tautomer"); }
	return checked_owned<ROMol>(new ROMol(*sptr));
}

std::unique_ptr<CleanupParameters> default_cleanup_parameters() {
	CleanupParameters *heap_cp = new CleanupParameters(MolStandardize::defaultCleanupParameters);
	return std::unique_ptr<CleanupParameters>(heap_cp);
}

std::unique_ptr<Uncharger> new_uncharger(bool canonical) {
	return std::unique_ptr<Uncharger>(new Uncharger(canonical));
}

std::unique_ptr<ROMol> uncharger_uncharge(Uncharger &uncharger, const ROMol &mol) {
	ROMol *return_mol = uncharger.uncharge(mol);
	return checked_owned<ROMol>(return_mol);
}

std::unique_ptr<RWMol> fragment_parent(const RWMol &rw_mol, const CleanupParameters &cleanup_params,
                                       bool skip_standardize) {
	RWMol *parent = RDKit::MolStandardize::fragmentParent(rw_mol, cleanup_params, skip_standardize);
	return checked_owned<RWMol>(parent);
}

std::unique_ptr<RWMol> normalize(const RWMol &mol, const CleanupParameters &cleanup_params) {
	RWMol *normalized = RDKit::MolStandardize::normalize(&mol, cleanup_params);
	return checked_owned<RWMol>(normalized);
}
} // namespace RDKit