#pragma once

#include "rust/cxx.h"

namespace RDKit {
const ROMol &rw_mol_as_ro_mol(const RWMol &mol);
std::unique_ptr<RWMol> rw_mol_from_mol_block(const std::string &mol_block, bool sanitize, bool remove_hs,
                                             bool strict_parsing);

std::unique_ptr<RWMol> rw_mol_from_ro_mol(const ROMol &mol, bool quick_copy, int conf_id);

std::unique_ptr<RWMol> rw_mol_from_rw_mol(const RWMol &mol);

std::unique_ptr<ROMol> rw_mol_to_ro_mol(std::unique_ptr<RWMol> mol);

std::unique_ptr<RWMol> smarts_to_mol(const std::string &smarts);
std::unique_ptr<RWMol> new_rw_mol();
ROMol &rw_mol_as_ro_mol_mut(RWMol &mol);
unsigned int rw_mol_add_atom(RWMol &mol, unsigned int atomic_num);
unsigned int rw_mol_add_bond(RWMol &mol, unsigned int begin, unsigned int end, int order);
void rw_mol_remove_atom(RWMol &mol, unsigned int idx);
void rw_mol_remove_bond(RWMol &mol, unsigned int begin, unsigned int end);
void rw_mol_sanitize(RWMol &mol);
} // namespace RDKit