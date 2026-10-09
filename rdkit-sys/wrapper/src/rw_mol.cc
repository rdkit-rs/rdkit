#include "rust/cxx.h"
#include "wrapper/include/bond.h"
#include "wrapper/include/owned.h"
#include "wrapper/include/periodic_table.h"
#include <GraphMol/FileParsers/FileParsers.h>
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>
#include <GraphMol/PeriodicTable.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <RDGeneral/FileParseException.h>
#include <stdexcept>

namespace RDKit {
const ROMol &rw_mol_as_ro_mol(const RWMol &mol) { return mol; }
std::unique_ptr<RWMol> rw_mol_from_mol_block(const std::string &mol_block, bool sanitize, bool remove_hs,
                                             bool strict_parsing) {
	auto mol = std::unique_ptr<RWMol>(MolBlockToMol(mol_block, sanitize, remove_hs, strict_parsing));
	if (!mol) { throw std::invalid_argument("invalid mol block"); }
	return mol;
}

std::unique_ptr<RWMol> rw_mol_from_ro_mol(const ROMol &mol, bool quick_copy, int conf_id) {
	RWMol *rw_mol = new RWMol(mol, quick_copy, conf_id);
	return checked_owned<RWMol>(rw_mol);
}

std::unique_ptr<RWMol> rw_mol_from_rw_mol(const RWMol &mol) {
	RWMol *rw_mol = new RWMol(mol);
	return checked_owned<RWMol>(rw_mol);
}

std::unique_ptr<ROMol> rw_mol_to_ro_mol(std::unique_ptr<RWMol> mol) { return checked_owned<ROMol>(std::move(mol)); }

std::unique_ptr<RWMol> smarts_to_mol(const std::string &smarts) {
	auto mol = std::unique_ptr<RWMol>(SmartsToMol(smarts));
	if (!mol) { throw std::invalid_argument("invalid SMARTS"); }
	return mol;
}
std::unique_ptr<RWMol> new_rw_mol() { return std::make_unique<RWMol>(); }
ROMol &rw_mol_as_ro_mol_mut(RWMol &mol) { return mol; }
unsigned int rw_mol_add_atom(RWMol &mol, unsigned int atomic_num) {
	if (atomic_num > native_periodic_table_max_atomic_number()) {
		throw std::invalid_argument("atomic number out of range");
	}
	Atom atom(atomic_num);
	return mol.addAtom(&atom, true, false);
}
unsigned int rw_mol_add_bond(RWMol &mol, unsigned int begin, unsigned int end, int order) {
	if (begin >= mol.getNumAtoms() || end >= mol.getNumAtoms()) { throw std::out_of_range("bond atom index"); }
	if (begin == end) { throw std::invalid_argument("self bond"); }
	if (mol.getBondBetweenAtoms(begin, end)) { throw std::invalid_argument("bond already exists"); }
	mol.addBond(begin, end, native_bond_type(order));
	return mol.getNumBonds() - 1;
}
void rw_mol_remove_atom(RWMol &mol, unsigned int idx) {
	if (idx >= mol.getNumAtoms()) { throw std::out_of_range("atom index"); }
	mol.removeAtom(idx);
}
void rw_mol_remove_bond(RWMol &mol, unsigned int begin, unsigned int end) {
	if (begin >= mol.getNumAtoms() || end >= mol.getNumAtoms()) { throw std::out_of_range("bond atom index"); }
	if (!mol.getBondBetweenAtoms(begin, end)) { throw std::invalid_argument("bond does not exist"); }
	mol.removeBond(begin, end);
}
void rw_mol_sanitize(RWMol &mol) { MolOps::sanitizeMol(mol); }

} // namespace RDKit