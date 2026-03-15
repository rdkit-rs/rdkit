#include "rust/cxx.h"
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/Fingerprints/MACCS.h>
#include <GraphMol/Fingerprints/MorganFingerprints.h>

namespace RDKit {
std::shared_ptr<ExplicitBitVect> rdk_fingerprint_mol(const std::shared_ptr<ROMol> &mol) {
	return std::shared_ptr<ExplicitBitVect>(RDKFingerprintMol(*mol));
}

std::shared_ptr<ExplicitBitVect> pattern_fingerprint_mol(const std::shared_ptr<ROMol> &mol) {
	return std::shared_ptr<ExplicitBitVect>(PatternFingerprintMol(*mol));
}

std::shared_ptr<ExplicitBitVect> morgan_fingerprint_mol(const std::shared_ptr<ROMol> &mol) {
	return std::shared_ptr<ExplicitBitVect>(MorganFingerprints::getFingerprintAsBitVect(*mol, 3, 2048));
}

std::shared_ptr<ExplicitBitVect> copy_explicit_bit_vect(const std::shared_ptr<ExplicitBitVect> &orig) {
	std::shared_ptr<ExplicitBitVect> fingerprint(new ExplicitBitVect(*orig));
	return fingerprint;
}

std::unique_ptr<std::vector<uint64_t>> explicit_bit_vect_to_u64_vec(const std::shared_ptr<ExplicitBitVect> &bitvect) {
	std::vector<uint64_t> bytes;
	bytes.reserve(bitvect->dp_bits->num_blocks());
	boost::to_block_range(*bitvect->dp_bits, (std::back_inserter(bytes)));
	std::vector<uint64_t> *bytes_heap = new std::vector<uint64_t>(bytes);
	return std::unique_ptr<std::vector<uint64_t>>(bytes_heap);
}

std::shared_ptr<ExplicitBitVect> morgan_fingerprint_mol_with_params(const std::shared_ptr<ROMol> &mol,
                                                                    unsigned int radius, unsigned int n_bits) {
	return std::shared_ptr<ExplicitBitVect>(MorganFingerprints::getFingerprintAsBitVect(*mol, radius, n_bits));
}

std::shared_ptr<ExplicitBitVect> rdk_fingerprint_mol_with_params(const std::shared_ptr<ROMol> &mol,
                                                                 unsigned int min_path, unsigned int max_path,
                                                                 unsigned int fp_size) {
	return std::shared_ptr<ExplicitBitVect>(RDKFingerprintMol(*mol, min_path, max_path, fp_size));
}

std::shared_ptr<ExplicitBitVect> pattern_fingerprint_mol_with_params(const std::shared_ptr<ROMol> &mol,
                                                                     unsigned int fp_size) {
	return std::shared_ptr<ExplicitBitVect>(PatternFingerprintMol(*mol, fp_size));
}

std::shared_ptr<ExplicitBitVect> maccs_fingerprint_mol(const std::shared_ptr<ROMol> &mol) {
	return std::shared_ptr<ExplicitBitVect>(MACCSFingerprints::getFingerprintAsBitVect(*mol));
}

unsigned int explicit_bit_vect_num_bits(const std::shared_ptr<ExplicitBitVect> &bitvect) {
	return bitvect->getNumBits();
}
} // namespace RDKit