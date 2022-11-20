use crate::SyntaxTree::*;
use crate::Error::*;
use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum ReOperator {
    Char(char),
    Concat(Box<ReOperator>, Box<ReOperator>),
    Or(Box<ReOperator>, Box<ReOperator>),
    KleeneStar(Box<ReOperator>),
}

impl PartialEq for ReOperator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ReOperator::Char(c1), ReOperator::Char(c2)) => c1 == c2,
            (ReOperator::Concat(b11, b12), ReOperator::Concat(b21, b22)) => {
                b11 == b21 && b12 == b22
            }
            (ReOperator::Or(b11, b12), ReOperator::Or(b21, b22)) => b11 == b21 && b12 == b22,
            (ReOperator::KleeneStar(b1), ReOperator::KleeneStar(b2)) => b1 == b2,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


impl ToSingleTree for ReOperator {
    fn to_syntax_tree(&self) -> SyntaxTree {
        let mut syntax = SyntaxTree::from_label(self.label());
        match self {
            ReOperator::Char(c) => {}
            ReOperator::Concat(left, right) => {
                syntax.children.push(left.to_syntax_tree());
                syntax.children.push(right.to_syntax_tree());
            }
            ReOperator::Or(left, right) => {
                syntax.children.push(left.to_syntax_tree());
                syntax.children.push(right.to_syntax_tree());
            }
            ReOperator::KleeneStar(op) => {
                syntax.children.push(op.to_syntax_tree());
            }
        }
        syntax
    }
    fn label(&self) -> String {
        match self {
            ReOperator::Char(c) => c.to_string(),
            ReOperator::Concat(l, r) => "+".to_string(),
            ReOperator::Or(l, r) => "|".to_string(),
            ReOperator::KleeneStar(r) => "*".to_string(),
        }
    }
}


pub fn parse(mut str: String) -> Result<Box<ReOperator>, Box<dyn Error>> {
    str.push(')'); // end character for the parse.
    let mut chars = str.chars().peekable();
    let res = parse_rec(&mut chars)?;

    if chars.peek().is_some() {
        return Err(Box::new(InvalidTokenError::new(
            "Invalid token, probably extra parentesis at the end.".to_string(),
        )));
    }

    Ok(res)
}

/// parse until closing parens or end of string
/// concatenation is left associative
/// or is right associative
/// kleene is an operation on a single character, or on parens.
fn parse_rec(chars: &mut Peekable<Chars>) -> Result<Box<ReOperator>, Box<dyn Error>> {
    let mut token = get_next_token(chars)?;
    let mut parse_tree = parse_token(token)?;

    loop {
        match chars.next() {
            Some(c) => {
                if !is_valid_char(c) {
                    return Err(Box::new(InvalidCharacter::new(c)));
                } else if c == '(' {
                    let next_tree = parse_rec(chars)?;
                    
                    if (chars.peek() == Some(&'*')) {
                        chars.next();
                        parse_tree = Box::new(ReOperator::Concat(parse_tree, 
                            Box::new(ReOperator::KleeneStar(next_tree))));
                    } else {
                        parse_tree = Box::new(ReOperator::Concat(parse_tree, next_tree));
                    }
                } else if c == '|' {
                    if let ReOperator::Or(_, _) = &*parse_tree {
                        return Err(Box::new(InvalidTokenError::new(
                            "Invalid token, cannot have two or operators in a row.".to_string(),
                        )));
                    }

                    let next_tree = parse_rec(chars)?;
                    parse_tree = Box::new(ReOperator::Or(parse_tree, next_tree));
                } else if c == ')' {
                    break;
                }

                token = get_next_token(chars)?;

                // quando è vuoto vuol dire che il prossimo carattere è (, ), | o
                // fine stringa, quindi non devo fare nulla, lo gestisco alla prossima iterazione.
                if token.len() > 0 {
                    let next_tree = parse_token(token)?;
                    parse_tree = Box::new(ReOperator::Concat(parse_tree, next_tree));
                }
            }
            None => break,
        }
    }

    Ok(parse_tree)
}

