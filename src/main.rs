use std::{fs, io::Read};
use yashack256::Sha256;

fn main() {
    let mut input = fs::File::open("Cargo.toml").unwrap();

    let mut bytes = Vec::<u8>::new();
    input.read_to_end(&mut bytes).unwrap();

    let mut h = Sha256::new();
    h.update(&mut bytes);
    let hash = h.to_string();

    // Sanity check against built-in sha256sum
    let sha256sum = "9bd330410e2d800aabb8b79a3a0a655f2f6f1b321edb21679c114bd360f2510e";
    assert_eq!(sha256sum, hash);
    print!("{}", hash)
}
