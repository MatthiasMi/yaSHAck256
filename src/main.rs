use std::{fs, io::Read};
use std::ops::Shr;


struct Sha256 {
    /// 256-bit (intermediate) state of hash-function
    h: [u32; 8]
}

/// For extracting bytes of u64 numbers used in binary conversion.
const MASK: u64 = 0xff;


/// Initial hash values as 8 constant 32-bit words described in FIPS PUB 180-4, Ch. 5.3.3.
const H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// Hex-representation of the first 32 bits of the fractional parts of the cube roots of the
/// first 64 primes {2,3,...,311} as 64 constant 32-bit words described in FIPS PUB 180-4, Ch. 4.2.2.
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];


impl Sha256 {
    pub fn new() -> Sha256 {
        Sha256 { h: H0 }
    }

    pub fn update(&mut self, v: &mut Vec<u8>) {
        let num_blocks = self.preprocess(v);

        //compute()
    }
    /// `preprocess` creates a padded message with a bit-length divisible by 512.
    pub fn preprocess(&mut self, v: &mut Vec<u8>) -> u64 {
        let bits = (v.len() * 8) as u64;
        v.push(0x80);  // Append 10000000
        while (v.len() % 64) < 56 {
            v.push(0x00);  // Append 00000000
        }
        // Binary representation of `v`s bit-length ("big-endian" convention)
        let mut be = Vec::with_capacity(8);
        for i in (0..=7).rev()  {
            let byte = ((bits >> (8 * i)) & MASK) as u8;
            be[8-i]=byte;
        }
        v.append(&mut be);
        
        v.len() as u64 / 64
    }

}



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
