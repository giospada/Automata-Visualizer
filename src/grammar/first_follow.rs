use std::collections::BTreeSet;

use crate::grammar::{
    Grammar,
    Letter,
    Production,
    consts::{EPSILON, STRING_END, Terminal, NonTerminal},
};

pub struct FirstFollow {
    first_table: Option<Vec<BTreeSet<Terminal>>>,
    follow_table: Option<Vec<BTreeSet<Terminal>>>,
    nullable: Option<Vec<bool>>,

    num_non_terminal: usize,
}

impl FirstFollow {
    fn new(non_non_terminals: usize) -> Self {
        FirstFollow {
            first_table: None,
            follow_table: None,
            nullable: None,
            num_non_terminal: non_non_terminals,
        }
    }

    pub fn get_first(&self, letter: &Letter) -> BTreeSet<Terminal> {
        match letter {
            Letter::NonTerminal(idx) => self.first_table.as_ref().unwrap()[*idx].clone(),
            Letter::Terminal(ch) => {
                let mut set = BTreeSet::new();
                set.insert(*ch);
                set
            }
        }
    }

    pub fn get_follow(&self, non_terminal: NonTerminal) -> BTreeSet<Terminal> {
        self.follow_table.as_ref().unwrap()[non_terminal].clone()
    }

    fn compute_nullable(&mut self, grammar: &Grammar) {
        let nullable = grammar.get_nullable();
        self.nullable = Some(vec![false; self.num_non_terminal]);

        nullable.iter().for_each(|non_terminal| -> () {
            self.nullable.as_mut().unwrap()[*non_terminal] = true;
        });
    }

    fn compute_first(&mut self, grammar: &Grammar) {
        if let None = self.nullable {
            self.compute_nullable(grammar);
        }

        self.first_table = Some(vec![BTreeSet::new(); self.num_non_terminal]);
        let productions = grammar.get_productions();
        let mut has_changed = true;

        while has_changed {
            has_changed = false;

            productions.iter().for_each(|production| -> () {
                has_changed |= self.update_first_table(production);
            });
        }

        (0..self.num_non_terminal).for_each(|i| -> () {
            if self.nullable.as_ref().unwrap()[i] {
                self.first_table.as_mut().unwrap()[i].insert(EPSILON);
            }
        });
    }

    fn update_first_table(&mut self, production: &Production) -> bool {
        let mut has_changed = false;
        for letter in production.rhs.iter() {
            match letter {
                Letter::NonTerminal(idx) => {
                    let set_to_join = self.first_table.as_ref().unwrap()[*idx].clone();

                    has_changed |= Self::append_if_not_superset(
                        &mut self.first_table.as_mut().unwrap()[production.lhs],
                        set_to_join,
                    );
                    if !self.nullable.as_ref().unwrap()[*idx] {
                        break;
                    }
                }
                Letter::Terminal(ch) => {
                    has_changed |= self.first_table.as_mut().unwrap()[production.lhs].insert(*ch);
                    if *ch != EPSILON {
                        break;
                    }
                }
            }
        }

        has_changed
    }

    fn compute_follow(&mut self, grammar: &Grammar) {
        if let None = self.first_table {
            self.compute_first(grammar);
        }
        self.follow_table = Some(vec![BTreeSet::new(); self.num_non_terminal]);
        self.follow_table.as_mut().unwrap()[grammar.get_start_symbol()].insert(STRING_END);

        let productions = grammar.get_productions();
        let mut has_changed = true;
    
        while has_changed {
            has_changed = false;
            
            productions.iter().for_each(|production| {
                has_changed |= self.update_follow_table(production);
            });
        }
    }

