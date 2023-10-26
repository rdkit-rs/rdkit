#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/Substruct/SubstructMatch.h>

namespace RDKit {
    std::unique_ptr<std::vector<MatchVectType>> substruct_match(const std::shared_ptr<ROMol> &mol, const std::shared_ptr<ROMol> &other_mol, const std::shared_ptr<SubstructMatchParameters> &params);

    bool substruct_match_as_bool(const std::shared_ptr<ROMol> &mol, const std::shared_ptr<ROMol> &other_mol, const std::shared_ptr<SubstructMatchParameters> &params);

    std::shared_ptr<SubstructMatchParameters> new_substruct_match_parameters();
}