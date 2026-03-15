#include "rust/cxx.h"
#include <GraphMol/Depictor/RDDepictor.h>
#include <GraphMol/DistGeomHelpers/Embedder.h>
#include <GraphMol/GraphMol.h>

namespace RDKit {

int embed_molecule(std::shared_ptr<ROMol> &mol) { return DGeomHelpers::EmbedMolecule(*mol); }

std::unique_ptr<std::vector<int32_t>> embed_multiple_confs(std::shared_ptr<ROMol> &mol, unsigned int num_confs) {
	auto ids = DGeomHelpers::EmbedMultipleConfs(*mol, num_confs);
	return std::make_unique<std::vector<int32_t>>(ids.begin(), ids.end());
}

unsigned int compute_2d_coords(std::shared_ptr<ROMol> &mol) { return RDDepict::compute2DCoords(*mol); }

unsigned int mol_get_num_conformers(const std::shared_ptr<ROMol> &mol) { return mol->getNumConformers(); }

bool conformer_is_3d(const std::shared_ptr<ROMol> &mol, int conf_id) { return mol->getConformer(conf_id).is3D(); }

std::unique_ptr<std::vector<double>> get_atom_pos(const std::shared_ptr<ROMol> &mol, int conf_id,
                                                  unsigned int atom_idx) {
	const auto &pos = mol->getConformer(conf_id).getAtomPos(atom_idx);
	return std::make_unique<std::vector<double>>(std::vector<double>{pos.x, pos.y, pos.z});
}

} // namespace RDKit
