use std::collections::{BTreeSet, BTreeMap};

use crate::automata::DFA;

pub type NonTerminal = usize;
pub type Terminal = char;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub enum Letter {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

#[derive(Debug, PartialEq)]
pub struct Production {
    lhs: NonTerminal,
    rhs: Vec<Letter>,
}

#[derive(Debug, PartialEq)]
pub struct Grammar {
    start_symbol: NonTerminal,
    productions: Vec<Production>,

    // NOTE: the nullable non terminals could be cached in a Option<BTreeSet<NonTerminal>>
    // but we need to assume that the grammar once created is immutable, or we need to update
    // this option after each update to the grammar
    nullable: Option<BTreeSet<NonTerminal>>,
}

const EPSILON: char = 'ε';
const STRING_END: char = '$';

// NOTE: this could be in conflict with the Terminal symbols, so
// it is MANDATORY that the Terminal doesn´t have dots in it!
const ITEM_SEP: char = '.';

impl Grammar {
    pub fn first(&mut self, letter: &Letter) -> BTreeSet<Terminal> {
        if let None = self.nullable {
            self.nullable = Some(self.get_nullable());
        }

        match letter {
            Letter::NonTerminal(non_terminal) => {
                let mut first = self._first(non_terminal);

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

    fn _first(&self, non_terminal: &NonTerminal) -> BTreeSet<Terminal> {
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
                        first.append(&mut self._first(idx));
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
        let mut used = vec![0; num_non_terminal];

        self._follow(non_terminal, &mut used)
    }

    fn _follow(&self, non_terminal: &NonTerminal, used: &mut Vec<usize>) -> BTreeSet<Terminal> {
        if used[*non_terminal] == 1 {
            return BTreeSet::new();
        }
        used[*non_terminal] = 1;

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
                                    follow.append(&mut self._first(idx));
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
            for production in self.productions.iter() {
                if !reachable.contains(&production.lhs) {
                    continue;
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
            }
        }

        reachable
    }

    /// returns set of generator non terminals
    /// a non terminal is a generator when it produces some finite
    /// string of terminals
    pub fn get_generators(&self) -> BTreeSet<usize> {
        let mut generators = BTreeSet::new();
        for production in self.productions.iter() {
            let mut is_generator = true;
            for letter in production.rhs.iter() {
                match letter {
                    Letter::NonTerminal(non_terminal) => {
                        if !generators.contains(non_terminal) {
                            is_generator = false;
                            break;
                        }
                    }
                    Letter::Terminal(_) => {}
                }
            }
            if is_generator {
                generators.insert(production.lhs);
            }
        }

        generators
    }

    pub fn remove_useless(&mut self) -> () {
        // first remove non generators
        let generators = self.get_generators();

        self.productions.retain(|production| {
            generators.contains(&production.lhs) && production.rhs.iter().all(|letter| {
                match letter {
                    Letter::NonTerminal(idx) => generators.contains(idx),
                    Letter::Terminal(_) => true
                }
            })
        });

        // then remove non reachable
        let reachable = self.get_reachable();

        self.productions.retain(|production| {
            reachable.contains(&production.lhs) && production.rhs.iter().all(|letter| {
                match letter {
                    Letter::NonTerminal(idx) => reachable.contains(idx),
                    Letter::Terminal(_) => true
                }
            })
        });

        // invalidate nullable
        self.nullable = None;
    }

    pub fn get_unitary_couples(&self) -> BTreeSet<(NonTerminal, NonTerminal)>  {
        let non_terminals = self.get_non_terminal();
        let mut unitary_couples = BTreeSet::new();
        let mut has_changed = true;

        for non_terminal in non_terminals {
            unitary_couples.insert((non_terminal, non_terminal));
        }
        
        while has_changed {
            has_changed = false;
            for production in self.productions.iter() {
                if production.rhs.len() != 1 {
                    continue;
                }
                let mut to_insert = BTreeSet::new();
                for unitary_couple in unitary_couples.iter() {
                    if let Letter::NonTerminal(non_term) = production.rhs[0] {
                        if unitary_couple.1 == production.lhs && 
                         !unitary_couples.contains(&(unitary_couple.0, non_term)) &&
                         !to_insert.contains(&(unitary_couple.0, non_term)) {
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

    // TODO: this is a very complex function in this moment, it needs refactor
    // it also has some points were it can be optimized 
    pub fn remove_unitary_cycles(&mut self) {
        let unitary_couples = self.get_unitary_couples();

        // remove all unitary couples
        self.productions.retain(|production| {
            if production.rhs.len() != 1 {
                return true;
            }

            match production.rhs[0] {
                Letter::NonTerminal(non_term) => !unitary_couples.contains(&(production.lhs, non_term)),
                Letter::Terminal(_) => true
            }
        });

        // add corresponding productions 
        let mut adj_list = self.transitions_to_adj_list();
        for unitary_couple in unitary_couples.iter() {
            if unitary_couple.0 == unitary_couple.1 {
                continue;
            }

            let mut to_insert = adj_list.get(&unitary_couple.1).unwrap().clone();

            adj_list.entry(unitary_couple.0)
                .or_insert(BTreeSet::new())
                .append(&mut to_insert);
        }

        // trasform adj list back to transitions
        let mut new_transitions = vec![];
        for (non_terminal, transitions) in adj_list.iter() {
            for transition in transitions.iter() {
                new_transitions.push(Production {
                    lhs: *non_terminal,
                    rhs: transition.clone()
                });
            }
        }
        self.productions = new_transitions;

        // invalidate nullable
        self.nullable = None;
    }

    fn transitions_to_adj_list(&self) -> BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>> {
        let mut adj_list: BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>> = BTreeMap::new();
        for production in self.productions.iter() {
            adj_list.entry(production.lhs)
                .or_insert(BTreeSet::new())
                .insert(production.rhs.clone());
        }

        adj_list
    }

    pub fn get_itemization(&self) -> Vec<Production> {
        let mut itemized_transitions = vec![];
        for production in self.productions.iter() {
            if production.rhs.len() == 1 && production.rhs[0] == Letter::Terminal(EPSILON) {
                itemized_transitions.push(Production {
                    lhs: production.lhs,
                    rhs: vec![Letter::Terminal(ITEM_SEP)]
                });
                continue;
            }

            for i in 0..=production.rhs.len() {
                let mut rhs = production.rhs.clone();

                rhs.insert(i, Letter::Terminal(ITEM_SEP));
                itemized_transitions.push(Production {
                    lhs: production.lhs,
                    rhs: rhs
                });
            }
        }

        itemized_transitions
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
    fn test_nullable() {
        let grammar = get_test_grammar();

        let nullable = grammar.get_nullable();
        assert_eq!(nullable.len(), 1);
        assert!(nullable.contains(&1));
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

    #[test]
    fn test_remove_useless() {
        let mut grammar = {
            // S -> AB | a
            // B -> b

            // S = 0
            // B = 1
            // A = 2

            Grammar {
                start_symbol: 0,
                productions: vec![
                    Production { lhs: 0, rhs: vec![Letter::NonTerminal(1), Letter::NonTerminal(2)] },
                    Production { lhs: 0, rhs: vec![Letter::Terminal('a')] },
                    Production { lhs: 1, rhs: vec![Letter::Terminal('b')] },
                ],
                nullable: None,
            }
        };

        grammar.remove_useless();

        let result = Grammar {
            start_symbol: 0,
            productions: vec![
                Production { lhs: 0, rhs: vec![Letter::Terminal('a')] },
            ],
            nullable: None,
        };

        assert_eq!(grammar, result);
    }

    #[test]
    fn test_remove_unitary_cycles() {
        // E -> E + T | T
        // T -> T * F | F
        // F -> (E) | a 
        
        let mut grammar = Grammar {
            start_symbol: 0,
            productions: vec![
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(0), Letter::Terminal('+'), Letter::NonTerminal(1)] },
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::NonTerminal(1), Letter::Terminal('*'), Letter::NonTerminal(2)] },
                Production { lhs: 1, rhs: vec![Letter::NonTerminal(2)] },
                Production { lhs: 2, rhs: vec![Letter::Terminal('('), Letter::NonTerminal(0), Letter::Terminal(')')] },
                Production { lhs: 2, rhs: vec![Letter::Terminal('a')] },
            ],
            nullable: None,
        };

        let result = Grammar {
            // E -> E + T | T * F | (E) | a
            // T -> T * F | (E) | a
            // F -> (E) | a
            start_symbol: 0,
            productions: vec![
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(0), Letter::Terminal('+'), Letter::NonTerminal(1)] },
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(1), Letter::Terminal('*'), Letter::NonTerminal(2)] },
                Production { lhs: 0, rhs: vec![Letter::Terminal('('), Letter::NonTerminal(0), Letter::Terminal(')')] },
                Production { lhs: 0, rhs: vec![Letter::Terminal('a')] },
                Production { lhs: 1, rhs: vec![Letter::NonTerminal(1), Letter::Terminal('*'), Letter::NonTerminal(2)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('('), Letter::NonTerminal(0), Letter::Terminal(')')] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('a')] },
                Production { lhs: 2, rhs: vec![Letter::Terminal('('), Letter::NonTerminal(0), Letter::Terminal(')')] },
                Production { lhs: 2, rhs: vec![Letter::Terminal('a')] },
            ],
            nullable: None,
        };

        grammar.remove_unitary_cycles();

        assert_eq!(grammar, result);
    }

    #[test]
    fn test_itemization() {
        let grammar = get_test_grammar();

        let items = grammar.get_itemization();

        let result_productions = vec![
            Production { lhs: 0, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::NonTerminal(1), Letter::Terminal('b')] },
            Production { lhs: 0, rhs: vec![Letter::NonTerminal(1), Letter::Terminal(ITEM_SEP), Letter::Terminal('b')] },
            Production { lhs: 0, rhs: vec![Letter::NonTerminal(1), Letter::Terminal('b'), Letter::Terminal(ITEM_SEP)] },

            Production { lhs: 0, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('c')] },
            Production { lhs: 0, rhs: vec![Letter::Terminal('c'), Letter::Terminal(ITEM_SEP)] },
            
            Production { lhs: 1, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('a'), Letter::NonTerminal(1)] },
            Production { lhs: 1, rhs: vec![Letter::Terminal('a'), Letter::Terminal(ITEM_SEP), Letter::NonTerminal(1)] },
            Production { lhs: 1, rhs: vec![Letter::Terminal('a'), Letter::NonTerminal(1), Letter::Terminal(ITEM_SEP)] },
            Production { lhs: 1, rhs: vec![Letter::Terminal(ITEM_SEP)] },
        ];

        assert!(items.iter().all(|item| result_productions.contains(item)));
        assert!(result_productions.iter().all(|item| items.contains(item)));
    }
}