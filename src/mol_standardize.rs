use cxx::UniquePtr;
use rdkit_sys::mol_standardize_ffi as ffi;

use crate::{ROMol, RWMol};

pub struct CleanupParameters {
    pub(crate) ptr: UniquePtr<ffi::CleanupParameters>,
}

impl Default for CleanupParameters {
    fn default() -> Self {
        Self {
            ptr: ffi::default_cleanup_parameters().expect("RDKit cleanup allocation failed"),
        }
    }
}

pub struct TautomerEnumerator {
    pub(crate) ptr: UniquePtr<ffi::TautomerEnumerator>,
}

impl Default for TautomerEnumerator {
    fn default() -> Self {
        Self::new()
    }
}

impl TautomerEnumerator {
    pub fn new() -> Self {
        Self::try_new().expect("RDKit tautomer enumerator allocation failed")
    }

    pub fn try_new() -> Result<Self, cxx::Exception> {
        Ok(Self {
            ptr: ffi::tautomer_enumerator()?,
        })
    }

    pub fn enumerate(&self, mol: &ROMol) -> TautomerEnumeratorResult {
        self.try_enumerate(mol)
            .expect("RDKit tautomer enumeration failed")
    }

    pub fn try_enumerate(&self, mol: &ROMol) -> Result<TautomerEnumeratorResult, cxx::Exception> {
        let ptr = ffi::tautomer_enumerate(&self.ptr, &mol.ptr)?;
        let size = ffi::tautomer_enumerator_result_tautomers_size(&ptr)?;
        Ok(TautomerEnumeratorResult { ptr, pos: 0, size })
    }

    pub fn canonicalize(&self, mol: &ROMol) -> Result<ROMol, cxx::Exception> {
        Ok(ROMol {
            ptr: ffi::tautomer_enumerator_canonicalize(&self.ptr, &mol.ptr)?,
        })
    }
}

pub struct TautomerEnumeratorResult {
    ptr: UniquePtr<ffi::TautomerEnumeratorResult>,
    pos: usize,
    size: usize,
}

impl TautomerEnumeratorResult {
    pub fn get(&self, idx: usize) -> Result<ROMol, cxx::Exception> {
        Ok(ROMol {
            ptr: ffi::tautomer_enumerator_result_tautomers_at(&self.ptr, idx)?,
        })
    }

    pub fn try_next(&mut self) -> Result<Option<ROMol>, cxx::Exception> {
        if self.pos == self.size {
            return Ok(None);
        }
        let next = self.get(self.pos)?;
        self.pos += 1;
        Ok(Some(next))
    }
}

impl Iterator for TautomerEnumeratorResult {
    type Item = ROMol;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().expect("RDKit tautomer copy failed")
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.pos;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for TautomerEnumeratorResult {}
impl std::iter::FusedIterator for TautomerEnumeratorResult {}

pub fn try_fragment_parent(
    mol: &RWMol,
    params: &CleanupParameters,
    skip_standardize: bool,
) -> Result<RWMol, cxx::Exception> {
    Ok(RWMol {
        ptr: ffi::fragment_parent(&mol.ptr, &params.ptr, skip_standardize)?,
    })
}

pub fn fragment_parent(mol: &RWMol, params: &CleanupParameters, skip_standardize: bool) -> RWMol {
    try_fragment_parent(mol, params, skip_standardize).expect("RDKit fragment parent failed")
}

pub fn normalize(mol: &RWMol, params: &CleanupParameters) -> Result<RWMol, cxx::Exception> {
    Ok(RWMol {
        ptr: ffi::normalize(&mol.ptr, &params.ptr)?,
    })
}

pub struct Uncharger {
    ptr: UniquePtr<ffi::Uncharger>,
}

impl Uncharger {
    pub fn new(canonical: bool) -> Self {
        Self::try_new(canonical).expect("RDKit uncharger allocation failed")
    }

    pub fn try_new(canonical: bool) -> Result<Self, cxx::Exception> {
        Ok(Self {
            ptr: ffi::new_uncharger(canonical)?,
        })
    }

    pub fn uncharge(&mut self, mol: &ROMol) -> ROMol {
        self.try_uncharge(mol).expect("RDKit uncharging failed")
    }

    pub fn try_uncharge(&mut self, mol: &ROMol) -> Result<ROMol, cxx::Exception> {
        Ok(ROMol {
            ptr: ffi::uncharger_uncharge(self.ptr.pin_mut(), &mol.ptr)?,
        })
    }
}
