use rdkit_sys::substruct_match_ffi::{new_substruct_match_parameters, substruct_match_as_bool};

use crate::ROMol;

pub fn substruct_match(mol: &ROMol, query: &ROMol) -> bool {
    let params = new_substruct_match_parameters();
    substruct_match_as_bool(&mol.ptr, &query.ptr, &params)
}
