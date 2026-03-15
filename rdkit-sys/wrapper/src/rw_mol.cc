#include "rust/cxx.h"
#include <GraphMol/FileParsers/FileParsers.h>
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <RDGeneral/FileParseException.h>

namespace RDKit {
std::shared_ptr<RWMol> rw_mol_from_mol_block(const std::string &mol_block, bool sanitize, bool remove_hs,
                                             bool strict_parsing) {
	RWMol *mol;
	try {
		mol = MolBlockToMol(mol_block, sanitize, remove_hs, strict_parsing);
	} catch (const RDKit::FileParseException &e) { mol = nullptr; } catch (const RDKit::AtomValenceException &e) {
		mol = nullptr;
	}
	return std::shared_ptr<RWMol>(mol);
}

std::shared_ptr<RWMol> rw_mol_from_ro_mol(const std::shared_ptr<ROMol> &mol, bool quick_copy, int conf_id) {
	RWMol *rw_mol = new RWMol(*mol, quick_copy, conf_id);
	return std::shared_ptr<RWMol>(rw_mol);
}

std::shared_ptr<RWMol> rw_mol_from_rw_mol(const std::shared_ptr<RWMol> &mol) {
	RWMol *rw_mol = new RWMol(*mol);
	return std::shared_ptr<RWMol>(rw_mol);
}

std::shared_ptr<ROMol> rw_mol_to_ro_mol(std::shared_ptr<RWMol> mol) { return std::static_pointer_cast<ROMol>(mol); }

std::shared_ptr<RWMol> smarts_to_mol(const std::string &smarts) { return std::shared_ptr<RWMol>(SmartsToMol(smarts)); }
std::shared_ptr<RWMol> new_rw_mol() { return std::shared_ptr<RWMol>(new RWMol()); }

unsigned int rw_mol_add_atom(std::shared_ptr<RWMol> &mol, unsigned int atomic_num) {
	Atom *atom = new Atom(atomic_num);
	return mol->addAtom(atom, true, true);
}

unsigned int rw_mol_add_bond(std::shared_ptr<RWMol> &mol, unsigned int begin_idx, unsigned int end_idx,
                             int bond_order) {
	return mol->addBond(begin_idx, end_idx, static_cast<Bond::BondType>(bond_order));
}

void rw_mol_remove_atom(std::shared_ptr<RWMol> &mol, unsigned int idx) { mol->removeAtom(idx); }

void rw_mol_remove_bond(std::shared_ptr<RWMol> &mol, unsigned int begin_idx, unsigned int end_idx) {
	mol->removeBond(begin_idx, end_idx);
}

unsigned int rw_mol_get_num_atoms(const std::shared_ptr<RWMol> &mol, bool only_explicit) {
	return mol->getNumAtoms(only_explicit);
}
} // namespace RDKit