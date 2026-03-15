#pragma once

#include "rust/cxx.h"
#include <GraphMol/Descriptors/MolDescriptors.h>
#include <GraphMol/Descriptors/Property.h>

namespace RDKit {
using Descriptors::Properties;

std::shared_ptr<Properties> new_properties();
std::unique_ptr<std::vector<std::string>> get_property_names(const std::shared_ptr<Properties> &props);
std::unique_ptr<std::vector<double>> compute_properties(const std::shared_ptr<Properties> &props,
                                                        const std::shared_ptr<ROMol> &mol);
// Targeted descriptors
double calc_exact_mw(const std::shared_ptr<ROMol> &mol);
double calc_amw(const std::shared_ptr<ROMol> &mol);
rust::String calc_mol_formula(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_heavy_atoms(const std::shared_ptr<ROMol> &mol);
double calc_fraction_csp3(const std::shared_ptr<ROMol> &mol);
double calc_labute_asa(const std::shared_ptr<ROMol> &mol);
double calc_tpsa(const std::shared_ptr<ROMol> &mol);
double calc_clog_p(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_hbd(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_hba(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_rotatable_bonds(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_amide_bonds(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_heteroatoms(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_aromatic_rings(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_aliphatic_rings(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_saturated_rings(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_heterocycles(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_aromatic_heterocycles(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_spiro_atoms(const std::shared_ptr<ROMol> &mol);
unsigned int calc_num_bridgehead_atoms(const std::shared_ptr<ROMol> &mol);
} // namespace RDKit