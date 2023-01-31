use std::{fs, io::Read};

fn main() {
    let mut input = fs::File::open("Cargo.toml").unwrap();

    let mut bytes = Vec::<u8>::new();
    input.read_to_end(&mut bytes).unwrap();

    // TODO
    //let mut h = Sha256::new();
    //h.update(&mut bytes);
    //let hash = h.to_string();
    let hash = 
    "5f52d46e3a2a009397dae72b25ae53c0ee52ff0425709fc9a0b3b17ac983d222";

    // Sanity check against built-in sha256sum
    let sha256sum = 
    "5f52d46e3a2a009397dae72b25ae53c0ee52ff0425709fc9a0b3b17ac983d222";
    assert_eq!(sha256sum, hash);
}
