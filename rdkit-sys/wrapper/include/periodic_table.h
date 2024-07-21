#pragma once

#include "rust/cxx.h"

namespace RDKit {
const std::vector<int> &get_valence_list(unsigned int atomic_number);
}