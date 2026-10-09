#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/Substruct/SubstructMatch.h>

namespace RDKit {
using SubstructMatchItem = std::pair<int, int>;
using MatchVectType      = MatchVectType;

std::unique_ptr<std::vector<MatchVectType>> substruct_match(const ROMol &mol, const ROMol &other_mol,
                                                            const SubstructMatchParameters &params);

std::unique_ptr<SubstructMatchParameters> new_substruct_match_parameters();
bool get_use_chirality(const SubstructMatchParameters &params);
bool get_use_enhanced_stereo(const SubstructMatchParameters &params);
bool get_aromatic_matches_conjugated(const SubstructMatchParameters &params);
bool get_use_query_query_matches(const SubstructMatchParameters &params);
bool get_use_generic_matchers(const SubstructMatchParameters &params);
bool get_recursion_possible(const SubstructMatchParameters &params);
bool get_uniquify(const SubstructMatchParameters &params);
void set_use_chirality(SubstructMatchParameters &params, bool what);
void set_use_enhanced_stereo(SubstructMatchParameters &params, bool what);
void set_aromatic_matches_conjugated(SubstructMatchParameters &params, bool what);
void set_use_query_query_matches(SubstructMatchParameters &params, bool what);
void set_use_generic_matchers(SubstructMatchParameters &params, bool what);
void set_recursion_possible(SubstructMatchParameters &params, bool what);
void set_uniquify(SubstructMatchParameters &params, bool what);

std::unique_ptr<std::vector<SubstructMatchItem>>
substruct_matchvect_type_to_vec_substruct_match_item(const MatchVectType &match_vect);
int substruct_match_item_query_atom_idx(const SubstructMatchItem &item);
int substruct_match_item_mol_atom_idx(const SubstructMatchItem &item);
} // namespace RDKit