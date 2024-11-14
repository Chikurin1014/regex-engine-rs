//! 正規表現の式をパースし、抽象構文木に変換

mod error;
mod parse;

pub use error::ParseError;
pub use parse::{parse, Ast};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expr = "a|b|c";
        let ast = parse(expr).unwrap();
        assert_eq!(
            ast,
            Ast::Or(
                Box::new(Ast::Or(Box::new(Ast::Char('a')), Box::new(Ast::Char('b')))),
                Box::new(Ast::Char('c'))
            )
        );

        let expr = "a|b|c|";
        let ast = parse(expr);
        assert!(ast.is_err());

        let expr = "a|b|c|d";
        let ast = parse(expr).unwrap();
        assert_eq!(
            ast,
            Ast::Or(
                Box::new(Ast::Or(
                    Box::new(Ast::Or(Box::new(Ast::Char('a')), Box::new(Ast::Char('b')))),
                    Box::new(Ast::Char('c'))
                )),
                Box::new(Ast::Char('d'))
            )
        );

        let expr = "a|b|c|d|";
        let ast = parse(expr);
        assert!(ast.is_err());

        let expr = "a|b|c|d|e";
        let ast = parse(expr).unwrap();
        assert_eq!(
            ast,
            Ast::Or(
                Box::new(Ast::Or(
                    Box::new(Ast::Or(
                        Box::new(Ast::Or(Box::new(Ast::Char('a')), Box::new(Ast::Char('b')))),
                        Box::new(Ast::Char('c'))
                    )),
                    Box::new(Ast::Char('d')),
                )),
                Box::new(Ast::Char('e')),
            )
        );

        let expr = "a|b|c|d|e|";
        let ast = parse(expr);
        assert!(ast.is_err());
    }
}
