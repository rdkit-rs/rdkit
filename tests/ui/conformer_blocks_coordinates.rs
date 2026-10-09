use rdkit::ROMol;
fn main() {
    let mut mol = ROMol::from_smiles("C").unwrap();
    mol.compute_2d_coords().unwrap();
    let conf = mol.conformer(-1).unwrap();
    mol.compute_2d_coords().unwrap();
    println!("{}", conf.id());
}
