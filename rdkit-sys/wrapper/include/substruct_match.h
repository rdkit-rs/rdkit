#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/Substruct/SubstructMatch.h>

namespace RDKit {
    std::unique_ptr<std::vector<MatchVectType>> substruct_match(const std::shared_ptr<ROMol> &mol, const std::shared_ptr<ROMol> &other_mol, const std::shared_ptr<SubstructMatchParameters> &params);

    rust::Vec<rust::Vec<i32>> substruct_match_gen_2(const std::shared_ptr<ROMol> &mol, const std::shared_ptr<ROMol> &other_mol, const std::shared_ptr<SubstructMatchParameters> &params);

    std::shared_ptr<SubstructMatchParameters> new_substruct_match_parameters();
    bool get_use_chirality(const std::shared_ptr<SubstructMatchParameters> &params);
    bool get_use_enhanced_stereo(const std::shared_ptr<SubstructMatchParameters> &params);
    bool get_aromatic_matches_conjugated(const std::shared_ptr<SubstructMatchParameters> &params);
    bool get_use_query_query_matches(const std::shared_ptr<SubstructMatchParameters> &params);
    bool get_use_generic_matchers(const std::shared_ptr<SubstructMatchParameters> &params);
    bool get_recursion_possible(const std::shared_ptr<SubstructMatchParameters> &params);
    bool get_uniquify(const std::shared_ptr<SubstructMatchParameters> &params);
    void set_use_chirality(std::shared_ptr<SubstructMatchParameters> &params, bool what);
    void set_use_enhanced_stereo(std::shared_ptr<SubstructMatchParameters> &params, bool what);
    void set_aromatic_matches_conjugated(std::shared_ptr<SubstructMatchParameters> &params, bool what);
    void set_use_query_query_matches(std::shared_ptr<SubstructMatchParameters> &params, bool what);
    void set_use_generic_matchers(std::shared_ptr<SubstructMatchParameters> &params, bool what);
    void set_recursion_possible(std::shared_ptr<SubstructMatchParameters> &params, bool what);
    void set_uniquify(std::shared_ptr<SubstructMatchParameters> &params, bool what);

    int substruct_match_item_query_atom_idx(const std::pair<int, int> &item);
    int substruct_match_item_mol_atom_idx(const std::pair<int, int> &item);
}