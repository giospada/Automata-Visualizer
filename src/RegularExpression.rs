use crate::Error::*;
use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;
use crate::DisplayGraph::*;

#[cfg(test)]
mod tests;

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
    fn childs(&self)->Vec<&Self>{
        match self{
            ReOperator::Char(_) => Vec::new(),
            ReOperator::Concat(b1,b2) => vec![b1,b2],
            ReOperator::Or(b1,b2) => vec![b1,b2],
            ReOperator::KleeneStar(b) => vec![b]
        }
    }
}

impl ToDisplayGraph for ReOperator{
    // fa una bfs 
     fn to_display_graph(&self) -> DisplayGraph{
        let mut child =vec![];
        let mut graph=vec![];
        let mut labels=vec![];
        let mut edge:Vec<(usize,usize,Option<String>)>=Vec::new();
        let mut number_nodes=1 as usize;
        graph.push(vec![0 as usize]);        
        child.push((0,self));
        while !child.is_empty() {
            let mut current_nodes=vec![];
            let mut newchild =vec![];    
            
            for (index,node) in child{
                current_nodes.push(index);
                labels.push(node.label());
                
                for child in node.childs(){
                    edge.push((index,number_nodes,None));
                    newchild.push((number_nodes,child));
                    number_nodes+=1;
                }

            }
            graph.push(current_nodes);
            child=newchild;
        }
        DisplayGraph::new(edge,labels,graph)
    }
    
}

/// parse until closing parens or end of string
/// concatenation is left associative
/// or is right associative
/// kleene is an operation on a single character, or on parens.
fn parse_rec(chars: &mut Peekable<Chars>, open_parens: &mut i32) -> Result<Box<ReOperator>, Box<dyn Error>> {
    if open_parens < &mut 0 {
        return Err(Box::new(UnvalidParentesis {}));
    }

    let mut parse_tree = elaborate_next_token(chars, None, open_parens)?;
    let curr_parentesis = open_parens.clone();

    while chars.peek().is_some() {
        parse_tree = elaborate_next_token(chars, Some(parse_tree), open_parens)?;

        if curr_parentesis > open_parens.clone() {
            break;
        }
    }

    Ok(parse_tree)
}

fn elaborate_next_token(chars: &mut Peekable<Chars>, mut tree: Option<Box<ReOperator>>, open_parens: &mut i32) -> Result<Box<ReOperator>, Box<dyn Error>> {
    if chars.peek().is_none() {
        if let Some(t) = tree {
            return Ok(t);
        } else {
            return Err(Box::new(InvalidTokenError::new("Empty string is not accepted".to_string())));
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
                tree = Some(Box::new(ReOperator::Concat(parse_tree, 
                    Box::new(ReOperator::KleeneStar(next_tree)))));
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

    let mut chars = token.chars().peekable();
    let mut tree_top = get_next_node(&mut chars)?;
    
    while chars.peek().is_some() {
        tree_top = Box::new(ReOperator::Concat(tree_top, get_next_node(&mut chars)?));
    }

    Ok(tree_top)
}

/// assume that at least one char is still available.
fn get_next_node(chars: &mut Peekable<Chars>) -> Result<Box<ReOperator>, Box<dyn Error>> {
    let curr_char = chars.next().unwrap();
    if !is_valid_char(curr_char) {
        return Err(Box::new(InvalidCharacter::new(curr_char)));
    }

    match curr_char {
        'a'..='z' | 'A'..='Z' | '0'..='9' => {
            if chars.peek() == Some(&'*') {
                chars.next();
                return Ok(Box::new(ReOperator::KleeneStar(Box::new(ReOperator::Char(curr_char)))));
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

fn is_valid_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '|' || c == '*' || c == '(' || c == ')'
}
