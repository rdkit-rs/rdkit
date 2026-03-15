#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>

namespace RDKit {

int embed_molecule(std::shared_ptr<ROMol> &mol);
std::unique_ptr<std::vector<int32_t>> embed_multiple_confs(std::shared_ptr<ROMol> &mol, unsigned int num_confs);
unsigned int compute_2d_coords(std::shared_ptr<ROMol> &mol);

unsigned int mol_get_num_conformers(const std::shared_ptr<ROMol> &mol);
bool conformer_is_3d(const std::shared_ptr<ROMol> &mol, int conf_id);
std::unique_ptr<std::vector<double>> get_atom_pos(const std::shared_ptr<ROMol> &mol, int conf_id,
                                                  unsigned int atom_idx);

} // namespace RDKit
