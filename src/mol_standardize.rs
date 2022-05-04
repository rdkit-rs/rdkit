use cxx::SharedPtr;
use crate::{ ROMol, RWMol };

pub struct CleanupParameters {
    pub(crate) ptr: SharedPtr<rdkit_sys::mol_standardize_ffi::CleanupParameters>
}

impl Default for CleanupParameters {
    fn default() -> Self {
        CleanupParameters {
            ptr: rdkit_sys::mol_standardize_ffi::default_cleanup_parameters()
        }
    }
}


pub struct TautomerEnumerator {
    pub(crate) t_enumerator: SharedPtr<rdkit_sys::mol_standardize_ffi::TautomerEnumerator>
}

impl TautomerEnumerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enumerate(&self, ro_mol: crate::ROMol) -> TautomerEnumeratorResult {
        let t_enumerator_result = rdkit_sys::mol_standardize_ffi::tautomer_enumerate(self.t_enumerator.clone(), ro_mol.ptr);

        TautomerEnumeratorResult {
            pos: 0,
            t_enumerator_result
        }
    }
}

impl Default for TautomerEnumerator {
    fn default() -> Self {
        TautomerEnumerator {
            t_enumerator: rdkit_sys::mol_standardize_ffi::tautomer_enumerator()
        }
    }
}

pub struct TautomerEnumeratorResult {
    // t_enumerator: SharedPtr<TautomerEnumerator>,
    pub(crate) t_enumerator_result: SharedPtr<rdkit_sys::mol_standardize_ffi::TautomerEnumeratorResult>,
    pos: usize,
}

impl Iterator for TautomerEnumeratorResult {
    type Item = crate::ROMol;

    fn next(&mut self) -> Option<Self::Item> {
        let next = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(self.t_enumerator_result.clone(), self.pos);
        self.pos += 1;
        if next.is_null() {
            None
        } else {
            Some(crate::ROMol{ ptr: next })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_size(self.t_enumerator_result.clone());
        (size as usize, Some(size as usize))
    }
}

pub fn fragment_parent(rw_mol: RWMol, cleanup_params: CleanupParameters, skip_standardize: bool) -> RWMol {
    let ptr = rdkit_sys::mol_standardize_ffi::fragment_parent(rw_mol.ptr.clone(), cleanup_params.ptr.clone(), skip_standardize);
    RWMol{ ptr }
}

pub struct Uncharger {
    pub(crate) ptr: SharedPtr<rdkit_sys::mol_standardize_ffi::Uncharger>
}

impl Uncharger {
    pub fn new(standardize: bool) -> Self {
        Uncharger {
            ptr: rdkit_sys::mol_standardize_ffi::new_uncharger(standardize)
        }
    }

    pub fn uncharge(&self, mol: ROMol) -> ROMol {
        ROMol {
            ptr: rdkit_sys::mol_standardize_ffi::uncharger_uncharge(self.ptr.clone(), mol.ptr.clone())
        }
    }
}