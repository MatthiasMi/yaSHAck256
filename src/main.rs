use std::{fs, io::Read};
use yashack256::Sha256;
fn main() {
    let mut input = fs::File::open("tests/test_template.rs").unwrap();

    let mut bytes = Vec::<u8>::new();
    input.read_to_end(&mut bytes).unwrap();

    let mut h = Sha256::new();
    h.update(&mut bytes);
    let hash = h.to_string();

    // Sanity check against built-in sha256sum
    let sha256sum = "ce011240884ff4b347218b7296cfd300a107b04a61e07e90994291c0656ca939";
    assert_eq!(sha256sum, hash);
}
