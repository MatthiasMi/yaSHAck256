use std::{env, fs, io::Read, path::Path, process::Command};
use yashack256::Sha256;

/// `sanity_check`s against built-in sha256sum
fn sanity_check(file_to_hash: &str) -> String {
    // "8a4727ddbb1a0962a0c6bf27d74cbb4205707c7856d4b8671fe6110663bfea15"
    let output = Command::new("sha256sum")
        .arg(file_to_hash)
        .output()
        .expect("sha256sum failed");
    String::from_utf8(output.stdout).unwrap()
}

fn main() {
    let file_to_hash = "tests/test_template.rs";
    let mut bytes = Vec::<u8>::new();

    let args: Vec<String> = env::args().collect();

    let file_to_hash = match args.len() {
        1 => {
            println!("Called {} with no arguments.\nSetting file_to_hash = \"{}\";", args[0],file_to_hash);
            file_to_hash
        },
        2 => {
            let file_to_hash = &args[1];
            println!("Called {} with one argument.\nAssuming file_to_hash = \"{}\";", args[0],file_to_hash);
            file_to_hash
        },
        _ => {
            println!("If no arguments are given, one file of the repository (`\"tests/test_template.rs\"`) is hashed for demonstrations purposes.
            If an arguments is given, it is assumed to be a `file_to_hash` whose content is then hashed. If this assumption does not hold, the hash values of an empty string, i.e., a zero-length input text,
            `SHA256(\"\")` is computed and checked against `0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`](https://en.wikipedia.org/wiki/SHA-2#Test_vectors).");
            ""
        }
    };

    let mut res = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string();
    if Path::new(file_to_hash).exists(){
        let mut input = fs::File::open(file_to_hash).unwrap();
        input.read_to_end(&mut bytes).unwrap();
        res = sanity_check(file_to_hash)[..64].to_string();
        println!("Using {} for demo.", file_to_hash);
    };

    let mut h = Sha256::new();
    h.update(&mut bytes);
    let hash = h.to_string();

    println!("{} == {}",res, hash);
    assert_eq!(res, hash);
    println!("Success: {}",res == hash);
}
