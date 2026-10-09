#include "rust/cxx.h"
#include <GraphMol/Descriptors/Property.h>
#include <GraphMol/ROMol.h>

namespace RDKit {
using Descriptors::Properties;

std::unique_ptr<Properties> new_properties() { return std::unique_ptr<Properties>(new Properties()); }

std::unique_ptr<std::vector<std::string>> get_property_names(const Properties &props) {
	return std::make_unique<std::vector<std::string>>(props.getPropertyNames());
}

std::unique_ptr<std::vector<double>> compute_properties(const Properties &props, const ROMol &mol) {
	return std::make_unique<std::vector<double>>(props.computeProperties(mol));
}
} // namespace RDKit