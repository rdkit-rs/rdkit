#include "rust/cxx.h"
#include <rdkit/GraphMol/PeriodicTable.h>

namespace RDKit {
std::unique_ptr<PeriodicTable> get_periodic_table() {
	// in rdkit we have the following:
	// PeriodicTable::getTable() return : const PeriodicTable*
	// so we need to take into account that it is a const pointer
	return std::make_unique<PeriodicTable>(*PeriodicTable::getTable());
}

const std::vector<int> &get_valence_list(unsigned int atomic_number) {
	PeriodicTable *pt = RDKit::PeriodicTable::getTable();
	return pt->getValenceList(atomic_number);
}

// double get_most_common_isotope_mass(const std::string &symbol) {
// PeriodicTable *pt = RDKit::PeriodicTable::getTable();
// return pt->getMostCommonIsotopeMass(symbol);
// }
} // namespace RDKit