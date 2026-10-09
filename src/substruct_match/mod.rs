mod substruct_match_item;
pub use substruct_match_item::*;

mod substruct_match_parameters;
pub use substruct_match_parameters::*;

use crate::ROMol;
use rdkit_sys::substruct_match_ffi;

pub fn substruct_match(
    mol: &ROMol,
    query: &ROMol,
    params: &SubstructMatchParameters,
) -> Vec<Vec<SubstructMatchItem>> {
    try_substruct_match(mol, query, params).expect("RDKit substructure matching failed")
}

pub fn try_substruct_match(
    mol: &ROMol,
    query: &ROMol,
    params: &SubstructMatchParameters,
) -> Result<Vec<Vec<SubstructMatchItem>>, cxx::Exception> {
    let matches = substruct_match_ffi::substruct_match(&mol.ptr, &query.ptr, &params.ptr)?;
    matches
        .iter()
        .map(|matched| {
            let items =
                substruct_match_ffi::substruct_matchvect_type_to_vec_substruct_match_item(matched)?;
            Ok(items.iter().map(SubstructMatchItem::from).collect())
        })
        .collect()
}
