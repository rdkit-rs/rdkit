#pragma once
#include "rust/cxx.h"
#include <GraphMol/Conformer.h>
#include <GraphMol/GraphMol.h>

namespace RDKit {
struct Point3D;
unsigned int compute_2d_coords(ROMol &mol);
unsigned int get_num_conformers(const ROMol &mol);
const Conformer &get_conformer(const ROMol &mol, int id);
Conformer &get_conformer_mut(ROMol &mol, int id);
unsigned int conformer_id(const Conformer &conf);
bool conformer_is_3d(const Conformer &conf);
unsigned int conformer_num_atoms(const Conformer &conf);
Point3D conformer_position(const Conformer &conf, unsigned int idx);
void conformer_set_position(Conformer &conf, unsigned int idx, Point3D pos);
}
