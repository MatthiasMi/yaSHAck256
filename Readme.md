# yaSHAck256
This implementation is *yet another 256-bit SHA hack* for a subset of the specified secure hash algorithms, SHA-256, as described in the [Secure Hash Standard, Ch. 4.1.2](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf) of [FIPS PUB 180-4](https://csrc.nist.gov/publications/detail/fips/180/4/final).

## Summary
Hash algorithms that can be used to generate digests of messages that are used to detect whether messages have been changed since the digests were generated.

This repository contains a Rust implementation of the SHA-256 secure hash algorithm.

The SHA-2 hash function is [widely deployed in applications](https://en.wikipedia.org/wiki/SHA-2#Applications) ranging from several cryptocurrencies to security protocols.

## Main
If no arguments are given, one file of the repository (`"tests/test_template.rs"`) is hashed for demonstrations purposes.
If an arguments is given, it is assumed to be a `file_to_hash` whose content is then hashed. If this assumption does not hold, the hash values of an empty string, i.e., a zero-length input text,
`SHA256("")` is computed and checked against [`0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`](https://en.wikipedia.org/wiki/SHA-2#Test_vectors).

### tl;dr
- `cargo run tests/test_temp.rs` uses "" for demo,
- `cargo run tests/test_template.rs` uses "tests/test_template.rs" for demo,
- `cargo run ` uses "tests/test_template.rs" for demo.

## Tests
A few unit tests cover the most obvious mistakes, run them very explicitly via:

`cargo test --package yashack256 --bin yashack256 -- test_vectors --exact --nocapture`

or simply:

`cargo test --release -- test_vectors`

## Keywords
computer security, cryptographic algorithm, message digest, hash function, FIPS Secure Hash Standard.

## Disclaimer
While SHA-256 is considered secure because, for a given algorithm, it
is computationally infeasible to find 
1) a pre-image, i.e., a message corresponding to a given digest, or
2) two different messages that produce a collision, i.e., the same message digest.

the implementation is not to be considered production-ready. Use at own risk.

Any change to a message will, with a very high probability, result in a different message digest. This will result in a verification failure when the secure hash algorithm is used with a digital signature algorithm or a keyed-hash message authentication algorithm.