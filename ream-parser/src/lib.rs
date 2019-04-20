use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub scheme);
pub mod ast;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert!(scheme::ExprsParser::new().parse("(22)").is_ok());
        assert!(scheme::ExprsParser::new().parse("1523665908450923").is_ok());
        assert!(scheme::ExprsParser::new().parse("12349871239487").is_ok());
    }

    #[test]
    fn arithmetic() {
        assert!(scheme::ExprsParser::new().parse("(+ 10 20)").is_ok());
        assert!(scheme::ExprsParser::new().parse("(- 432 6456)").is_ok());
        assert!(scheme::ExprsParser::new().parse("(* 86234 4646)").is_ok());
        assert!(scheme::ExprsParser::new().parse("(/ 243 54732)").is_ok());
    }

    #[test]
    fn line_comments() {
        let code: &str = "
        ;Lorem ipsum
        (+ 10 40)

        ; dolor sit amet
        (+ 1 80)

        ;consectetur adipiscing elit
        ;Donec rutrum, tellus non euismod accumsan";
        scheme::ExprsParser::new().parse(code).unwrap();
    }
    #[test]
    fn block_comments() {
        let code: &str = "
        #|
            Lorem ipsum
        |#

        (+ 10 40)

        #|dolor sit amet
        consectetur adipiscing elit
        Donec rutrum, tellus non euismod accumsan|#";
        scheme::ExprsParser::new().parse(code).unwrap();
    }
}
