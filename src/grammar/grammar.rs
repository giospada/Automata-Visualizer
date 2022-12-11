use std::collections::{BTreeSet, BTreeMap};

use crate::automata::DFA;
use crate::grammar::{
    Production,
    Letter
};
use crate::grammar::consts::{
    EPSILON, 
    STRING_END,
    Terminal,
    NonTerminal,
};

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

    pub fn first(&mut self, letter: &Letter) -> BTreeSet<Terminal> {
        if let None = self.nullable {
            self.nullable = Some(self.get_nullable());
        }

        match letter {
            Letter::NonTerminal(non_terminal) => {
                let mut used = vec![false; self.get_non_terminal().len()];
                let mut first = self._first(non_terminal, &mut used);

                if self.nullable.as_ref().unwrap().contains(&non_terminal) {
                    first.insert(EPSILON);
                }

                first
            }
            Letter::Terminal(terminal) => {
                let mut first = BTreeSet::new();
                first.insert(*terminal);
                first
            }
        }
    }

    fn _first(&self, non_terminal: &NonTerminal, used: &mut Vec<bool>) -> BTreeSet<Terminal> {
        if used[*non_terminal] == true {
            return BTreeSet::new();
        }
        used[*non_terminal] = true;

        let nullable = self.nullable.as_ref().unwrap();
        let mut first = BTreeSet::new();

        for production in self.productions.iter() {
            if production.lhs != *non_terminal {
                continue;
            }
            
            for letter in production.rhs.iter() {
                // we can continue to add more only if previous symbols are nullable
                match letter {
                    Letter::NonTerminal(idx) => {
                        first.append(&mut self._first(idx, used));
                        if !nullable.contains(idx) {
                            break;
                        }
                    },
                    Letter::Terminal(ch) => {
                        // NOTE: i don't want to insert epsilons, because
                        // then i should remove them in upper level!
                        if *ch != EPSILON {
                            first.insert(*ch);
                            break;
                        }
                    }
                }
            }
        }

        first
    }

    pub fn follow(&mut self, non_terminal: &NonTerminal) -> BTreeSet<Terminal> {
        if let None = self.nullable {
            self.nullable = Some(self.get_nullable());
        }

        let num_non_terminal = self.get_non_terminal().len();
        let mut used = vec![false; num_non_terminal];

        self._follow(non_terminal, &mut used)
    }

    fn _follow(&self, non_terminal: &NonTerminal, used: &mut Vec<bool>) -> BTreeSet<Terminal> {
        if used[*non_terminal] == true {
            return BTreeSet::new();
        }
        used[*non_terminal] = true;

        let nullable = self.nullable.as_ref().unwrap();
        let mut follow = BTreeSet::new();

        if *non_terminal == self.start_symbol {
            follow.insert(STRING_END);
        }

        for production in self.productions.iter() {
            for (i, letter) in production.rhs.iter().enumerate() {
                match letter {
                    Letter::NonTerminal(idx) => {
                        if *idx != *non_terminal {
                            continue;
                        }

                        // if we are at the end of the production, then we need to add the follow of the lhs
                        if i == production.rhs.len() - 1 {
                            follow.append(&mut self._follow(&production.lhs, used));
                        } else {
                            // otherwise we need to add the first of the next symbol
                            let next_letter = &production.rhs[i + 1];
                            match next_letter {
                                Letter::NonTerminal(idx) => {
                                    let mut first_used_table = vec![false; used.len()];
                                    follow.append(&mut self._first(idx, &mut first_used_table));
                                },
                                Letter::Terminal(ch) => {
                                    follow.insert(*ch);
                                }
                            }

                            // if the whole next symbol is nullable, then we need to add the follow of the lhs
                            let mut is_nullable = true;
                            for letter in production.rhs[i + 1..].iter() {
                                match letter {
                                    Letter::NonTerminal(idx) => {
                                        if !nullable.contains(idx) {
                                            is_nullable = false;
                                            break;
                                        }
                                    },
                                    Letter::Terminal(ch) => {
                                        if *ch != EPSILON {
                                            is_nullable = false;
                                            break;
                                        }
                                    }
                                }
                            }

                            if is_nullable {
                                follow.append(&mut self._follow(&production.lhs, used));
                            }
                        }
                    },
                    Letter::Terminal(_) => {}
                }
            }
        }

        follow
    }

    pub fn productions_to_adj_list(&self) -> BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>> {
        let mut adj_list: BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>> = BTreeMap::new();
        for production in self.productions.iter() {
            adj_list.entry(production.lhs)
                .or_insert(BTreeSet::new())
                .insert(production.rhs.clone());
        }

        adj_list
    }

    pub fn add_fake_initial_state(&mut self) -> () {
        let new_state = self.get_non_terminal().iter().max().unwrap() + 1;
        self.productions.push(Production {
            lhs: new_state,
            rhs: vec![Letter::NonTerminal(self.start_symbol)]
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
                productions.push(Production { lhs: lhs, rhs });
            }
        }

        for end_state in dfa.get_end_states() {
            let lhs = *end_state;
            let rhs = vec![Letter::Terminal(EPSILON)];
            productions.push(Production { lhs: lhs, rhs });
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
    use crate::map;
    use crate::automata::DFA;

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
        let mut grammar = Grammar {
            start_symbol: 0,
            productions: vec![
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::NonTerminal(0)] },
            ],
            nullable: None,
        };

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
            None
        );

        let grammar = Grammar::from(&dfa);

        // FIXME: the order in the production matters, but it shouldn't be the case.
        let result = Grammar {
            start_symbol: 0,
            productions: vec![
                Production { lhs: 0, rhs: vec![Letter::Terminal('a'), Letter::NonTerminal(2)] },
                Production { lhs: 0, rhs: vec![Letter::Terminal('b'), Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('a'), Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('b'), Letter::NonTerminal(2)] },
                Production { lhs: 2, rhs: vec![Letter::Terminal('a'), Letter::NonTerminal(2)] },
                Production { lhs: 2, rhs: vec![Letter::Terminal('b'), Letter::NonTerminal(2)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal(EPSILON)] },
            ],
            nullable: None,
        };

        assert_eq!(grammar, result);
    }
}