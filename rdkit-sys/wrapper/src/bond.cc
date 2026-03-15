#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>

namespace RDKit {

using BondType   = Bond::BondType;
using BondStereo = Bond::BondStereo;
using BondDir    = Bond::BondDir;

unsigned int get_num_bonds(const std::shared_ptr<ROMol> &mol, bool only_heavy) { return mol->getNumBonds(only_heavy); }

Bond &get_bond_with_idx(std::shared_ptr<ROMol> &mol, unsigned int idx) { return *mol->getBondWithIdx(idx); }

int get_bond_idx_between_atoms(const std::shared_ptr<ROMol> &mol, unsigned int begin_idx, unsigned int end_idx) {
	const Bond *bond = mol->getBondBetweenAtoms(begin_idx, end_idx);
	if (bond == nullptr) { return -1; }
	return static_cast<int>(bond->getIdx());
}

BondType bond_get_bond_type(const Bond &bond) { return bond.getBondType(); }

double bond_get_bond_type_as_double(const Bond &bond) { return bond.getBondTypeAsDouble(); }

unsigned int bond_get_begin_atom_idx(const Bond &bond) { return bond.getBeginAtomIdx(); }

unsigned int bond_get_end_atom_idx(const Bond &bond) { return bond.getEndAtomIdx(); }

unsigned int bond_get_other_atom_idx(const Bond &bond, unsigned int this_idx) { return bond.getOtherAtomIdx(this_idx); }

bool bond_get_is_aromatic(const Bond &bond) { return bond.getIsAromatic(); }

bool bond_get_is_conjugated(const Bond &bond) { return bond.getIsConjugated(); }

BondStereo bond_get_stereo(const Bond &bond) { return bond.getStereo(); }

BondDir bond_get_bond_dir(const Bond &bond) { return bond.getBondDir(); }

unsigned int bond_get_idx(const Bond &bond) { return bond.getIdx(); }

} // namespace RDKit
