use cxx::SharedPtr;
use rdkit_sys::ro_mol_ffi::ROMol;

pub struct TautomerEnumerator {
    pub(crate) t_enumerator: SharedPtr<rdkit_sys::mol_standardize_ffi::TautomerEnumerator>
}

impl TautomerEnumerator {
    pub fn new() -> Self {
        TautomerEnumerator {
            t_enumerator: rdkit_sys::mol_standardize_ffi::tautomer_enumerator()
        }
    }

    pub fn enumerate(&self, ro_mol: crate::ROMol) -> TautomerEnumeratorResult {
        let t_enumerator_result = rdkit_sys::mol_standardize_ffi::tautomer_enumerate(self.t_enumerator.clone(), ro_mol.ptr);

        TautomerEnumeratorResult {
            pos: 0,
            t_enumerator_result
        }
    }
}

pub struct TautomerEnumeratorResult {
    // t_enumerator: SharedPtr<TautomerEnumerator>,
    pub(crate) t_enumerator_result: SharedPtr<rdkit_sys::mol_standardize_ffi::TautomerEnumeratorResult>,
    pos: usize,
}

impl Iterator for TautomerEnumeratorResult {
    type Item = SharedPtr<ROMol>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(self.t_enumerator_result.clone(), self.pos);
        if next.is_null() {
            None
        } else {
            Some(next)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_size(self.t_enumerator_result.clone());
        (size as usize, Some(size as usize))
    }
}