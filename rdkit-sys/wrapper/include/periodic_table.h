#pragma once

#include "rust/cxx.h"
#include <GraphMol/PeriodicTable.h>

namespace RDKit {
class RustPeriodicTable {
public:
	double getAtomicWeight(unsigned int atomic_number) const {
		return PeriodicTable::getTable()->getAtomicWeight(atomic_number);
	}
	int getAtomicNumber(const std::string &atom) const { return PeriodicTable::getTable()->getAtomicNumber(atom); }
	double getRvdw(unsigned int atomic_number) const { return PeriodicTable::getTable()->getRvdw(atomic_number); }
	double getRcovalent(unsigned int atomic_number) const {
		return PeriodicTable::getTable()->getRcovalent(atomic_number);
	}
	double getRb0(unsigned int atomic_number) const { return PeriodicTable::getTable()->getRb0(atomic_number); }
	int getDefaultValence(unsigned int atomic_number) const {
		return PeriodicTable::getTable()->getDefaultValence(atomic_number);
	}
	int getNouterElecs(unsigned int atomic_number) const {
		return PeriodicTable::getTable()->getNouterElecs(atomic_number);
	}
	int getMostCommonIsotope(unsigned int atomic_number) const {
		return PeriodicTable::getTable()->getMostCommonIsotope(atomic_number);
	}
	double getMostCommonIsotopeMass(const std::string &atom) const {
		return PeriodicTable::getTable()->getMostCommonIsotopeMass(atom);
	}
	double getMassForIsotope(unsigned int atomic_number, unsigned int isotope) const {
		return PeriodicTable::getTable()->getMassForIsotope(atomic_number, isotope);
	}
	double getAbundanceForIsotope(unsigned int atomic_number, unsigned int isotope) const {
		return PeriodicTable::getTable()->getAbundanceForIsotope(atomic_number, isotope);
	}
	bool moreElectroNegative(unsigned int atomic_number1, unsigned int atomic_number2) const {
		return PeriodicTable::getTable()->moreElectroNegative(atomic_number1, atomic_number2);
	}
};
unsigned int native_periodic_table_max_atomic_number();

unsigned int periodic_table_max_atomic_number(const RustPeriodicTable &table);
unsigned int periodic_table_row(const RustPeriodicTable &table, unsigned int atomic_number);
std::unique_ptr<RustPeriodicTable> get_periodic_table();
rust::String getElementSymbol(unsigned int atomic_number);
rust::String getElementName(unsigned int atomic_number);
std::unique_ptr<std::vector<int>> getValenceList(unsigned int atomic_number);
} // namespace RDKit
