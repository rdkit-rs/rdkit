#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/RingInfo.h>

namespace RDKit {

unsigned int mol_num_rings(const std::shared_ptr<ROMol> &mol);
bool mol_is_atom_in_ring_of_size(const std::shared_ptr<ROMol> &mol, unsigned int atom_idx, unsigned int size);
bool mol_is_bond_in_ring_of_size(const std::shared_ptr<ROMol> &mol, unsigned int bond_idx, unsigned int size);
unsigned int mol_num_atom_rings(const std::shared_ptr<ROMol> &mol, unsigned int atom_idx);
unsigned int mol_num_bond_rings(const std::shared_ptr<ROMol> &mol, unsigned int bond_idx);
std::unique_ptr<std::vector<int32_t>> mol_atom_ring_sizes(const std::shared_ptr<ROMol> &mol, unsigned int atom_idx);
std::unique_ptr<std::vector<int32_t>> mol_bond_ring_sizes(const std::shared_ptr<ROMol> &mol, unsigned int bond_idx);

} // namespace RDKit
