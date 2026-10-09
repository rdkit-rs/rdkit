#pragma once

#include "rust/cxx.h"
#include <GraphMol/Descriptors/Property.h>

namespace RDKit {
using Descriptors::Properties;

std::unique_ptr<Properties> new_properties();
std::unique_ptr<std::vector<std::string>> get_property_names(const Properties &props);
std::unique_ptr<std::vector<double>> compute_properties(const Properties &props, const ROMol &mol);
} // namespace RDKit