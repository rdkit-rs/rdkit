use rdkit_sys::substruct_match_ffi::*;

#[test]
fn test_substruct_match_as_bool() {
    cxx::let_cxx_string!(smiles = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    cxx::let_cxx_string!(smiles = "C");
    let query = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let match_params = new_substruct_match_parameters();

    let hits = substruct_match(&mol, &query, &match_params);
    // Note that idx are not stable between test runs, we would need to pull out
    // atoms but that's too much work for me.
    let testable_hits: Vec<Vec<(i32,i32)>> = hits
        .into_iter()
        .map(|x| {
            let substruct_match_items = rdkit_sys::substruct_match_ffi::substruct_matchvect_type_to_vec_substruct_match_item(x);
            substruct_match_items.iter().map(|smi| {
                (
                    substruct_match_item_query_atom_idx(smi),
                    substruct_match_item_mol_atom_idx(smi),
                )
            }).collect::<Vec<_>>()

        })
        .collect::<Vec<_>>();

    assert_eq!(testable_hits.len(), 14);
}
