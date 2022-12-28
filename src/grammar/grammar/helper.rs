/// This file contains some general helper functions used
/// To implement grammar semplification and first and follows
use std::collections::BTreeSet;

use super::{Grammar, Letter, NonTerminal, Production, EPSILON};

impl Grammar {
    pub fn get_non_terminal(&self) -> BTreeSet<NonTerminal> {
        let mut non_terminals = BTreeSet::new();
        for production in self.productions.iter() {
            non_terminals.insert(production.start_symbol);
        }

        non_terminals
    }

    pub fn get_nullable(&self) -> BTreeSet<NonTerminal> {
        let mut nullable = BTreeSet::new();
        let mut has_changed = true;
        while has_changed {
            has_changed = false;
            for production in self.productions.iter() {
                if production.check_is_nullable(&nullable)
                    && !nullable.contains(&production.start_symbol)
                {
                    nullable.insert(production.start_symbol);
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
                if !reachable.contains(&production.start_symbol) {
                    return;
                }
                for letter in production.expand_rule.iter() {
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
                production.expand_rule.iter().for_each(|letter| -> () {
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
                    generators.insert(production.start_symbol);
                    has_changed = true;
                }
            });
        }

        generators
    }

    /// returns set of unitary couples of non terminals
    /// a unitary couple is a couple of non terminals (A, B) such that
    /// A -> B is a production in the grammar or A -> C, C -> B is a production
    /// (aka it's transitive and reflexive)
    pub fn get_unitary_couples(&self) -> BTreeSet<(NonTerminal, NonTerminal)> {
        let non_terminals = self.get_non_terminal();
        let mut unitary_couples = BTreeSet::new();
        let mut has_changed = true;

        for non_terminal in non_terminals {
            unitary_couples.insert((non_terminal, non_terminal));
        }

        while has_changed {
            has_changed = false;
            for production in self.productions.iter() {
                if production.expand_rule.len() != 1 {
                    continue;
                }
                let mut to_insert = BTreeSet::new();
                for unitary_couple in unitary_couples.iter() {
                    if let Letter::NonTerminal(non_term) = production.expand_rule[0] {
                        if unitary_couple.1 == production.start_symbol
                            && !unitary_couples.contains(&(unitary_couple.0, non_term))
                            && !to_insert.contains(&(unitary_couple.0, non_term))
                        {
                            to_insert.insert((unitary_couple.0, non_term));
                        }
                    }
                }

                if to_insert.len() > 0 {
                    unitary_couples.append(&mut to_insert);
                    has_changed = true;
                }
            }
        }

        unitary_couples
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::Production;

    fn get_test_grammar() -> Grammar {
        // S -> Ab | c
        // A -> aA | ε
        // S = 0
        // A = 1
        Grammar {
            start_symbol: 0,
            productions: vec![
                Production {
                    start_symbol: 0,
                    expand_rule: vec![Letter::NonTerminal(1), Letter::Terminal('b')],
                },
                Production {
                    start_symbol: 0,
                    expand_rule: vec![Letter::Terminal('c')],
                },
                Production {
                    start_symbol: 1,
                    expand_rule: vec![Letter::Terminal('a'), Letter::NonTerminal(1)],
                },
                Production {
                    start_symbol: 1,
                    expand_rule: vec![Letter::Terminal(EPSILON)],
                },
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
