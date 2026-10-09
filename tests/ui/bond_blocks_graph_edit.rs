use rdkit::RWMol;
fn main() {
    let mut mol = RWMol::from_smarts("CC").unwrap();
    let bond = mol.bond(0).unwrap();
    mol.remove_atom(0).unwrap();
    println!("{}", bond.idx());
}
