#include "rust/cxx.h"
#include <GraphMol/Descriptors/Crippen.h>
#include <GraphMol/Descriptors/Lipinski.h>
#include <GraphMol/Descriptors/MolDescriptors.h>
#include <GraphMol/Descriptors/MolSurf.h>
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
double calc_exact_mw(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcExactMW(*mol); }

double calc_amw(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcAMW(*mol); }

rust::String calc_mol_formula(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcMolFormula(*mol); }

unsigned int calc_num_heavy_atoms(const std::shared_ptr<ROMol> &mol) { return mol->getNumHeavyAtoms(); }

double calc_fraction_csp3(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcFractionCSP3(*mol); }

double calc_labute_asa(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcLabuteASA(*mol); }

double calc_tpsa(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcTPSA(*mol); }

double calc_clog_p(const std::shared_ptr<ROMol> &mol) {
	double logp, mr;
	Descriptors::calcCrippenDescriptors(*mol, logp, mr);
	return logp;
}

unsigned int calc_num_hbd(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcNumHBD(*mol); }

unsigned int calc_num_hba(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcNumHBA(*mol); }

unsigned int calc_num_rotatable_bonds(const std::shared_ptr<ROMol> &mol) {
	return Descriptors::calcNumRotatableBonds(*mol);
}

unsigned int calc_num_amide_bonds(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcNumAmideBonds(*mol); }

unsigned int calc_num_heteroatoms(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcNumHeteroatoms(*mol); }

unsigned int calc_num_aromatic_rings(const std::shared_ptr<ROMol> &mol) {
	return Descriptors::calcNumAromaticRings(*mol);
}

unsigned int calc_num_aliphatic_rings(const std::shared_ptr<ROMol> &mol) {
	return Descriptors::calcNumAliphaticRings(*mol);
}

unsigned int calc_num_saturated_rings(const std::shared_ptr<ROMol> &mol) {
	return Descriptors::calcNumSaturatedRings(*mol);
}

unsigned int calc_num_heterocycles(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcNumHeterocycles(*mol); }

unsigned int calc_num_aromatic_heterocycles(const std::shared_ptr<ROMol> &mol) {
	return Descriptors::calcNumAromaticHeterocycles(*mol);
}

unsigned int calc_num_spiro_atoms(const std::shared_ptr<ROMol> &mol) { return Descriptors::calcNumSpiroAtoms(*mol); }

unsigned int calc_num_bridgehead_atoms(const std::shared_ptr<ROMol> &mol) {
	return Descriptors::calcNumBridgeheadAtoms(*mol);
}
} // namespace RDKit