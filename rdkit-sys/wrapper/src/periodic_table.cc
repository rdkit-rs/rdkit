#include "rust/cxx.h"
#include <rdkit/GraphMol/PeriodicTable.h>

namespace RDKit {
std::unique_ptr<PeriodicTable> get_periodic_table() {
	// in rdkit we have the following:
	// PeriodicTable::getTable() return : const PeriodicTable*
	// so we need to take into account that it is a const pointer
	return std::make_unique<PeriodicTable>(*PeriodicTable::getTable());
}
rust::String getElementSymbol(unsigned int atomic_number) {
	PeriodicTable *pt = RDKit::PeriodicTable::getTable();
	return pt->getElementSymbol(atomic_number);
}

rust::String getElementName(unsigned int atomic_number) {
	PeriodicTable *pt = RDKit::PeriodicTable::getTable();
	return pt->getElementName(atomic_number);
}

const std::vector<int> &getValenceList(unsigned int atomic_number) {
	PeriodicTable *pt = RDKit::PeriodicTable::getTable();
	return pt->getValenceList(atomic_number);
}

} // namespace RDKit