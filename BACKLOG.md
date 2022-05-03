Passing Vector of SharedPtr
---

I want a bridge like:

    #[cxx::bridge(namespace = "RDKit")]
    pub mod ffi {
        struct SharedROMol {
            s: SharedPtr<ROMol>
        }
    
        unsafe extern "C++" {
            pub fn tautomer_enumerator_result_tautomers(
                tautomer_enumerator_result: SharedPtr<TautomerEnumeratorResult>
            ) -> Vec<SharedROMol>;
        }
    }

With a C++ impl like:

    rust::Vec<SharedROMol> tautomer_enumerator_result_tautomers(std::shared_ptr<TautomerEnumeratorResult> enumerator_result) {
        std::vector<RDKit::ROMOL_SPTR> tautomers = enumerator_result->tautomers();
        rust::Vec<std::shared_ptr<ROMol>> tautomer_pointers;
        tautomer_pointers.reserve(tautomers.size());

        for (auto iter = tautomers.cbegin(); iter != tautomers.cend(); iter++) {
            //            tautomer_pointers.push_back(std::shared_ptr<ROMol>(new ROMol(**iter)));
            tautomer_pointers.push_back(new ROMol(**iter));
        }
        tautomers.clear();

        return tautomer_pointers;
    }

but I always get issue saying the `SharedROMol` wrapper struct isn't defined. Got this wrapper struct idea from https://github.com/dtolnay/cxx/issues/741