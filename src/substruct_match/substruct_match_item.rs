use rdkit_sys::substruct_match_ffi::SubstructMatchItem as SubstructMatchItemFFI;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SubstructMatchItem {
    query_atom_idx: i32,
    mol_atom_idx: i32,
}

impl From<&SubstructMatchItemFFI> for SubstructMatchItem {
    fn from(value: &SubstructMatchItemFFI) -> Self {
        Self {
            query_atom_idx: rdkit_sys::substruct_match_ffi::substruct_match_item_query_atom_idx(
                value,
            ),
            mol_atom_idx: rdkit_sys::substruct_match_ffi::substruct_match_item_mol_atom_idx(value),
        }
    }
}
