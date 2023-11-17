#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/MolStandardize/Tautomer.h>

namespace RDKit {
    using ROMolSharedPtr = std::shared_ptr<ROMol>;

    std::shared_ptr<ROMol> copy_mol(const std::shared_ptr<ROMol> &mol);
    std::shared_ptr<ROMol> smiles_to_mol(const std::string &smiles);
    rust::String mol_to_smiles(const std::shared_ptr<ROMol> &mol);

    std::shared_ptr<ROMol> smiles_to_mol_with_params(const std::string &smiles, const std::shared_ptr<SmilesParserParams> &params);
    std::shared_ptr<SmilesParserParams> new_smiles_parser_params();
    void smiles_parser_params_set_sanitize(const std::shared_ptr<SmilesParserParams> &params, bool sanitize);

    using MolSanitizeExceptionUniquePtr = std::unique_ptr<MolSanitizeException>;
    std::unique_ptr<std::vector<MolSanitizeExceptionUniquePtr>> detect_chemistry_problems(const std::shared_ptr<ROMol> &mol);
    rust::String mol_sanitize_exception_type(const MolSanitizeExceptionUniquePtr &mol_except);
    unsigned int atom_sanitize_exception_get_atom_idx(const MolSanitizeExceptionUniquePtr &mol_except);

    unsigned int get_num_atoms(const std::shared_ptr<ROMol> &mol, bool only_explicit);
    Atom& get_atom_with_idx(std::shared_ptr<ROMol> &mol, unsigned int idx);
    rust::String get_symbol(const Atom &atom);
    bool get_is_aromatic(const Atom &atom);
    int get_atomic_num(const Atom &atom);
    int get_formal_charge(const Atom &atom);
    unsigned int get_total_num_hs(const Atom &atom);
    unsigned int get_total_valence(const Atom &atom);
    void set_formal_charge(Atom &atom, int what);
    void set_num_explicit_hs(Atom &atom, int what);
    void atom_update_property_cache(Atom &atom, bool strict);

    using HybridizationType = Atom::HybridizationType;
    void atom_set_hybridization(Atom &atom, HybridizationType what);
    HybridizationType atom_get_hybridization(const Atom &atom);

    void ro_mol_update_property_cache(std::shared_ptr<ROMol> &mol, bool strict);
}
