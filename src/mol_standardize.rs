use cxx::SharedPtr;

use crate::{ROMol, RWMol};

pub struct CleanupParameters {
    pub(crate) ptr: SharedPtr<rdkit_sys::mol_standardize_ffi::CleanupParameters>,
}

impl Default for CleanupParameters {
    fn default() -> Self {
        CleanupParameters {
            ptr: rdkit_sys::mol_standardize_ffi::default_cleanup_parameters(),
        }
    }
}

pub struct TautomerEnumerator {
    pub(crate) ptr: SharedPtr<rdkit_sys::mol_standardize_ffi::TautomerEnumerator>,
}

impl Default for TautomerEnumerator {
    fn default() -> Self {
        TautomerEnumerator::new()
    }
}

impl TautomerEnumerator {
    pub fn new() -> Self {
        let ptr = rdkit_sys::mol_standardize_ffi::tautomer_enumerator();

        TautomerEnumerator { ptr }
    }

    pub fn enumerate(&self, ro_mol: &crate::ROMol) -> TautomerEnumeratorResult {
        let t_enumerator_result =
            rdkit_sys::mol_standardize_ffi::tautomer_enumerate(&self.ptr, &ro_mol.ptr);
        let size = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_size(
            &t_enumerator_result,
        ) as usize;

        TautomerEnumeratorResult {
            pos: 0,
            size,
            t_enumerator_result,
        }
    }

    pub fn canonicalize(&self, ro_mol: &crate::ROMol) -> Result<crate::ROMol, cxx::Exception> {
        let canonical_mol_ptr = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_canonicalize(
            &self.ptr,
            &ro_mol.ptr,
        );
        Ok(crate::ROMol {
            ptr: canonical_mol_ptr?,
        })
    }
}

pub struct TautomerEnumeratorResult {
    // t_enumerator: SharedPtr<TautomerEnumerator>,
    pub(crate) t_enumerator_result:
        SharedPtr<rdkit_sys::mol_standardize_ffi::TautomerEnumeratorResult>,
    pos: usize,
    size: usize,
}

impl Iterator for TautomerEnumeratorResult {
    type Item = crate::ROMol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.size {
            None
        } else {
            let next = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(
                &self.t_enumerator_result,
                self.pos,
            );
            if next.is_null() {
                log::warn!(
                    "got a null ptr from tautomer_enumerator_result->at({})",
                    self.pos
                );
                None
            } else {
                self.pos += 1;
                Some(crate::ROMol { ptr: next })
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_size(
            &self.t_enumerator_result,
        );
        (size as usize, Some(size as usize))
    }
}

pub fn fragment_parent(
    rw_mol: &RWMol,
    cleanup_params: &CleanupParameters,
    skip_standardize: bool,
) -> RWMol {
    let ptr = rdkit_sys::mol_standardize_ffi::fragment_parent(
        &rw_mol.ptr,
        &cleanup_params.ptr,
        skip_standardize,
    );
    RWMol { ptr }
}

pub struct Uncharger {
    pub(crate) ptr: SharedPtr<rdkit_sys::mol_standardize_ffi::Uncharger>,
}

impl Uncharger {
    pub fn new(standardize: bool) -> Self {
        Uncharger {
            ptr: rdkit_sys::mol_standardize_ffi::new_uncharger(standardize),
        }
    }

    pub fn uncharge(&self, mol: &ROMol) -> ROMol {
        ROMol {
            ptr: rdkit_sys::mol_standardize_ffi::uncharger_uncharge(&self.ptr, &mol.ptr),
        }
    }
}
