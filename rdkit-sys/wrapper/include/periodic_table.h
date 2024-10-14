#pragma once

#include "rust/cxx.h"
#include <GraphMol/PeriodicTable.h>

namespace RDKit {
std::unique_ptr<PeriodicTable> get_periodic_table();
rust::String getElementSymbol(unsigned int atomic_number);
rust::String getElementName(unsigned int atomic_number);
const std::vector<int> &getValenceList(unsigned int atomic_number);
} // namespace RDKit
