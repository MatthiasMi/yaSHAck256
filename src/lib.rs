use std::{fmt, ops::Shr};

pub struct Sha256 {
    /// 256-bit (intermediate) state of hash-function
    h: [u32; 8],
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

    /// `update` the state with hashed blocks after padding message, c.f. FIPS PUB 180-4, Ch. 6.2.1.
    pub fn update(&mut self, v: &mut Vec<u8>) {
        let num_blocks = self.preprocess(v);

        for i in 0..num_blocks {
            self.compute(&v[i * 64..i * 64 + 64]);
        }
    }

    /// `compute` hash of raw data blocks, c.f. FIPS PUB 180-4, Ch. 6.2.2.
    fn compute(&mut self, data: &[u8]) {
        let mut w = [0u32; 64];

        // First 16 words of message schedule "just" copy the data, [borrowed from](https://github.com/danieldidiobalsamo/sha256sum_from_scratch/blob/master/sha_256_scratch/src/lib.rs#L76)
        for i in 0..16 {
            let byte = &data[4 * i..4 * i + 4];
            let mut word = 0u32;
            for j in 0..4 {
                word |= (byte[j] as u32) << (24 - (8 * j));
            }
            w[i] = word;
        }

        // Scheduling
        for i in 16..64 {
            let s0: u32 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ w[i - 15].shr(3);
            let s1: u32 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ w[i - 2].shr(10);
            w[i] = s0
                .wrapping_add(s1)
                .wrapping_add(w[i - 7])
                .wrapping_add(w[i - 16]);
        }

        // Initialize working variables
        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];
        let mut f = self.h[5];
        let mut g = self.h[6];
        let mut h = self.h[7];

        for i in 0..64 {
            let ch = (e & f) ^ ((!e) & g);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let t2 = s0.wrapping_add(maj);

            // Update working variables
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        // Update state
        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);
    }

    /// `preprocess` creates a padded message with a bit-length divisible by 512.
    fn preprocess(&mut self, v: &mut Vec<u8>) -> usize {
        let bits = (v.len() * 8) as u64;
        v.push(0x80); // Append 10000000
        while (v.len() % 64) < 56 {
            v.push(0x00); // Append 00000000
        }
        // Binary representation of `v`s bit-length ("big-endian" convention)
        let mut be = vec![0u8; 8];
        for i in (0..=7).rev() {
            let byte = ((bits >> (8 * i)) & MASK) as u8;
            be[7 - i] = byte;
        }
        v.append(&mut be);

        v.len() / 64
    }
}

impl fmt::Display for Sha256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.h.iter().map(|x| format!("{:08x}", x)).collect::<String>()
        )
    }
}
