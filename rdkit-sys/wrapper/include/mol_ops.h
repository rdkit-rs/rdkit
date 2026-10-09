#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>

namespace RDKit {
//  pub fn new_remove_hs_parameters() -> SharedPtr<RemoveHsParameters>;
using RemoveHsParameters = RDKit::MolOps::RemoveHsParameters;
std::unique_ptr<RemoveHsParameters> new_remove_hs_parameters();

bool get_remove_degree_zero(const RemoveHsParameters &params);
void set_remove_degree_zero(RemoveHsParameters &params, bool what);

bool get_remove_higher_degrees(const RemoveHsParameters &params);
void set_remove_higher_degrees(RemoveHsParameters &params, bool what);

bool get_remove_only_h_neighbors(const RemoveHsParameters &params);
void set_remove_only_h_neighbors(RemoveHsParameters &params, bool what);

bool get_remove_isotopes(const RemoveHsParameters &params);
void set_remove_isotopes(RemoveHsParameters &params, bool what);

bool get_remove_and_track_isotopes(const RemoveHsParameters &params);
void set_remove_and_track_isotopes(RemoveHsParameters &params, bool what);

bool get_remove_dummy_neighbors(const RemoveHsParameters &params);
void set_remove_dummy_neighbors(RemoveHsParameters &params, bool what);

bool get_remove_defining_bond_stereo(const RemoveHsParameters &params);
void set_remove_defining_bond_stereo(RemoveHsParameters &params, bool what);

bool get_remove_with_wedged_bond(const RemoveHsParameters &params);
void set_remove_with_wedged_bond(RemoveHsParameters &params, bool what);

bool get_remove_with_query(const RemoveHsParameters &params);
void set_remove_with_query(RemoveHsParameters &params, bool what);

bool get_remove_mapped(const RemoveHsParameters &params);
void set_remove_mapped(RemoveHsParameters &params, bool what);

bool get_remove_in_s_groups(const RemoveHsParameters &params);
void set_remove_in_s_groups(RemoveHsParameters &params, bool what);

bool get_show_warnings(const RemoveHsParameters &params);
void set_show_warnings(RemoveHsParameters &params, bool what);

bool get_remove_nonimplicit(const RemoveHsParameters &params);
void set_remove_nonimplicit(RemoveHsParameters &params, bool what);

bool get_update_explicit_count(const RemoveHsParameters &params);
void set_update_explicit_count(RemoveHsParameters &params, bool what);

bool get_remove_hydrides(const RemoveHsParameters &params);
void set_remove_hydrides(RemoveHsParameters &params, bool what);

bool get_remove_nontetrahedral_neighbors(const RemoveHsParameters &params);
void set_remove_nontetrahedral_neighbors(RemoveHsParameters &params, bool what);

std::unique_ptr<ROMol> remove_hs_parameters(const ROMol &mol, const RemoveHsParameters &params, bool sanitize);

std::unique_ptr<ROMol> add_hs(const ROMol &mol, bool explicit_only, bool add_coords, bool add_residue_info);

void romol_set_hybridization(ROMol &mol);

// pub fn clean_up(rw_mol: &mut SharedPtr<RWMol>)
void clean_up(RWMol &rw_mol);
} // namespace RDKit
