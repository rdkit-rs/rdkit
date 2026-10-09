#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>

namespace RDKit {

using BondType   = Bond::BondType;
using BondStereo = Bond::BondStereo;
using BondDir    = Bond::BondDir;

unsigned int get_num_bonds(const ROMol &mol, bool only_heavy);
const Bond &get_bond_with_idx(const ROMol &mol, unsigned int idx);
int get_bond_idx_between_atoms(const ROMol &mol, unsigned int begin_idx, unsigned int end_idx);

BondType bond_get_bond_type(const Bond &bond);
double bond_get_bond_type_as_double(const Bond &bond);
unsigned int bond_get_begin_atom_idx(const Bond &bond);
unsigned int bond_get_end_atom_idx(const Bond &bond);
unsigned int bond_get_other_atom_idx(const Bond &bond, unsigned int this_idx);
bool bond_get_is_aromatic(const Bond &bond);
bool bond_get_is_conjugated(const Bond &bond);
BondStereo bond_get_stereo(const Bond &bond);
BondDir bond_get_bond_dir(const Bond &bond);
unsigned int bond_get_idx(const Bond &bond);

}
