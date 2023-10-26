#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/Substruct/SubstructMatch.h>

namespace RDKit {
    std::unique_ptr<std::vector<MatchVectType>> substruct_match(const std::shared_ptr<ROMol> &mol, const std::shared_ptr<ROMol> &other_mol, const std::shared_ptr<SubstructMatchParameters> &params) {
        std::vector<MatchVectType> match = SubstructMatch(*mol, *other_mol, *params);
        std::vector<MatchVectType> *heap_match = new std::vector<MatchVectType>(match);
        return std::unique_ptr<std::vector<MatchVectType>>(heap_match);
    }

    bool substruct_match_as_bool(const std::shared_ptr<ROMol> &mol, const std::shared_ptr<ROMol> &other_mol, const std::shared_ptr<SubstructMatchParameters> &params) {
        std::vector<MatchVectType> match = SubstructMatch(*mol, *other_mol, *params);

        return match.size() > 0;
    }

    std::shared_ptr<SubstructMatchParameters> new_substruct_match_parameters() {
        return std::shared_ptr<SubstructMatchParameters>(new SubstructMatchParameters());
    }
}
