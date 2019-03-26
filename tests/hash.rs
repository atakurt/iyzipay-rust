pub mod hash_test {
    use iyzipay_rust::hash::HashGenerator;

    #[test]
    fn should_generate_hash() {
        assert_eq!("Cy84UuLZpfGhI7oaPD0Ckx1M0mo=", HashGenerator::generate_hash("apiKey", "secretKey", "random", "[data=value]"));
    }
}