/// This file contains some general helper functions used
/// To implement grammar semplification and first and follows
use std::collections::{BTreeSet};

use super::{Grammar, NonTerminal, Letter, EPSILON};

impl Grammar {
    pub fn get_non_terminal(&self) -> BTreeSet<NonTerminal> {
        let mut non_terminals = BTreeSet::new();
        for production in self.productions.iter() {
            non_terminals.insert(production.lhs);
        }

        non_terminals
    }

    pub fn get_nullable(&self) -> BTreeSet<NonTerminal> {
        let mut nullable = BTreeSet::new();
        let mut has_changed = true;
        while has_changed {
            has_changed = false;
            for production in self.productions.iter() {
                let mut is_nullable = true;
                for letter in production.rhs.iter() {
                    match letter {
                        Letter::NonTerminal(idx) => {
                            if !nullable.contains(idx) {
                                is_nullable = false;
                                break;
                            }
                        }
                        Letter::Terminal(ch) => {
                            if *ch != EPSILON {
                                is_nullable = false;
                                break;
                            }
                        }
                    }
                }
                if is_nullable && !nullable.contains(&production.lhs) {
                    nullable.insert(production.lhs);
                    has_changed = true;
                }
            }
        }
        
        nullable
    }

    /// O(m^2) implementation of reachable function, could be optimized
    /// but i need to store adjacency list of the graph in grammar, and the
    /// use bfs.
    pub fn get_reachable(&self) -> BTreeSet<usize> {
        let mut reachable = BTreeSet::new();
        let mut has_changed = true;
        reachable.insert(self.start_symbol);
        while has_changed {
            has_changed = false;
            self.productions.iter().for_each(|production| -> () {
                if !reachable.contains(&production.lhs) {
                    return;
                }
                for letter in production.rhs.iter() {
                    match letter {
                        Letter::NonTerminal(idx) => {
                            if !reachable.contains(idx) {
                                reachable.insert(*idx);
                                has_changed = true;
                            }
                        }
                        Letter::Terminal(_) => {}
                    }
                }
            });
        }

        reachable
    }

    /// returns set of generator non terminals
    /// a non terminal is a generator when it produces some finite
    /// string of terminals
    /// This is still O(m^2) implementation, could be optimized
    pub fn get_generators(&self) -> BTreeSet<usize> {
        let mut generators = BTreeSet::new();
        let mut has_changed = true;

        while has_changed {
            has_changed = false;
            
            self.productions.iter().for_each(|production| -> () {
                let mut is_generator = true;
                production.rhs.iter().for_each(|letter| -> () {
                    match letter {
                        Letter::NonTerminal(non_terminal) => {
                            if !generators.contains(non_terminal) {
                                is_generator = false;
                                return;
                            }
                        }
                        Letter::Terminal(_) => {}
                    }
                });

                if is_generator {
                    generators.insert(production.lhs);
                    has_changed = true;
                }
            });
        }

        generators
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Production};
    
    fn get_test_grammar() -> Grammar {
        // S -> Ab | c
        // A -> aA | ε
        // S = 0
        // A = 1
        Grammar {
            start_symbol: 0,
            productions: vec![
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(1), Letter::Terminal('b')] },
                Production { lhs: 0, rhs: vec![Letter::Terminal('c')] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('a'), Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal(EPSILON)] },
            ],
            nullable: None,
        }
    }

    #[test]
    fn test_nullable() {
        let grammar = get_test_grammar();

        let nullable = grammar.get_nullable();
        assert_eq!(nullable.len(), 1);
        assert!(nullable.contains(&1));
    }
}