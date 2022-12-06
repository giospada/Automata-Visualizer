use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;

use crate::display::display_graph::DisplayGraph;
use crate::error::{InvalidCharacter, InvalidTokenError, UnvalidParentesis};
use crate::utils::graph::{Graph, IndNode};

#[derive(Debug, Clone)]
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

impl ReOperator {
    pub fn from_string(str: &String) -> Result<ReOperator, Box<dyn Error>> {
        let mut chars = str.chars().peekable();
        let mut num_parens = 0;
        let res = parse_rec(&mut chars, &mut num_parens)?;

        if num_parens != 0 {
            return Err(Box::new(UnvalidParentesis {}));
        }

        Ok(*res)
    }

    fn label(&self) -> String {
        match self {
            ReOperator::Char(c) => c.to_string(),
            ReOperator::Concat(_, _) => "·".to_string(),
            ReOperator::Or(_, _) => "|".to_string(),
            ReOperator::KleeneStar(_) => "*".to_string(),
        }
    }
    fn childs(&self) -> Vec<&Self> {
        match self {
            ReOperator::Char(_) => Vec::new(),
            ReOperator::Concat(b1, b2) => vec![b1, b2],
            ReOperator::Or(b1, b2) => vec![b1, b2],
            ReOperator::KleeneStar(b) => vec![b],
        }
    }
    fn build_recursive_graph(&self, graph: &mut Graph) -> IndNode {
        let top = graph.add_node(Some(self.label()));
        for rep in self.childs() {
            let child = rep.build_recursive_graph(graph);
            graph.add_edge(top, child, None);
        }
        top
    }
}

impl Into<Graph> for ReOperator {
    fn into(self) -> Graph {
        let mut g = Graph::new();
        self.build_recursive_graph(&mut g);
        g
    }
}

/// parse until closing parens or end of string
/// concatenation is left associative
/// or is right associative
/// kleene is an operation on a single character, or on parens.

/// 
/// this parser return the re operator of the CURRENT SCOPE
/// this means that if ((aaa)bbb), it returns the re operator for this
/// scope, which is (aaa)bbb, and it's called recursively on the scope of aaa
/// when it sees other scoping character.
/// 
/// Scoping caracters are ( and |, these two caracters are used to enter in new scope
/// the caracter ) is used to exit from the current scope.
fn parse_rec(chars: &mut Peekable<Chars>, open_parens: &mut i32) -> Result<Box<ReOperator>, Box<dyn Error>> {
    if open_parens < &mut 0 {
        return Err(Box::new(UnvalidParentesis {}));
    }

    let mut parse_tree = elaborate_next_token(chars, None, open_parens)?;
    let curr_parentesis = open_parens.clone();

    while chars.peek().is_some() {
        parse_tree = elaborate_next_token(chars, Some(parse_tree), open_parens)?;

        // this means that the recursive call has closed the parens
        // so the scope of the current parentesis is over, and we should return.
        if curr_parentesis > open_parens.clone() {
            break;
        }
    }

    Ok(parse_tree)
}


