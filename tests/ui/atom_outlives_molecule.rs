use rdkit::ROMol;
fn main() {
    let atom = {
        let mol = ROMol::from_smiles("C").unwrap();
        mol.atom_ref(0).unwrap()
    };
    println!("{}", atom.symbol());
}
