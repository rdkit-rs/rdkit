#include "wrapper/include/conformer.h"
#include "rdkit-sys/src/bridge/conformer.rs.h"
#include <GraphMol/Depictor/RDDepictor.h>
#include <cmath>
#include <stdexcept>

namespace RDKit {
unsigned int compute_2d_coords(ROMol &mol) { return RDDepict::compute2DCoords(mol); }
unsigned int get_num_conformers(const ROMol &mol) { return mol.getNumConformers(); }
const Conformer &get_conformer(const ROMol &mol, int id) { return mol.getConformer(id); }
Conformer &get_conformer_mut(ROMol &mol, int id) { return mol.getConformer(id); }
unsigned int conformer_id(const Conformer &conf) { return conf.getId(); }
bool conformer_is_3d(const Conformer &conf) { return conf.is3D(); }
unsigned int conformer_num_atoms(const Conformer &conf) { return conf.getNumAtoms(); }
Point3D conformer_position(const Conformer &conf, unsigned int idx) {
	if (idx >= conf.getNumAtoms()) { throw std::out_of_range("conformer atom index"); }
	const auto &pos = conf.getAtomPos(idx);
	return {pos.x, pos.y, pos.z};
}
void conformer_set_position(Conformer &conf, unsigned int idx, Point3D pos) {
	if (idx >= conf.getNumAtoms()) { throw std::out_of_range("conformer atom index"); }
	if (!std::isfinite(pos.x) || !std::isfinite(pos.y) || !std::isfinite(pos.z)) {
		throw std::invalid_argument("coordinates must be finite");
	}
	conf.setAtomPos(idx, RDGeom::Point3D(pos.x, pos.y, pos.z));
}
}
