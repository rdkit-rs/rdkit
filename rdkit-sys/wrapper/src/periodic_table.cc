#include "rust/cxx.h"
#include <rdkit/GraphMol/PeriodicTable.h>

namespace RDKit {
const std::vector<int> &get_valence_list(unsigned int atomic_number) {
	PeriodicTable *pt = RDKit::PeriodicTable::getTable();
	return pt->getValenceList(atomic_number);
}
} // namespace RDKit
