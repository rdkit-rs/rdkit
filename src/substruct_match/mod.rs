mod substruct_match_item;
pub use substruct_match_item::*;

mod substruct_match_parameters;
pub use substruct_match_parameters::*;

use crate::ROMol;

pub fn substruct_match(
    mol: &ROMol,
    query: &ROMol,
    params: &SubstructMatchParameters,
) -> Vec<Vec<SubstructMatchItem>> {
    let matches =
        rdkit_sys::substruct_match_ffi::substruct_match(&mol.ptr, &query.ptr, &params.ptr);
    matches
        .into_iter()
        .map(|x| {
            rdkit_sys::substruct_match_ffi::substruct_matchvect_type_to_vec_substruct_match_item(x)
                .iter()
                .map(SubstructMatchItem::from)
                .collect::<Vec<_>>()
        })
        .collect()
}
