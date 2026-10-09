#include "rust/cxx.h"
#include "wrapper/include/owned.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>

namespace RDKit {
using RemoveHsParameters = RDKit::MolOps::RemoveHsParameters;
std::unique_ptr<RemoveHsParameters> new_remove_hs_parameters() {
	return std::unique_ptr<RemoveHsParameters>(new RemoveHsParameters);
}

bool get_remove_degree_zero(const RemoveHsParameters &params) { return params.removeDegreeZero; }
void set_remove_degree_zero(RemoveHsParameters &params, bool what) { params.removeDegreeZero = what; }

bool get_remove_higher_degrees(const RemoveHsParameters &params) { return params.removeHigherDegrees; }
void set_remove_higher_degrees(RemoveHsParameters &params, bool what) { params.removeHigherDegrees = what; }

bool get_remove_only_h_neighbors(const RemoveHsParameters &params) { return params.removeOnlyHNeighbors; }
void set_remove_only_h_neighbors(RemoveHsParameters &params, bool what) { params.removeOnlyHNeighbors = what; }

bool get_remove_isotopes(const RemoveHsParameters &params) { return params.removeIsotopes; }
void set_remove_isotopes(RemoveHsParameters &params, bool what) { params.removeIsotopes = what; }

bool get_remove_and_track_isotopes(const RemoveHsParameters &params) { return params.removeAndTrackIsotopes; }
void set_remove_and_track_isotopes(RemoveHsParameters &params, bool what) { params.removeAndTrackIsotopes = what; }

bool get_remove_dummy_neighbors(const RemoveHsParameters &params) { return params.removeDummyNeighbors; }
void set_remove_dummy_neighbors(RemoveHsParameters &params, bool what) { params.removeDummyNeighbors = what; }

bool get_remove_defining_bond_stereo(const RemoveHsParameters &params) { return params.removeDefiningBondStereo; }
void set_remove_defining_bond_stereo(RemoveHsParameters &params, bool what) { params.removeDefiningBondStereo = what; }

bool get_remove_with_wedged_bond(const RemoveHsParameters &params) { return params.removeWithWedgedBond; }
void set_remove_with_wedged_bond(RemoveHsParameters &params, bool what) { params.removeWithWedgedBond = what; }

bool get_remove_with_query(const RemoveHsParameters &params) { return params.removeWithQuery; }
void set_remove_with_query(RemoveHsParameters &params, bool what) { params.removeWithQuery = what; }

bool get_remove_mapped(const RemoveHsParameters &params) { return params.removeMapped; }
void set_remove_mapped(RemoveHsParameters &params, bool what) { params.removeMapped = what; }

bool get_remove_in_s_groups(const RemoveHsParameters &params) { return params.removeInSGroups; }
void set_remove_in_s_groups(RemoveHsParameters &params, bool what) { params.removeInSGroups = what; }

bool get_show_warnings(const RemoveHsParameters &params) { return params.showWarnings; }
void set_show_warnings(RemoveHsParameters &params, bool what) { params.showWarnings = what; }

bool get_remove_nonimplicit(const RemoveHsParameters &params) { return params.removeNonimplicit; }
void set_remove_nonimplicit(RemoveHsParameters &params, bool what) { params.removeNonimplicit = what; }

bool get_update_explicit_count(const RemoveHsParameters &params) { return params.updateExplicitCount; }
void set_update_explicit_count(RemoveHsParameters &params, bool what) { params.updateExplicitCount = what; }

bool get_remove_hydrides(const RemoveHsParameters &params) { return params.removeHydrides; }
void set_remove_hydrides(RemoveHsParameters &params, bool what) { params.removeHydrides = what; }

bool get_remove_nontetrahedral_neighbors(const RemoveHsParameters &params) {
	return params.removeNontetrahedralNeighbors;
}
void set_remove_nontetrahedral_neighbors(RemoveHsParameters &params, bool what) {
	params.removeNontetrahedralNeighbors = what;
}

std::unique_ptr<ROMol> remove_hs_parameters(const ROMol &mol, const RemoveHsParameters &params, bool sanitize) {
	ROMol *new_mol = MolOps::removeHs(mol, params, sanitize);
	return checked_owned<ROMol>(new_mol);
}

// TODO: do we care about the onlyOnAtoms when adding Hs?
std::unique_ptr<ROMol> add_hs(const ROMol &mol, bool explicit_only, bool add_coords, bool add_residue_info) {
	ROMol *new_mol = MolOps::addHs(mol, explicit_only, add_coords, nullptr, add_residue_info);
	return checked_owned<ROMol>(new_mol);
}

void romol_set_hybridization(ROMol &mol) { MolOps::setHybridization(mol); }

void clean_up(RWMol &rw_mol) { MolOps::cleanUp(rw_mol); }
} // namespace RDKit