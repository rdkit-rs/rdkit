#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>

namespace RDKit {
//  pub fn new_remove_hs_parameters() -> SharedPtr<RemoveHsParameters>;
using RemoveHsParameters = RDKit::MolOps::RemoveHsParameters;
std::shared_ptr<RemoveHsParameters> new_remove_hs_parameters();

bool get_remove_degree_zero(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_degree_zero(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_higher_degrees(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_higher_degrees(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_only_h_neighbors(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_only_h_neighbors(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_isotopes(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_isotopes(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_and_track_isotopes(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_and_track_isotopes(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_dummy_neighbors(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_dummy_neighbors(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_defining_bond_stereo(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_defining_bond_stereo(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_with_wedged_bond(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_with_wedged_bond(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_with_query(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_with_query(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_mapped(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_mapped(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_in_s_groups(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_in_s_groups(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_show_warnings(const std::shared_ptr<RemoveHsParameters> &params);
void set_show_warnings(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_nonimplicit(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_nonimplicit(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_update_explicit_count(const std::shared_ptr<RemoveHsParameters> &params);
void set_update_explicit_count(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_hydrides(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_hydrides(std::shared_ptr<RemoveHsParameters> &params, bool what);

bool get_remove_nontetrahedral_neighbors(const std::shared_ptr<RemoveHsParameters> &params);
void set_remove_nontetrahedral_neighbors(std::shared_ptr<RemoveHsParameters> &params, bool what);

std::shared_ptr<ROMol> remove_hs_parameters(const std::shared_ptr<ROMol> &mol,
                                            const std::shared_ptr<RemoveHsParameters> &params, bool sanitize);

std::shared_ptr<ROMol> add_hs(const std::shared_ptr<ROMol> &mol, bool explicit_only, bool add_coords,
                              bool add_residue_info);

void romol_set_hybridization(std::shared_ptr<ROMol> &mol);

// pub fn clean_up(rw_mol: &mut SharedPtr<RWMol>)
void clean_up(std::shared_ptr<RWMol> &rw_mol);
} // namespace RDKit
