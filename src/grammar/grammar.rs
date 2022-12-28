use std::collections::{BTreeMap, BTreeSet};

use crate::automata::DFA;
use crate::grammar::consts::{NonTerminal, Terminal, EPSILON, STRING_END};
use crate::grammar::{Letter, Production};

mod helper;
mod semplification;

#[derive(Debug, PartialEq)]
pub struct Grammar {
    start_symbol: NonTerminal,
    productions: Vec<Production>,

    // NOTE: the nullable non terminals could be cached in a Option<BTreeSet<NonTerminal>>
    // but we need to assume that the grammar once created is immutable, or we need to update
    // this option after each update to the grammar
    nullable: Option<BTreeSet<NonTerminal>>,
}

impl Grammar {
    pub fn new(start_symbol: NonTerminal, productions: Vec<Production>) -> Self {
        Grammar {
            start_symbol,
            productions,
            nullable: None,
        }
    }

    pub fn get_start_symbol(&self) -> NonTerminal {
        self.start_symbol
    }

    pub fn get_productions(&self) -> &Vec<Production> {
        &self.productions
    }

    pub fn productions_to_adj_list(&self) -> BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>> {
        let mut adj_list: BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>> = BTreeMap::new();
        for production in self.productions.iter() {
            adj_list
                .entry(production.start_symbol)
                .or_insert(BTreeSet::new())
                .insert(production.expand_rule.clone());
        }

        adj_list
    }

    pub fn add_fake_initial_state(&mut self) -> () {
        let new_state = self.get_non_terminal().iter().max().unwrap() + 1;
        self.productions.push(Production {
            start_symbol: new_state,
            expand_rule: vec![Letter::NonTerminal(self.start_symbol)],
        });

        self.start_symbol = new_state;
    }
}

impl<T> From<&DFA<T>> for Grammar {
    fn from(dfa: &DFA<T>) -> Self {
        // NOTE: the fact that i assume non terminal is usize, makes the grammar and DFA
        // internal representation tightly coupled, but this implementation is much simpler

        let mut productions = vec![];

        for (idx, transitions) in dfa.get_transitions().iter().enumerate() {
            for (transition_ch, dest) in transitions.iter() {
                let lhs = idx;
                let rhs = vec![Letter::Terminal(*transition_ch), Letter::NonTerminal(*dest)];
                productions.push(Production {
                    start_symbol: lhs,
                    expand_rule: rhs,
                });
            }
        }

        for end_state in dfa.get_end_states() {
            let lhs = *end_state;
            let rhs = vec![Letter::Terminal(EPSILON)];
            productions.push(Production {
                start_symbol: lhs,
                expand_rule: rhs,
            });
        }

        Self {
            start_symbol: dfa.get_start_state(),
            productions,

            nullable: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::automata::DFA;
    use crate::map;

    #[test]
    fn test_dfa_conversion() {
        // this dfa should recognize ba*
        let dfa: DFA<usize> = DFA::from_state(
            3,
            0,
            vec![1],
            vec![
                map! {
                    'a' => 2,
                    'b' => 1
                },
                map! {
                    'a' => 1,
                    'b' => 2
                },
                map! {
                    'a' => 2,
                    'b' => 2
                },
            ],
            None,
        );

        let grammar = Grammar::from(&dfa);

        // FIXME: the order in the production matters, but it shouldn't be the case.
        let result = Grammar {
            start_symbol: 0,
            productions: vec![
                Production {
                    start_symbol: 0,
                    expand_rule: vec![Letter::Terminal('a'), Letter::NonTerminal(2)],
                },
                Production {
                    start_symbol: 0,
                    expand_rule: vec![Letter::Terminal('b'), Letter::NonTerminal(1)],
                },
                Production {
                    start_symbol: 1,
                    expand_rule: vec![Letter::Terminal('a'), Letter::NonTerminal(1)],
                },
                Production {
                    start_symbol: 1,
                    expand_rule: vec![Letter::Terminal('b'), Letter::NonTerminal(2)],
                },
                Production {
                    start_symbol: 2,
                    expand_rule: vec![Letter::Terminal('a'), Letter::NonTerminal(2)],
                },
                Production {
                    start_symbol: 2,
                    expand_rule: vec![Letter::Terminal('b'), Letter::NonTerminal(2)],
                },
                Production {
                    start_symbol: 1,
                    expand_rule: vec![Letter::Terminal(EPSILON)],
                },
            ],
            nullable: None,
        };

        assert_eq!(grammar, result);
    }
}
