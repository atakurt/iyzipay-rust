pub mod iyzi_auth_v2_generator_test {
    use iyzipay_rust::hash::IyziAuthV2Generator;

    #[test]
    fn should_generate_hash() {
        assert_eq!("YXBpS2V5OmFwaUtleSZyYW5kb21LZXk6cmFuZG9tJnNpZ25hdHVyZTo0YWZhMjhjYjE3NTkwNThlYWEzNjNhZGVkNjAzM2NhNTg0N2NmNDYxODNhZDdiYTI5ZDEwZjE3ZWNiMGJmY2M4", IyziAuthV2Generator::generate_auth_content("/v2/uri?test=true", "apiKey", "secretKey", "random", "{\"data\":\"value\"}"));
    }

    #[test]
    fn should_generate_same_hash_when_uri_do_not_have_query_parameters() {
        assert_eq!("YXBpS2V5OmFwaUtleSZyYW5kb21LZXk6cmFuZG9tJnNpZ25hdHVyZTo0YWZhMjhjYjE3NTkwNThlYWEzNjNhZGVkNjAzM2NhNTg0N2NmNDYxODNhZDdiYTI5ZDEwZjE3ZWNiMGJmY2M4", IyziAuthV2Generator::generate_auth_content("/v2/uri", "apiKey", "secretKey", "random", "{\"data\":\"value\"}"));
    }

    #[test]
    fn should_generate_hash_when_given_request_body_is_empty_request() {
        assert_eq!("YXBpS2V5OmFwaUtleSZyYW5kb21LZXk6cmFuZG9tJnNpZ25hdHVyZTpjOWU1OTI2NjE4ODNlY2NkYjEzYmEwOGFhYTdhNTJiMDhmZTFkNDhkZTU2OGZmNDgxZDZmOGM3ZWFkMjkzN2Uy", IyziAuthV2Generator::generate_auth_content("/v2/uri?test=true", "apiKey", "secretKey", "random", ""));
    }
}