#include "rust/cxx.h"
#include <GraphMol/FragCatalog/FragCatParams.h>
// #include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>
#include <memory>

namespace RDKit {
std::shared_ptr<FragCatParams> init_frat_cat_from_string(const std::string &text) {
	return std::shared_ptr<FragCatParams>(new FragCatParams(text));
}

// here we need to do something
} // namespace RDKit