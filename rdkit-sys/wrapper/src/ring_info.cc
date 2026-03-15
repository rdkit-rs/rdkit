#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/RingInfo.h>

namespace RDKit {

unsigned int mol_num_rings(const std::shared_ptr<ROMol> &mol) { return mol->getRingInfo()->numRings(); }

bool mol_is_atom_in_ring_of_size(const std::shared_ptr<ROMol> &mol, unsigned int atom_idx, unsigned int size) {
	return mol->getRingInfo()->isAtomInRingOfSize(atom_idx, size);
}

bool mol_is_bond_in_ring_of_size(const std::shared_ptr<ROMol> &mol, unsigned int bond_idx, unsigned int size) {
	return mol->getRingInfo()->isBondInRingOfSize(bond_idx, size);
}

unsigned int mol_num_atom_rings(const std::shared_ptr<ROMol> &mol, unsigned int atom_idx) {
	return mol->getRingInfo()->numAtomRings(atom_idx);
}

unsigned int mol_num_bond_rings(const std::shared_ptr<ROMol> &mol, unsigned int bond_idx) {
	return mol->getRingInfo()->numBondRings(bond_idx);
}

std::unique_ptr<std::vector<int32_t>> mol_atom_ring_sizes(const std::shared_ptr<ROMol> &mol, unsigned int atom_idx) {
	auto sizes = mol->getRingInfo()->atomRingSizes(atom_idx);
	return std::make_unique<std::vector<int32_t>>(sizes.begin(), sizes.end());
}

std::unique_ptr<std::vector<int32_t>> mol_bond_ring_sizes(const std::shared_ptr<ROMol> &mol, unsigned int bond_idx) {
	auto sizes = mol->getRingInfo()->bondRingSizes(bond_idx);
	return std::make_unique<std::vector<int32_t>>(sizes.begin(), sizes.end());
}

} // namespace RDKit
