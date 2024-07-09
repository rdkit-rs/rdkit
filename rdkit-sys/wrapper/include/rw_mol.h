#pragma once

#include "rust/cxx.h"

namespace RDKit {
std::shared_ptr<RWMol> rw_mol_from_mol_block(const std::string &mol_block, bool sanitize, bool remove_hs,
                                             bool strict_parsing);

std::shared_ptr<RWMol> rw_mol_from_ro_mol(const std::shared_ptr<ROMol> &mol, bool quick_copy, int conf_id);

std::shared_ptr<RWMol> rw_mol_from_rw_mol(const std::shared_ptr<RWMol> &mol);

std::shared_ptr<ROMol> rw_mol_to_ro_mol(std::shared_ptr<RWMol> mol);

std::shared_ptr<RWMol> smarts_to_mol(const std::string &smarts);
} // namespace RDKit