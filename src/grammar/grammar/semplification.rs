use std::collections::{BTreeSet};

use super::{Grammar, Letter, Production};

impl Grammar {
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
        let mut adj_list = self.productions_to_adj_list();
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}