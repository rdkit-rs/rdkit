#pragma once

#include "rust/cxx.h"
#include <GraphMol/FragCatalog/FragCatParams.h>

namespace RDKit {
std::shared_ptr<FragCatParams> init_frat_cat_from_string(const std::string &text);
} // namespace RDKit