    /// updates the follow table with the given production
    /// returns true if the follow table has changed, false otherwise
    fn update_follow_table(&mut self, production: &Production) -> bool {
        let mut has_changed = false;
        for (i, letter) in production.rhs.iter().enumerate() {
            match letter {
                Letter::NonTerminal(idx) => {
                    // if we are at the end of the production, then we need to add the follow of the lhs
                    if i == production.rhs.len() - 1 {
                        let to_join = self.follow_table.as_ref().unwrap()[production.lhs].clone();
                        has_changed |= Self::append_if_not_superset(
                            &mut self.follow_table.as_mut().unwrap()[*idx],
                            to_join,
                        );
                    } else {
                        // otherwise we need to add the first of the next symbol
                        let next_letter = &production.rhs[i + 1];
                        match next_letter {
                            Letter::NonTerminal(next) => {
                                let to_join = self.first_table.as_ref().unwrap()[*next].clone();
                                has_changed |= Self::append_if_not_superset(
                                    &mut self.follow_table.as_mut().unwrap()[*idx],
                                    to_join,
                                );
                            },
                            Letter::Terminal(ch) => {
                                self.follow_table.as_mut().unwrap()[*idx].insert(*ch);
                            }
                        }

                        // if the whole next symbol is nullable, then we need to add the follow of the lhs
                        if self.is_nullable(&mut production.rhs[i+1..].iter()) {
                            let to_join = self.follow_table.as_ref().unwrap()[production.lhs].clone();
                            has_changed |= Self::append_if_not_superset(
                                &mut self.follow_table.as_mut().unwrap()[*idx],
                                to_join,
                            );
                        }
                    }
                },
                Letter::Terminal(_) => {}
            }
        }

        has_changed
    }

    fn append_if_not_superset<T: Ord>(first_set: &mut BTreeSet<T>, second_set: BTreeSet<T>) -> bool {
        if first_set.is_superset(&second_set) {
            return false;
        }
        let mut mutable = second_set;

        first_set.append(&mut mutable);

        true
    }

    /// checks if the rest of the iterator is all nullable.
    /// assumes the nullable set has been initialized.
    fn is_nullable<'a, T: Iterator<Item = &'a Letter>>(&self, iter: &mut T) -> bool {
        iter.all(|letter| -> bool {
            match letter {
                Letter::NonTerminal(idx) => self.nullable.as_ref().unwrap()[*idx],
                Letter::Terminal(ch) => *ch == EPSILON,
            }
        })
    }
}


impl From<&Grammar> for FirstFollow {
    fn from(grammar: &Grammar) -> Self {
        let num_non_terminal = grammar.get_non_terminal().len();

        let mut first_follow = FirstFollow::new(num_non_terminal);

        first_follow.compute_nullable(grammar);
        first_follow.compute_first(grammar);
        first_follow.compute_follow(grammar);

        first_follow
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    fn get_test_grammar() -> Grammar {
        // S -> Ab | c
        // A -> aA | ε
        // S = 0
        // A = 1
        Grammar::new(
            0,
            vec![
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(1), Letter::Terminal('b')] },
                Production { lhs: 0, rhs: vec![Letter::Terminal('c')] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('a'), Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal(EPSILON)] },
            ]
        )
    }

    #[test]
    fn test_first() {
        let mut grammar = get_test_grammar();

        let first = grammar.first(&Letter::NonTerminal(0));

        assert_eq!(first.len(), 3);
        assert!(first.contains(&'a'));
        assert!(first.contains(&'b'));
        assert!(first.contains(&'c'));
    }

    #[test]
    fn test_first_cycle() {
        let mut grammar = Grammar::new(
            0,
            vec![
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::NonTerminal(0)] },
            ],
        );

        let first = grammar.first(&Letter::NonTerminal(0));

        assert_eq!(first.len(), 0);
    }

    #[test]
    fn test_follow() {
        let mut grammar = get_test_grammar();

        let follow = grammar.follow(&0);
        assert_eq!(follow.len(), 1);
        assert!(follow.contains(&STRING_END));

        let follow = grammar.follow(&1);
        assert_eq!(follow.len(), 1);
        assert!(follow.contains(&'b'));
    }
}