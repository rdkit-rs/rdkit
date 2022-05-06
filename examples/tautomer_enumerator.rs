use rdkit::{ TautomerEnumerator, ROMol };

fn main() {
    let mol = ROMol::from_smile("c1ccccc1C(=O)NC").unwrap();
    let enumerator = TautomerEnumerator::new();
    let enumerator_result = enumerator.enumerate(mol);

    for t in enumerator_result {
        println!("{}", t.as_smile());
    }
}