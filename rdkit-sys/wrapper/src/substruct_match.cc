#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/Substruct/SubstructMatch.h>

namespace RDKit {
using SubstructMatchItem = std::pair<int, int>;

std::unique_ptr<std::vector<MatchVectType>> substruct_match(const ROMol &mol, const ROMol &other_mol,
                                                            const SubstructMatchParameters &params) {
	return std::make_unique<std::vector<MatchVectType>>(SubstructMatch(mol, other_mol, params));
}

std::unique_ptr<SubstructMatchParameters> new_substruct_match_parameters() {
	return std::unique_ptr<SubstructMatchParameters>(new SubstructMatchParameters());
}

bool get_use_chirality(const SubstructMatchParameters &params) { return params.useChirality; }
bool get_use_enhanced_stereo(const SubstructMatchParameters &params) { return params.useEnhancedStereo; }
bool get_aromatic_matches_conjugated(const SubstructMatchParameters &params) {
	return params.aromaticMatchesConjugated;
}
bool get_use_query_query_matches(const SubstructMatchParameters &params) { return params.useQueryQueryMatches; }
bool get_use_generic_matchers(const SubstructMatchParameters &params) { return params.useGenericMatchers; }
bool get_recursion_possible(const SubstructMatchParameters &params) { return params.recursionPossible; }
bool get_uniquify(const SubstructMatchParameters &params) { return params.uniquify; }

void set_use_chirality(SubstructMatchParameters &params, bool what) { params.useChirality = what; }
void set_use_enhanced_stereo(SubstructMatchParameters &params, bool what) { params.useEnhancedStereo = what; }
void set_aromatic_matches_conjugated(SubstructMatchParameters &params, bool what) {
	params.aromaticMatchesConjugated = what;
}
void set_use_query_query_matches(SubstructMatchParameters &params, bool what) { params.useQueryQueryMatches = what; }
void set_use_generic_matchers(SubstructMatchParameters &params, bool what) { params.useGenericMatchers = what; }
void set_recursion_possible(SubstructMatchParameters &params, bool what) { params.recursionPossible = what; }
void set_uniquify(SubstructMatchParameters &params, bool what) { params.uniquify = what; }
std::unique_ptr<std::vector<SubstructMatchItem>>
substruct_matchvect_type_to_vec_substruct_match_item(const MatchVectType &match_vect) {
	return std::make_unique<std::vector<SubstructMatchItem>>(match_vect.begin(), match_vect.end());
}

int substruct_match_item_query_atom_idx(const SubstructMatchItem &item) { return item.first; }

int substruct_match_item_mol_atom_idx(const SubstructMatchItem &item) { return item.second; }
} // namespace RDKit
