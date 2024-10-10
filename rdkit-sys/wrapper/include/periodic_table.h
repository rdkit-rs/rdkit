#pragma once

#include "rust/cxx.h"
#include <GraphMol/PeriodicTable.h>

namespace RDKit {
const std::vector<int> &get_valence_list(unsigned int atomic_number);
// double get_most_common_isotope_mass(const std::string &symbol);
} // namespace RDKit
