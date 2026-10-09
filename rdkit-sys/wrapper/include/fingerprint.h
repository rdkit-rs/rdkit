#pragma once

#include "rust/cxx.h"

namespace RDKit {
using ExplicitBitVect = ::ExplicitBitVect;
std::unique_ptr<ExplicitBitVect> rdk_fingerprint_mol(const ROMol &mol);
std::unique_ptr<ExplicitBitVect> pattern_fingerprint_mol(const ROMol &mol);
std::unique_ptr<ExplicitBitVect> morgan_fingerprint_mol(const ROMol &mol);
std::unique_ptr<ExplicitBitVect> copy_explicit_bit_vect(const ExplicitBitVect &orig);

std::unique_ptr<std::vector<uint64_t>> explicit_bit_vect_to_u64_vec(const ExplicitBitVect &bitvect);
unsigned int explicit_bit_vect_num_bits(const ExplicitBitVect &bits);
std::unique_ptr<ExplicitBitVect> morgan_fingerprint_mol_with_params(const ROMol &mol, unsigned int radius,
                                                                    unsigned int n_bits);
std::unique_ptr<ExplicitBitVect> rdk_fingerprint_mol_with_params(const ROMol &mol, unsigned int min_path,
                                                                 unsigned int max_path, unsigned int n_bits);
std::unique_ptr<ExplicitBitVect> pattern_fingerprint_mol_with_params(const ROMol &mol, unsigned int n_bits);
std::unique_ptr<ExplicitBitVect> maccs_fingerprint_mol(const ROMol &mol);
} // namespace RDKit