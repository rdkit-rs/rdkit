use crate::ROMol;

impl ROMol {
    pub fn embed_molecule(&mut self) -> i32 {
        rdkit_sys::conformer_ffi::embed_molecule(&mut self.ptr)
    }

    pub fn embed_multiple_confs(&mut self, num_confs: u32) -> Vec<i32> {
        let ids = rdkit_sys::conformer_ffi::embed_multiple_confs(&mut self.ptr, num_confs);
        ids.iter().copied().collect()
    }

    pub fn compute_2d_coords(&mut self) -> u32 {
        rdkit_sys::conformer_ffi::compute_2d_coords(&mut self.ptr)
    }

    pub fn num_conformers(&self) -> u32 {
        rdkit_sys::conformer_ffi::mol_get_num_conformers(&self.ptr)
    }

    pub fn conformer_is_3d(&self, conf_id: i32) -> bool {
        rdkit_sys::conformer_ffi::conformer_is_3d(&self.ptr, conf_id)
    }

    pub fn atom_position(&self, conf_id: i32, atom_idx: u32) -> (f64, f64, f64) {
        let pos = rdkit_sys::conformer_ffi::get_atom_pos(&self.ptr, conf_id, atom_idx);
        let s = pos.as_ref().unwrap().as_slice();
        (s[0], s[1], s[2])
    }
}