/// token is a valid regular expression without parenthesis nor Or operator
/// Described by
/// S -> ε | A | S(* | S)
/// A -> [a-z] | [A-Z] | [0-9]
fn parse_token(token: String) -> Result<Box<ReOperator>, Box<dyn Error>> {
    if token.len() == 0 {
        return Err(Box::new(InvalidTokenError::new("Empty token".to_string())));
    } else if token.starts_with("*") {
        return Err(Box::new(InvalidTokenError::new("token can't start with *".to_string())));
    }

    let epsilon = '\0';
    let mut chars = token.chars();
    let mut current_char_opt = chars.next();
    
    if !is_valid_char(current_char_opt.unwrap()) {
        return Err(Box::new(InvalidTokenError::new("Invalid starting char".to_string())));
    }

    let mut tree_top = Box::new(ReOperator::Char(current_char_opt.unwrap())); // NOTE: better way to do this?
    current_char_opt = chars.next();
    let mut next_char_opt = chars.next();

    while current_char_opt.is_some() {
        let current_char = current_char_opt.unwrap();
        let next_char = next_char_opt.unwrap_or(epsilon);
        let mut has_advanced = false;

        if next_char == '*' {
            match current_char {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    let op = ReOperator::KleeneStar(Box::new(ReOperator::Char(current_char)));
                    tree_top = Box::new(ReOperator::Concat(tree_top, Box::new(op)));
                    current_char_opt = chars.next();
                    next_char_opt = chars.next();

                    has_advanced = true;
                }
                '*' => {
                    return Err(Box::new(InvalidTokenError::new(
                        "* cannot follow *".to_string(),
                    )));
                }
                _ => {
                    return Err(Box::new(InvalidTokenError::new(
                        "Invalid character before *".to_string(),
                    )));
                }
            }
        } 
        
        if !has_advanced {
            tree_top = Box::new(ReOperator::Concat(tree_top, Box::new(ReOperator::Char(current_char))));
            current_char_opt = next_char_opt;
            next_char_opt = chars.next();
        }
    }

    Ok(tree_top)
}

fn get_next_token(chars: &mut Peekable<Chars>) -> Result<String, Box<dyn Error>> {
    let mut token = String::new();

    while chars.peek().is_some() && chars.peek().unwrap() != &'|' &&
        chars.peek().unwrap() != &')' && chars.peek().unwrap() != &'(' {
        let ch = chars.next().unwrap();

        if !is_valid_char(ch) {
            return Err(Box::new(InvalidTokenError::new(
                "Invalid character in token".to_string(),
            )));
        }
        token.push(ch);
    }

    Ok(token)
}

/// divides the string into tokens, so it drops the parenthesis
/// TODO: questa è la prima cosa che ho fatto, non per ora la tengo
/// Non so se servirà!
#[allow(dead_code)]
fn tokenize(str: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut tokens = Vec::new();
    let mut token = String::new();

    let mut open_parenthesis = 0;
    for curr_char in str.chars() {
        if !is_valid_char(curr_char) {
            return Err(Box::new(InvalidCharacter::new(curr_char)));
        }

        if curr_char == '(' {
            open_parenthesis += 1;
        } else if curr_char == ')' {
            open_parenthesis -= 1;
            if token.len() > 0 {
                tokens.push(token);
                token = String::new();
            }

            if open_parenthesis < 0 {
                // NOTE: maybe we should return the index of error parens??
                return Err(Box::new(UnvalidParentesis {}));
            }
        } else {
            token.push(curr_char);
        }
    }

    if open_parenthesis > 0 {
        return Err(Box::new(UnvalidParentesis {}));
    }

    Ok(tokens)
}

fn is_valid_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '|' || c == '*' || c == '(' || c == ')'
}

#[cfg(test)]
mod tests {
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

            assert_eq!(*tree, 
                ReOperator::Concat(
                Box::new(ReOperator::Concat(
                    Box::new(ReOperator::Concat(
                        Box::new(ReOperator::Char('a')),
                        Box::new(ReOperator::Char('a')),
                    )),
                    Box::new(ReOperator::Char('a')),
                )),
                Box::new(ReOperator::Char('a')),
            ));
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
            assert_eq!(*tree, 
                ReOperator::Concat(
                Box::new(ReOperator::Concat(
                    Box::new(ReOperator::Char('d')),
                    Box::new(ReOperator::KleeneStar(
                        Box::new(ReOperator::Char('a')),
                    )),
                )),
                Box::new(ReOperator::Char('b')),
            ));
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
            let tree = parse(str).unwrap();
            // Non so che albero dovrebbe venire :D
            // TODO: finire il test con l'albero corretto
        }
    }
}