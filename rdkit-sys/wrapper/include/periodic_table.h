#pragma once

#include "rust/cxx.h"
#include <GraphMol/PeriodicTable.h>

namespace RDKit {
unsigned int periodic_table_max_atomic_number(const PeriodicTable &table);
unsigned int periodic_table_row(const PeriodicTable &table, unsigned int atomic_number);
std::unique_ptr<PeriodicTable> get_periodic_table();
rust::String getElementSymbol(unsigned int atomic_number);
rust::String getElementName(unsigned int atomic_number);
std::unique_ptr<std::vector<int>> getValenceList(unsigned int atomic_number);
} // namespace RDKit
