use rdkit::ROMol;
fn main() {
    let mut mol = ROMol::from_smiles("C").unwrap();
    let shared = mol.atom_ref(0).unwrap();
    mol.atom_with_idx(0).set_formal_charge(1);
    println!("{}", shared.symbol());
}
