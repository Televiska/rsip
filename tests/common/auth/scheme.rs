pub mod tokenizer {
    use rsip::common::auth::scheme::Tokenizer;

    #[test]
    fn with_str_input() -> Result<(), rsip::Error> {
        assert_eq!(Tokenizer::from("basic"), Tokenizer::tokenize("basic")?.1);
        assert_eq!(Tokenizer::from("basic"), Tokenizer::tokenize("basic  ")?.1);

        Ok(())
    }

    #[test]
    fn with_bytes_input() -> Result<(), rsip::Error> {
        assert_eq!(
            Tokenizer::from("basic".as_bytes()),
            Tokenizer::tokenize("basic".as_bytes())?.1
        );
        assert_eq!(
            Tokenizer::from("basic".as_bytes()),
            Tokenizer::tokenize("basic  ".as_bytes())?.1
        );

        Ok(())
    }
}
