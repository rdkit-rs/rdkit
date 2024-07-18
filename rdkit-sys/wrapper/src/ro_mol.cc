#include "rust/cxx.h"
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/FileParsers/FileWriters.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>
#include <GraphMol/MolStandardize/Tautomer.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>

#include <iostream>

namespace RDKit {
using ExplicitBitVect = ::ExplicitBitVect;

std::shared_ptr<ROMol> copy_mol(const std::shared_ptr<ROMol> &mol) { return std::shared_ptr<ROMol>(new ROMol(*mol)); }

std::shared_ptr<ROMol> smiles_to_mol(const std::string &smiles) {
	ROMol *mol = SmilesToMol(smiles);

	return std::shared_ptr<ROMol>(mol);
}

rust::String mol_to_smiles(const std::shared_ptr<ROMol> &mol) { return MolToSmiles(*mol); }

rust::String mol_to_molblock(const std::shared_ptr<ROMol> &mol) { return MolToMolBlock(*mol); }

std::shared_ptr<ROMol> smiles_to_mol_with_params(const std::string &smiles,
                                                 const std::shared_ptr<SmilesParserParams> &params) {
	ROMol *mol = SmilesToMol(smiles, *params);

	return std::shared_ptr<ROMol>(mol);
}
std::shared_ptr<SmilesParserParams> new_smiles_parser_params() {
	return std::shared_ptr<SmilesParserParams>(new SmilesParserParams());
}
void smiles_parser_params_set_sanitize(const std::shared_ptr<SmilesParserParams> &params, bool sanitize) {
	params->sanitize = sanitize;
}
bool smiles_parser_params_get_sanitize(const std::shared_ptr<SmilesParserParams> &params) { return params->sanitize; }

using MolSanitizeExceptionUniquePtr = std::unique_ptr<MolSanitizeException>;
std::unique_ptr<std::vector<MolSanitizeExceptionUniquePtr>>
detect_chemistry_problems(const std::shared_ptr<ROMol> &mol) {
	std::vector<MolSanitizeExceptionUniquePtr> exceptions         = RDKit::MolOps::detectChemistryProblems(*mol);
	std::vector<MolSanitizeExceptionUniquePtr> *heaped_exceptions = new std::vector<MolSanitizeExceptionUniquePtr>();

	auto s = exceptions.size();
	heaped_exceptions->reserve(s);
	for (int i = 0; i < s; i++) { heaped_exceptions->push_back(std::move(exceptions[i])); }

	return std::unique_ptr<std::vector<MolSanitizeExceptionUniquePtr>>(heaped_exceptions);
}

rust::String mol_sanitize_exception_type(const MolSanitizeExceptionUniquePtr &mol_except) {
	return mol_except->getType();
}

unsigned int atom_sanitize_exception_get_atom_idx(const MolSanitizeExceptionUniquePtr &mol_except) {
	MolSanitizeException *mol_except_ptr            = mol_except.get();
	AtomSanitizeException *atom_sanitize_except_ptr = (AtomSanitizeException *)mol_except_ptr;

	return atom_sanitize_except_ptr->getAtomIdx();
}

unsigned int get_num_atoms(const std::shared_ptr<ROMol> &mol, bool only_explicit) {
	return mol->getNumAtoms(only_explicit);
}

Atom &get_atom_with_idx(std::shared_ptr<ROMol> &mol, unsigned int idx) { return *mol->getAtomWithIdx(idx); }

rust::String get_symbol(const Atom &atom) { return atom.getSymbol(); }

bool get_is_aromatic(const Atom &atom) { return atom.getIsAromatic(); }
int get_atomic_num(const Atom &atom) { return atom.getAtomicNum(); }
int get_formal_charge(const Atom &atom) { return atom.getFormalCharge(); }
unsigned int get_total_num_hs(const Atom &atom) { return atom.getTotalNumHs(); }
unsigned int get_total_valence(const Atom &atom) { return atom.getTotalValence(); }
void set_formal_charge(Atom &atom, int what) { atom.setFormalCharge(what); }
void set_num_explicit_hs(Atom &atom, int what) { atom.setNumExplicitHs(what); }
void atom_update_property_cache(Atom &atom, bool strict) { atom.updatePropertyCache(strict); }

using HybridizationType = Atom::HybridizationType;
void atom_set_hybridization(Atom &atom, HybridizationType what) { atom.setHybridization(what); }
HybridizationType atom_get_hybridization(const Atom &atom) { return atom.getHybridization(); }

void ro_mol_update_property_cache(std::shared_ptr<ROMol> &mol, bool strict) { mol->updatePropertyCache(strict); }

// We add the different properties of and an atom to the Atom struct
void set_int_prop(Atom &atom, const std::string &key, int value) { atom.setProp(key, value); }
int get_int_prop(const Atom &atom, const std::string &key) { return atom.getProp<int>(key); }
void set_float_prop(Atom &atom, const std::string &key, double value) { atom.setProp(key, value); }
double get_float_prop(const Atom &atom, const std::string &key) { return atom.getProp<double>(key); }
void set_bool_prop(Atom &atom, const std::string &key, bool value) { atom.setProp(key, value); }
bool get_bool_prop(const Atom &atom, const std::string &key) { return atom.getProp<bool>(key); }
void set_prop(Atom &atom, const std::string &key, const std::string &value) { atom.setProp(key, value); }
rust::String get_prop(const Atom &atom, const std::string &key) { return atom.getProp<std::string>(key); }

unsigned int get_num_radical_electrons(const Atom &atom) { return atom.getNumRadicalElectrons(); }
} // namespace RDKit