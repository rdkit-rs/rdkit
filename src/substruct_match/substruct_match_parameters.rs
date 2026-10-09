use cxx::UniquePtr;
use rdkit_sys::substruct_match_ffi::new_substruct_match_parameters;

pub struct SubstructMatchParameters {
    pub(crate) ptr: UniquePtr<rdkit_sys::substruct_match_ffi::SubstructMatchParameters>,
}

impl Default for SubstructMatchParameters {
    fn default() -> Self {
        SubstructMatchParameters::new()
    }
}

#[allow(dead_code)]
impl SubstructMatchParameters {
    pub fn new() -> Self {
        let ptr = new_substruct_match_parameters().expect("RDKit operation failed");

        SubstructMatchParameters { ptr }
    }

    pub fn get_use_chirality(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_use_chirality(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn get_use_enhanced_stereo(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_use_enhanced_stereo(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn get_aromatic_matches_conjugated(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_aromatic_matches_conjugated(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn get_use_query_query_matches(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_use_query_query_matches(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn get_use_generic_matchers(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_use_generic_matchers(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn get_recursion_possible(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_recursion_possible(&self.ptr)
            .expect("RDKit operation failed")
    }

    pub fn get_uniquify(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_uniquify(&self.ptr).expect("RDKit operation failed")
    }

    pub fn set_use_chirality(&mut self, what: bool) {
        rdkit_sys::substruct_match_ffi::set_use_chirality(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn set_use_enhanced_stereo(&mut self, what: bool) {
        rdkit_sys::substruct_match_ffi::set_use_enhanced_stereo(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn set_aromatic_matches_conjugated(&mut self, what: bool) {
        rdkit_sys::substruct_match_ffi::set_aromatic_matches_conjugated(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn set_use_query_query_matches(&mut self, what: bool) {
        rdkit_sys::substruct_match_ffi::set_use_query_query_matches(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn set_use_generic_matchers(&mut self, what: bool) {
        rdkit_sys::substruct_match_ffi::set_use_generic_matchers(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn set_recursion_possible(&mut self, what: bool) {
        rdkit_sys::substruct_match_ffi::set_recursion_possible(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }

    pub fn set_uniquify(&mut self, what: bool) {
        rdkit_sys::substruct_match_ffi::set_uniquify(self.ptr.pin_mut(), what)
            .expect("RDKit operation failed")
    }
}
