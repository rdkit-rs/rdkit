#include "rust/cxx.h"
#include <GraphMol/Descriptors/Property.h>
#include <GraphMol/ROMol.h>

namespace RDKit {
using Descriptors::Properties;

std::shared_ptr<Properties> new_properties() { return std::shared_ptr<Properties>(new Properties()); }

std::unique_ptr<std::vector<std::string>> get_property_names(const std::shared_ptr<Properties> &props) {
	std::vector<std::string> names       = props->getPropertyNames();
	std::vector<std::string> *names_heap = new std::vector<std::string>(names);
	return std::unique_ptr<std::vector<std::string>>(names_heap);
}

std::unique_ptr<std::vector<double>> compute_properties(const std::shared_ptr<Properties> &props,
                                                        const std::shared_ptr<ROMol> &mol) {
	std::vector<double> computed = props->computeProperties(*mol);
	auto computed_heap           = new std::vector<double>(computed);
	return std::unique_ptr<std::vector<double>>(computed_heap);
}
} // namespace RDKit