#include "rust/cxx.h"
#include <GraphMol/PeriodicTable.h>
#include <algorithm>
#include <array>
#include <stdexcept>

namespace RDKit {
template<typename Table> unsigned int table_max_atomic_number(const Table &table) {
	if constexpr (requires { table.getMaxAtomicNumber(); }) {
		return table.getMaxAtomicNumber();
	} else {
		return 118;
	}
}

template<typename Table> unsigned int table_row(const Table &table, unsigned int atomic_number) {
	if constexpr (requires { table.getRow(atomic_number); }) {
		return table.getRow(atomic_number);
	} else {
		constexpr std::array<unsigned int, 8> row_ends{0, 2, 10, 18, 36, 54, 86, 118};
		if (atomic_number > row_ends.back()) { throw std::out_of_range("atomic number"); }
		return std::lower_bound(row_ends.begin(), row_ends.end(), atomic_number) - row_ends.begin();
	}
}

unsigned int periodic_table_max_atomic_number(const PeriodicTable &table) { return table_max_atomic_number(table); }
unsigned int periodic_table_row(const PeriodicTable &table, unsigned int atomic_number) {
	return table_row(table, atomic_number);
}

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

std::unique_ptr<std::vector<int>> getValenceList(unsigned int atomic_number) {
	PeriodicTable *pt = RDKit::PeriodicTable::getTable();
	return std::make_unique<std::vector<int>>(pt->getValenceList(atomic_number));
}

} // namespace RDKit