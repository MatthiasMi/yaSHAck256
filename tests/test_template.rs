#[cfg(test)]
mod tests {

    use yashack256::Sha256;

    /// `test_template()` is a test case that asserts that 1 + 2 equals 3.
    #[test]
    fn test_template() {
        assert_eq!(1 + 2, 3);
    }

    /// `test_correctness` tests if `SHA256(msg) == md`, with `len(msg) == len`.
    fn test_correctness(len: usize, msg: Vec<u8>, md: String) -> bool {
        let mut h = Sha256::new();
        let m: &mut Vec<u8> = &mut msg.clone();

        h.update(m);
        let hash = h.to_string();
        assert_eq!(hash, md);
        return true;
    }

    /// `test_vectors` tests the implementation against a few hard-coded example vectors.
    #[test]
    fn test_vectors() {
        // Run via:
        // cargo test --package yashack256 --bin yashack256 -- test_vectors --exact --nocapture
        // or simply:
        // cargo test --release -- test_vectors

        let examples = vec![
        ( 0, "", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        ( 8,
          "d3", //hex::decode("d3").unwrap(),  // = 211 != "d3".as_bytes().to_vec()
         "28969cdfa74a12c82f3bad960b0b000aca2ac329deea5c2328ebc6f2ba9802c1",
        ),
        (1304, "451101250ec6f26652249d59dc974b7361d571a8101cdfd36aba3b5854d3ae086b5fdd4597721b66e3c0dc5d8c606d9657d0e323283a5217d1f53f2f284f57b85c8a61ac8924711f895c5ed90ef17745ed2d728abd22a5f7a13479a462d71b56c19a74a40b655c58edfe0a188ad2cf46cbf30524f65d423c837dd1ff2bf462ac4198007345bb44dbb7b1c861298cdf61982a833afc728fae1eda2f87aa2c9480858bec", "3c593aa539fdcdae516cdf2f15000f6634185c88f505b39775fb9ab137a10aa2"),
        (2096, "6b918fb1a5ad1f9c5e5dbdf10a93a9c8f6bca89f37e79c9fe12a57227941b173ac79d8d440cde8c64c4ebc84a4c803d198a296f3de060900cc427f58ca6ec373084f95dd6c7c427ecfbf781f68be572a88dbcbb188581ab200bfb99a3a816407e7dd6dd21003554d4f7a99c93ebfce5c302ff0e11f26f83fe669acefb0c1bbb8b1e909bd14aa48ba3445c88b0e1190eef765ad898ab8ca2fe507015f1578f10dce3c11a55fb9434ee6e9ad6cc0fdc4684447a9b3b156b908646360f24fec2d8fa69e2c93db78708fcd2eef743dcb9353819b8d667c48ed54cd436fb1476598c4a1d7028e6f2ff50751db36ab6bc32435152a00abd3d58d9a8770d9a3e52d5a3628ae3c9e0325", "46500b6ae1ab40bde097ef168b0f3199049b55545a1588792d39d594f493dca7"),
    ];
        examples.iter().for_each(|(len, msg, md)| {
            test_correctness(*len, hex::decode(msg).unwrap(), md.to_string());
        });
    }
}
