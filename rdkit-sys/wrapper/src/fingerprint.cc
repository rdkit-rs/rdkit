#include "rust/cxx.h"
#include "wrapper/include/owned.h"
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/Fingerprints/MACCS.h>
#include <GraphMol/Fingerprints/MorganFingerprints.h>
#include <stdexcept>

namespace RDKit {
std::unique_ptr<ExplicitBitVect> rdk_fingerprint_mol(const ROMol &mol) {
	return checked_owned<ExplicitBitVect>(RDKFingerprintMol(mol));
}

std::unique_ptr<ExplicitBitVect> pattern_fingerprint_mol(const ROMol &mol) {
	return checked_owned<ExplicitBitVect>(PatternFingerprintMol(mol));
}

std::unique_ptr<ExplicitBitVect> morgan_fingerprint_mol(const ROMol &mol) {
	return checked_owned<ExplicitBitVect>(MorganFingerprints::getFingerprintAsBitVect(mol, 3, 2048));
}

std::unique_ptr<ExplicitBitVect> copy_explicit_bit_vect(const ExplicitBitVect &orig) {
	std::unique_ptr<ExplicitBitVect> fingerprint(new ExplicitBitVect(orig));
	return fingerprint;
}

std::unique_ptr<std::vector<uint64_t>> explicit_bit_vect_to_u64_vec(const ExplicitBitVect &bitvect) {
	auto words = std::make_unique<std::vector<uint64_t>>((static_cast<size_t>(bitvect.getNumBits()) + 63u) / 64u, 0);
	for (unsigned int i = 0; i < bitvect.getNumBits(); ++i) {
		if (bitvect.getBit(i)) { (*words)[i / 64u] |= uint64_t{1} << (i % 64u); }
	}
	return words;
}
unsigned int explicit_bit_vect_num_bits(const ExplicitBitVect &bits) { return bits.getNumBits(); }
std::unique_ptr<ExplicitBitVect> morgan_fingerprint_mol_with_params(const ROMol &mol, unsigned int radius,
                                                                    unsigned int n_bits) {
	if (!n_bits) { throw std::invalid_argument("fingerprint size must be positive"); }
	return checked_owned<ExplicitBitVect>(MorganFingerprints::getFingerprintAsBitVect(mol, radius, n_bits));
}
std::unique_ptr<ExplicitBitVect> rdk_fingerprint_mol_with_params(const ROMol &mol, unsigned int min_path,
                                                                 unsigned int max_path, unsigned int n_bits) {
	if (!n_bits || !min_path || min_path > max_path) { throw std::invalid_argument("invalid fingerprint parameters"); }
	return checked_owned<ExplicitBitVect>(RDKFingerprintMol(mol, min_path, max_path, n_bits));
}
std::unique_ptr<ExplicitBitVect> pattern_fingerprint_mol_with_params(const ROMol &mol, unsigned int n_bits) {
	if (!n_bits) { throw std::invalid_argument("fingerprint size must be positive"); }
	return checked_owned<ExplicitBitVect>(PatternFingerprintMol(mol, n_bits));
}
std::unique_ptr<ExplicitBitVect> maccs_fingerprint_mol(const ROMol &mol) {
	return checked_owned<ExplicitBitVect>(MACCSFingerprints::getFingerprintAsBitVect(mol));
}
} // namespace RDKit