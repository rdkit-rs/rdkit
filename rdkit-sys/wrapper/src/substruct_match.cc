#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/Substruct/SubstructMatch.h>

#include <iostream>

namespace RDKit {
    std::unique_ptr<std::vector<MatchVectType>> substruct_match(const std::shared_ptr<ROMol> &mol, const std::shared_ptr<ROMol> &other_mol, const std::shared_ptr<SubstructMatchParameters> &params) {
        std::vector<MatchVectType> match = SubstructMatch(*mol, *other_mol, *params);
        std::vector<MatchVectType> *heap_match = new std::vector<MatchVectType>(match);

        //for (auto& m : *heap_match) {      // reference avoids copying element
        //  for (auto &p : m) {
        //    std::cout << "p first: " << p.first << " p second: " << p.second << std::endl;
        //  }
        //}

        return std::unique_ptr<std::vector<MatchVectType>>(heap_match);
    }

    rust::Vec<rust::Vec<i32>> substruct_match_gen_2(const std::shared_ptr<ROMol> &mol, const std::shared_ptr<ROMol> &other_mol, const std::shared_ptr<SubstructMatchParameters> &params) {
      auto vec: rust::Vec<rust::Vec<i32>> = rust::Vec();
      return vec;
    }

    std::shared_ptr<SubstructMatchParameters> new_substruct_match_parameters() {
        return std::shared_ptr<SubstructMatchParameters>(new SubstructMatchParameters());
    }

    bool get_use_chirality(const std::shared_ptr<SubstructMatchParameters> &params) {
        return params->useChirality;
    }
    bool get_use_enhanced_stereo(const std::shared_ptr<SubstructMatchParameters> &params) {
        return params->useEnhancedStereo;
    }
    bool get_aromatic_matches_conjugated(const std::shared_ptr<SubstructMatchParameters> &params) {
        return params->aromaticMatchesConjugated;
    }
    bool get_use_query_query_matches(const std::shared_ptr<SubstructMatchParameters> &params) {
        return params->useQueryQueryMatches;
    }
    bool get_use_generic_matchers(const std::shared_ptr<SubstructMatchParameters> &params) {
        return params->useGenericMatchers;
    }
    bool get_recursion_possible(const std::shared_ptr<SubstructMatchParameters> &params) {
        return params->recursionPossible;
    }
    bool get_uniquify(const std::shared_ptr<SubstructMatchParameters> &params) {
        return params->uniquify;
    }

    void set_use_chirality(std::shared_ptr<SubstructMatchParameters> &params, bool what) {
        params->useChirality = what;
    }
    void set_use_enhanced_stereo(std::shared_ptr<SubstructMatchParameters> &params, bool what) {
        params->useEnhancedStereo = what;
    }
    void set_aromatic_matches_conjugated(std::shared_ptr<SubstructMatchParameters> &params, bool what) {
        params->aromaticMatchesConjugated = what;
    }
    void set_use_query_query_matches(std::shared_ptr<SubstructMatchParameters> &params, bool what) {
        params->useQueryQueryMatches = what;
    }
    void set_use_generic_matchers(std::shared_ptr<SubstructMatchParameters> &params, bool what) {
        params->useGenericMatchers = what;
    }
    void set_recursion_possible(std::shared_ptr<SubstructMatchParameters> &params, bool what) {
        params->recursionPossible = what;
    }
    void set_uniquify(std::shared_ptr<SubstructMatchParameters> &params, bool what) {
        params->uniquify = what;
    }

    int substruct_match_item_query_atom_idx(const std::pair<int, int> &item) {
        return item.first;
    }

    int substruct_match_item_mol_atom_idx(const std::pair<int, int> &item) {
        return item.second;
    }
}
