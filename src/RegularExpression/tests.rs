use super::*;
mod next_token {
    use super::*;
    #[test]
    fn only_char() {
        let mut chars = "aaaa".chars().peekable();
        let token = get_next_token(&mut chars).unwrap();
        assert_eq!(token, "aaaa");
    }

    #[test]
    fn with_star_and_or() {
        let mut chars = "aa|b*".chars().peekable();
        let token = get_next_token(&mut chars).unwrap();
        assert_eq!(token, "aa");
    }

    #[test]
    fn with_parenthesis() {
        let mut chars = "aaa(a|b*)".chars().peekable();
        let token = get_next_token(&mut chars).unwrap();
        assert_eq!(token, "aaa");
    }

    #[test]
    fn begin_parens() {
        let mut chars = "(a|b*)".chars().peekable();
        let token = get_next_token(&mut chars).unwrap();
        assert_eq!(token, "");
    }
}

mod token_parse {
    use super::*;

    #[test]
    fn only_char() {
        let token = "aaaa".to_string();
        let tree = parse_token(token).unwrap();

        // this should be
        // a
        // |  a
        // c /
        // |  a
        // c /
        // |  a
        // c /
        // |
        // start

        assert_eq!(
            *tree,
            ReOperator::Concat(
                Box::new(ReOperator::Concat(
                    Box::new(ReOperator::Concat(
                        Box::new(ReOperator::Char('a')),
                        Box::new(ReOperator::Char('a')),
                    )),
                    Box::new(ReOperator::Char('a')),
                )),
                Box::new(ReOperator::Char('a')),
            )
        );
    }

    #[test]
    fn kleene_star() {
        let token = "da*b".to_string();
        let tree = parse_token(token).unwrap();

        // this should be
        // d  a
        // |  |
        // |  k
        // c /
        // |  b
        // c /
        // |
        // start
        // debug print tree
        assert_eq!(
            *tree,
            ReOperator::Concat(
                Box::new(ReOperator::Concat(
                    Box::new(ReOperator::Char('d')),
                    Box::new(ReOperator::KleeneStar(Box::new(ReOperator::Char('a')),)),
                )),
                Box::new(ReOperator::Char('b')),
            )
        );
    }

    #[test]
    fn error_double_star() {
        let token = "ab**c".to_string();
        let tree = parse_token(token);
        assert!(tree.is_err());
    }
}

mod parse_all {
    use super::*;

    #[test]
    fn parse_and_parentesis() {
        let str = "a(b|c)".to_string();
        let tree = ReOperator::from_string(&str).unwrap();

        let answer = ReOperator::Concat(
            Box::new(ReOperator::Char('a')),
            Box::new(ReOperator::Or(
                Box::new(ReOperator::Char('b')),
                Box::new(ReOperator::Char('c')),
            )),
        );

        assert_eq!(tree, answer);
    }

    #[test]
    fn parens_at_end_should_err() {
        let str = "a(b|c".to_string();
        let tree = ReOperator::from_string(&str);
        assert!(tree.is_err());

        let str = "a(b|c))".to_string();
        let tree = ReOperator::from_string(&str);
        assert!(tree.is_err());
    }

    #[test]
    fn empty_brakets() {
        let str = "a()".to_string();
        let tree = ReOperator::from_string(&str);
        assert!(tree.is_err());
    }

    #[test]
    fn brakets_with_star() {
        let str = "(a|b)*".to_string();
        let tree = ReOperator::from_string(&str);
        assert!(!tree.is_err());
    }
}