/// returns the re operator for a single token, described by the same grammar in [parse_token]
fn elaborate_next_token(
    chars: &mut Peekable<Chars>, 
    mut tree: Option<Box<ReOperator>>, 
    open_parens: &mut i32
) -> Result<Box<ReOperator>, Box<dyn Error>> {
    if chars.peek().is_none() {
        if let Some(t) = tree {
            return Ok(t);
        } else {
            return Err(Box::new(InvalidTokenError::new(
                "Empty string is not accepted".to_string(),
            )));
        }
    }

    let curr_char = chars.peek().unwrap();
    if !is_valid_char(*curr_char) {
        return Err(Box::new(InvalidCharacter::new(*curr_char)));
    }

    if *curr_char == '(' {
        chars.next();
        *open_parens += 1;
        let next_tree = parse_rec(chars, open_parens)?;

        if chars.peek() == Some(&'*') {
            chars.next();
            if let Some(parse_tree) = tree {
                tree = Some(Box::new(ReOperator::Concat(
                    parse_tree,
                    Box::new(ReOperator::KleeneStar(next_tree)),
                )));
            } else {
                tree = Some(Box::new(ReOperator::KleeneStar(next_tree)));
            }
        } else if let Some(parse_tree) = tree {
            tree = Some(Box::new(ReOperator::Concat(parse_tree, next_tree)));
        } else {
            tree = Some(next_tree);
        }
    } else if *curr_char == '|' {
        chars.next();

        if let Some(parse_tree) = tree {
            // può ritornare solamente per ")" oppure fine stringa
            let next_tree = parse_rec(chars, open_parens)?;
            tree = Some(Box::new(ReOperator::Or(parse_tree, next_tree)));
        } else {
            return Err(Box::new(InvalidTokenError::new(
                "Invalid token, cannot begin with |".to_string(),
            )));
        }
    } else if *curr_char == ')' {
        chars.next();
        *open_parens -= 1;
    } else {
        let token = get_next_token(chars)?;

        // quando è vuoto vuol dire che il prossimo carattere è (, ), | o
        // fine stringa, quindi non devo fare nulla, lo gestisco alla prossima iterazione.
        if token.len() > 0 {
            let next_tree = parse_token(token)?;

            if let Some(parse_tree) = tree {
                tree = Some(Box::new(ReOperator::Concat(parse_tree, next_tree)));
            } else {
                tree = Some(next_tree);
            }
        }
    }

    if let Some(parse_tree) = tree {
        Ok(parse_tree)
    } else {
        Err(Box::new(InvalidTokenError::new(
            "Empty token Error".to_string(),
        )))
    }
}

/// this function returns the next token in the regexp, and advances the chars iterator accordingly
/// 
/// remember that a token is described by the same grammar of [parse_token]
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

/// token is a valid regular expression without parenthesis nor Or operator
/// Described by the following grammar:
/// 
/// S -> ε | A | S(* | S)
/// 
/// A -> [a-z] | [A-Z] | [0-9]
/// 
fn parse_token(token: String) -> Result<Box<ReOperator>, Box<dyn Error>> {
    if token.len() == 0 {
        return Err(Box::new(InvalidTokenError::new("Empty token".to_string())));
    } else if token.starts_with("*") {
        return Err(Box::new(InvalidTokenError::new(
            "token can't start with *".to_string(),
        )));
    }

    let mut chars = token.chars().peekable();
    let mut tree_top = get_next_node(&mut chars)?;

    while chars.peek().is_some() {
        tree_top = Box::new(ReOperator::Concat(tree_top, get_next_node(&mut chars)?));
    }

    Ok(tree_top)
}

/// a node is defined as a single character in the regexp alfabet or couple <char, kleene star>
/// this function returns the next node in the regexp, and advances the chars iterator accordingly
/// 
/// example:
/// 
/// if chars is a, b, *
/// when it's it just returns a node with label a
/// 
/// when it's at b it returns a correct box operator with label * and b as child
/// 
fn get_next_node(chars: &mut Peekable<Chars>) -> Result<Box<ReOperator>, Box<dyn Error>> {
    let curr_char = chars.next().unwrap();
    if !is_valid_char(curr_char) {
        return Err(Box::new(InvalidCharacter::new(curr_char)));
    }

    match curr_char {
        'a'..='z' | 'A'..='Z' | '0'..='9' => {
            if chars.peek() == Some(&'*') {
                chars.next();
                return Ok(Box::new(ReOperator::KleeneStar(Box::new(
                    ReOperator::Char(curr_char),
                ))));
            } else {
                return Ok(Box::new(ReOperator::Char(curr_char)));
            }
        }
        '*' => {
            return Err(Box::new(InvalidTokenError::new(
                "cannot have Kleene star without valid alphabet char".to_string(),
            )));
        }
        _ => {
            return Err(Box::new(InvalidTokenError::new(
                "Invalid character, only ([a-z]|[A-Z]|[0-9]|\\*)* is accepted".to_string(),
            )));
        }
    }
}



fn is_valid_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '|' || c == '*' || c == '(' || c == ')'
}

#[cfg(test)]
mod test {
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
}
