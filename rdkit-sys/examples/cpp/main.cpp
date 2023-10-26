#include <iostream>

#include "GraphMol/GraphMol.h"
#include "GraphMol/MolStandardize/Tautomer.h"
#include "GraphMol/MolStandardize/Charge.h"
#include "GraphMol/SmilesParse/SmilesParse.h"
#include "GraphMol/SmilesParse/SmilesWrite.h"
#include "GraphMol/MolStandardize/Tautomer.h"

using namespace RDKit;
//using namespace RDKit::MolStandardize;

int main() {
    std::shared_ptr<ROMol> mol(RDKit::SmilesToMol("c1ccccc1C(=O)NC"));

    std::shared_ptr<MolStandardize::TautomerEnumerator> tautomer_enumerator(new MolStandardize::TautomerEnumerator());
//    MolStandardize::TautomerEnumerator *tautomer_enumerator = new MolStandardize::TautomerEnumerator();

    auto tauts = tautomer_enumerator->enumerate(*mol);

    for (auto &t: tauts) {
        std::cout << "hello " << RDKit::MolToSmiles(*t, RDKit::SmilesWriteParams{}) << std::endl;
    }

}