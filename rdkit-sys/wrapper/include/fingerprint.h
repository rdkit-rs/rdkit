#pragma once

#include "rust/cxx.h"

namespace RDKit {
using ExplicitBitVect = ::ExplicitBitVect;
std::shared_ptr<ExplicitBitVect> fingerprint_mol(const std::shared_ptr<ROMol> &mol);
std::shared_ptr<ExplicitBitVect> copy_explicit_bit_vect(const std::shared_ptr<ExplicitBitVect> &orig);
unsigned int get_num_on_bits(const std::shared_ptr<ExplicitBitVect> &bitvect);

std::unique_ptr<std::vector<uint64_t>> explicit_bit_vect_to_u64_vec(const std::shared_ptr<ExplicitBitVect> &bitvect);
} // namespace RDKit