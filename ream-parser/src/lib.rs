use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub scheme);

mod tests {
    use crate::scheme;

    #[test]
    fn num_terminal() {
        assert!(scheme::NumParser::new().parse("22").is_ok());
        assert!(scheme::NumParser::new().parse("1523665908450923").is_ok());
        assert!(scheme::NumParser::new().parse("12349871239487").is_ok());
        assert!(scheme::NumParser::new().parse("ABCDEF123").is_err());
    }
}
