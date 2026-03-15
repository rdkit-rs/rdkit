#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>

namespace RDKit {
std::shared_ptr<RWMol> rw_mol_from_mol_block(const std::string &mol_block, bool sanitize, bool remove_hs,
                                             bool strict_parsing);

std::shared_ptr<RWMol> rw_mol_from_ro_mol(const std::shared_ptr<ROMol> &mol, bool quick_copy, int conf_id);

std::shared_ptr<RWMol> rw_mol_from_rw_mol(const std::shared_ptr<RWMol> &mol);

std::shared_ptr<ROMol> rw_mol_to_ro_mol(std::shared_ptr<RWMol> mol);

std::shared_ptr<RWMol> smarts_to_mol(const std::string &smarts);
// Molecule editing
std::shared_ptr<RWMol> new_rw_mol();
unsigned int rw_mol_add_atom(std::shared_ptr<RWMol> &mol, unsigned int atomic_num);
unsigned int rw_mol_add_bond(std::shared_ptr<RWMol> &mol, unsigned int begin_idx, unsigned int end_idx, int bond_order);
void rw_mol_remove_atom(std::shared_ptr<RWMol> &mol, unsigned int idx);
void rw_mol_remove_bond(std::shared_ptr<RWMol> &mol, unsigned int begin_idx, unsigned int end_idx);
unsigned int rw_mol_get_num_atoms(const std::shared_ptr<RWMol> &mol, bool only_explicit);
} // namespace RDKit