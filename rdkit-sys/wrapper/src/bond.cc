#include "wrapper/include/bond.h"
#include "rdkit-sys/src/bridge/bond.rs.h"
#include <GraphMol/GraphMol.h>
#include <stdexcept>

namespace RDKit {

Bond::BondType native_bond_type(int value) {
	switch (value) {
	case static_cast<int>(BondType::UNSPECIFIED): return Bond::UNSPECIFIED;
	case static_cast<int>(BondType::SINGLE): return Bond::SINGLE;
	case static_cast<int>(BondType::DOUBLE): return Bond::DOUBLE;
	case static_cast<int>(BondType::TRIPLE): return Bond::TRIPLE;
	case static_cast<int>(BondType::QUADRUPLE): return Bond::QUADRUPLE;
	case static_cast<int>(BondType::QUINTUPLE): return Bond::QUINTUPLE;
	case static_cast<int>(BondType::HEXTUPLE): return Bond::HEXTUPLE;
	case static_cast<int>(BondType::ONEANDAHALF): return Bond::ONEANDAHALF;
	case static_cast<int>(BondType::TWOANDAHALF): return Bond::TWOANDAHALF;
	case static_cast<int>(BondType::THREEANDAHALF): return Bond::THREEANDAHALF;
	case static_cast<int>(BondType::FOURANDAHALF): return Bond::FOURANDAHALF;
	case static_cast<int>(BondType::FIVEANDAHALF): return Bond::FIVEANDAHALF;
	case static_cast<int>(BondType::AROMATIC): return Bond::AROMATIC;
	case static_cast<int>(BondType::IONIC): return Bond::IONIC;
	case static_cast<int>(BondType::HYDROGEN): return Bond::HYDROGEN;
	case static_cast<int>(BondType::THREECENTER): return Bond::THREECENTER;
	case static_cast<int>(BondType::DATIVEONE): return Bond::DATIVEONE;
	case static_cast<int>(BondType::DATIVE): return Bond::DATIVE;
	case static_cast<int>(BondType::DATIVEL): return Bond::DATIVEL;
	case static_cast<int>(BondType::DATIVER): return Bond::DATIVER;
	case static_cast<int>(BondType::OTHER): return Bond::OTHER;
	case static_cast<int>(BondType::ZERO): return Bond::ZERO;
	default: throw std::invalid_argument("invalid bond type");
	}
}

unsigned int get_num_bonds(const ROMol &mol, bool only_heavy) { return mol.getNumBonds(only_heavy); }

const Bond &get_bond_with_idx(const ROMol &mol, unsigned int idx) {
	if (idx >= mol.getNumBonds()) { throw std::out_of_range("bond index"); }
	return *mol.getBondWithIdx(idx);
}

int get_bond_idx_between_atoms(const ROMol &mol, unsigned int begin_idx, unsigned int end_idx) {
	if (begin_idx >= mol.getNumAtoms() || end_idx >= mol.getNumAtoms()) { throw std::out_of_range("bond atom index"); }
	const Bond *bond = mol.getBondBetweenAtoms(begin_idx, end_idx);
	if (bond == nullptr) { return -1; }
	return static_cast<int>(bond->getIdx());
}

BondType bond_get_bond_type(const Bond &bond) {
	switch (bond.getBondType()) {
	case Bond::UNSPECIFIED: return BondType::UNSPECIFIED;
	case Bond::SINGLE: return BondType::SINGLE;
	case Bond::DOUBLE: return BondType::DOUBLE;
	case Bond::TRIPLE: return BondType::TRIPLE;
	case Bond::QUADRUPLE: return BondType::QUADRUPLE;
	case Bond::QUINTUPLE: return BondType::QUINTUPLE;
	case Bond::HEXTUPLE: return BondType::HEXTUPLE;
	case Bond::ONEANDAHALF: return BondType::ONEANDAHALF;
	case Bond::TWOANDAHALF: return BondType::TWOANDAHALF;
	case Bond::THREEANDAHALF: return BondType::THREEANDAHALF;
	case Bond::FOURANDAHALF: return BondType::FOURANDAHALF;
	case Bond::FIVEANDAHALF: return BondType::FIVEANDAHALF;
	case Bond::AROMATIC: return BondType::AROMATIC;
	case Bond::IONIC: return BondType::IONIC;
	case Bond::HYDROGEN: return BondType::HYDROGEN;
	case Bond::THREECENTER: return BondType::THREECENTER;
	case Bond::DATIVEONE: return BondType::DATIVEONE;
	case Bond::DATIVE: return BondType::DATIVE;
	case Bond::DATIVEL: return BondType::DATIVEL;
	case Bond::DATIVER: return BondType::DATIVER;
	case Bond::OTHER: return BondType::OTHER;
	case Bond::ZERO: return BondType::ZERO;
	default: throw std::invalid_argument("unsupported native bond enum");
	}
}

double bond_get_bond_type_as_double(const Bond &bond) { return bond.getBondTypeAsDouble(); }

unsigned int bond_get_begin_atom_idx(const Bond &bond) { return bond.getBeginAtomIdx(); }

unsigned int bond_get_end_atom_idx(const Bond &bond) { return bond.getEndAtomIdx(); }

unsigned int bond_get_other_atom_idx(const Bond &bond, unsigned int this_idx) { return bond.getOtherAtomIdx(this_idx); }

bool bond_get_is_aromatic(const Bond &bond) { return bond.getIsAromatic(); }

bool bond_get_is_conjugated(const Bond &bond) { return bond.getIsConjugated(); }

template<typename NativeBond> BondStereo native_bond_stereo(const NativeBond &bond) {
	if constexpr (requires {
		              NativeBond::STEREOATROPCW;
		              NativeBond::STEREOATROPCCW;
	              }) {
		if (bond.getStereo() == NativeBond::STEREOATROPCW) { return BondStereo::STEREOATROPCW; }
		if (bond.getStereo() == NativeBond::STEREOATROPCCW) { return BondStereo::STEREOATROPCCW; }
	}
	switch (bond.getStereo()) {
	case NativeBond::STEREONONE: return BondStereo::STEREONONE;
	case NativeBond::STEREOANY: return BondStereo::STEREOANY;
	case NativeBond::STEREOZ: return BondStereo::STEREOZ;
	case NativeBond::STEREOE: return BondStereo::STEREOE;
	case NativeBond::STEREOCIS: return BondStereo::STEREOCIS;
	case NativeBond::STEREOTRANS: return BondStereo::STEREOTRANS;
	default: throw std::invalid_argument("unsupported native bond stereo");
	}
}

BondStereo bond_get_stereo(const Bond &bond) { return native_bond_stereo(bond); }

BondDir bond_get_bond_dir(const Bond &bond) {
	switch (bond.getBondDir()) {
	case Bond::NONE: return BondDir::NONE;
	case Bond::BEGINWEDGE: return BondDir::BEGINWEDGE;
	case Bond::BEGINDASH: return BondDir::BEGINDASH;
	case Bond::ENDDOWNRIGHT: return BondDir::ENDDOWNRIGHT;
	case Bond::ENDUPRIGHT: return BondDir::ENDUPRIGHT;
	case Bond::EITHERDOUBLE: return BondDir::EITHERDOUBLE;
	case Bond::UNKNOWN: return BondDir::UNKNOWN;
	default: throw std::invalid_argument("unsupported native bond enum");
	}
}

unsigned int bond_get_idx(const Bond &bond) { return bond.getIdx(); }

}
