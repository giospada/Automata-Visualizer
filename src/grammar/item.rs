use std::collections::{BTreeSet, BTreeMap, VecDeque};

use crate::grammar::{Grammar, Letter, Production};
use crate::grammar::consts::{EPSILON, ITEM_SEP, Terminal, NonTerminal};

use super::consts::STRING_END;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    production: Production,
    look_ahead: Option<Terminal>,
}

impl Item {
    pub fn get_itemization(productions: &Vec<Production>) -> Vec<Production> {
        let mut itemized_transitions = vec![];
        for production in productions.iter() {
            itemized_transitions.append(&mut Item::itemize(production));
        }

        itemized_transitions
    }

    /// itemizes a single production
    pub fn itemize(production: &Production) -> Vec<Production> {
        let mut itemized_prod = vec![];

        if production.rhs.len() == 1 && production.rhs[0] == Letter::Terminal(EPSILON) {
            itemized_prod.push(Production {
                lhs: production.lhs,
                rhs: vec![Letter::Terminal(ITEM_SEP)]
            });
            return itemized_prod;
        }

        for i in 0..=production.rhs.len() {
            let mut rhs = production.rhs.clone();

            rhs.insert(i, Letter::Terminal(ITEM_SEP));
            itemized_prod.push(Production {
                lhs: production.lhs,
                rhs: rhs
            });
        }

        itemized_prod
    }

    /// adds a ITEM_SEP to the beginning of each production
    fn add_initial_sep(productions: BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>>) 
        -> BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>> {
            let mut result_prod = BTreeMap::new();
            for (non_terminal, set) in productions.into_iter() {
                let mut letters = set.into_iter().collect::<Vec<Vec<Letter>>>();
                letters.iter_mut().for_each(|letter| {
                    letter.insert(0, Letter::Terminal(ITEM_SEP));
                });

                result_prod.insert(non_terminal, letters.into_iter().collect::<BTreeSet<Vec<Letter>>>());
            }

            result_prod
        }

    /// return the closure of the set **productions** in the input
    /// with the given look_ahead
    /// production is the adjiacency list of all the productions
    pub fn closure(
        items: &BTreeSet<Item>,
        grammar: &mut Grammar,
    ) -> BTreeSet<Item> {
        let mut closure_items = (*items).clone();
        let mut used_non_term = vec![false; grammar.get_non_terminal().len()];
        let dot_production = Self::add_initial_sep(grammar.productions_to_adj_list());

        // the non terminals to explore are seen as a BFS that expands to other non terminals
        // when it sees an arc (e.g. a non terminal after a SEP).
        let mut closure_queue = Self::compute_closure_queue(items, &mut used_non_term);

        // apply the closure to all the non terminals in closure_queue
        while let Some((non_terminal, letter_first)) = closure_queue.pop_front() {
            dot_production.get(&non_terminal)
            .unwrap()
            .iter()
            .for_each(|rhs| {
                // with dot_production, the dot is always at 0, so the first letter is 1
                let (non_term_opt, look_ahead) = Self::get_next_closure_non_term(rhs);

                match non_term_opt {
                    Some(non_term) => {
                        if !used_non_term[non_term] {
                            used_non_term[non_term] = true;

                            match look_ahead {
                                Some(_) => {
                                    if let None = letter_first {
                                        closure_queue.push_back((non_term, None));
                                    } else {
                                        closure_queue.push_back((non_term, look_ahead))
                                    }
                                },
                                None => closure_queue.push_back((non_term, None)),
                            }
                        }
                    }
                    None => {}
                }

                let production = Production {
                    lhs: non_terminal,
                    rhs: rhs.clone(),
                };
                
                // Closure with first only if the precedente look_ahead has been set!
                // this should save some computation time.
                if let None = letter_first {
                    closure_items.insert(Item {
                        production: production,
                        look_ahead: None,
                    });
                    return;
                }

                let first = letter_first.as_ref().unwrap();
                let first_letter_set = grammar.first(first);
                
                first_letter_set.iter().for_each(|look_ahead| {
                    closure_items.insert(Item {
                        production: production.clone(),
                        look_ahead: Some(*look_ahead),
                    });
                });
            });
        }

        closure_items
    }

    /// queue of non_terminal to explore and next letter for first
    fn compute_closure_queue(items: &BTreeSet<Item>, used_non_term: &mut Vec<bool>) 
        -> VecDeque<(NonTerminal, Option<Letter>)> {
            let mut non_terminals: VecDeque<(NonTerminal, Option<Letter>)> = VecDeque::new();
            items.iter().for_each(|item| {
                let item_sep_pos = item.production.rhs.iter()
                    .position(|letter| *letter == Letter::Terminal(ITEM_SEP));
                if item_sep_pos.is_none() {
                    return;
                }

                let item_sep_pos = item_sep_pos.unwrap();
                if item_sep_pos == item.production.rhs.len() - 1 {
                    return;
                }

                let next_letter = &item.production.rhs[item_sep_pos + 1];

                if let Letter::NonTerminal(non_terminal) = next_letter {
                    if used_non_term[*non_terminal] {
                        return;
                    }
                    used_non_term[*non_terminal] = true;

                    if let None = item.look_ahead {
                        non_terminals.push_back((*non_terminal, None));
                    } else if item_sep_pos < item.production.rhs.len() - 2 {
                        non_terminals.push_back((*non_terminal, Some(item.production.rhs[item_sep_pos + 2].clone())));
                    } else {
                        non_terminals.push_back((*non_terminal, Some(Letter::Terminal(STRING_END))));
                    }
                }
            });

            non_terminals
    }

    /// this function assumes the dot is at the beginning of the production
    /// and returns the non terminal after the dot, with the look ahead letter,
    /// if there is one.
    fn get_next_closure_non_term(rhs: &Vec<Letter>) -> (Option<NonTerminal>, Option<Letter>) {
        let first_non_term = Production::get_nth_if_non_terminal(rhs, 1);
        let second_letter = Production::get_nth(rhs, 2);

        match first_non_term {
            Some(non_term) => {
                match second_letter {
                    Some(letter) => (Some(*non_term), Some(letter.clone())),
                    None => (Some(*non_term), None),
                }
            }
            None => (None, None),
        }
    }

    pub fn goto(items: &BTreeSet<Item>, letter: &Letter) -> BTreeSet<Item> {
        let mut goto_items: BTreeSet<Item> = BTreeSet::new();

        items.iter().for_each(|item| {
            let item_sep_pos = item.production.rhs.iter().position(|letter| *letter == Letter::Terminal(ITEM_SEP));
            if item_sep_pos.is_none() {
                return;
            }

            let item_sep_pos = item_sep_pos.unwrap();
            if item_sep_pos == item.production.rhs.len() - 1 {
                return;
            }

            if &item.production.rhs[item_sep_pos + 1] != letter {
                return;
            }

            goto_items.insert({
                let mut new_item = item.clone();
                new_item.production.rhs[item_sep_pos] = new_item.production.rhs[item_sep_pos + 1].clone();
                new_item.production.rhs[item_sep_pos + 1] = Letter::Terminal(ITEM_SEP);

                new_item
            });
        });

        goto_items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set;
    #[test]
    fn test_itemization() {
        let productions = vec![
            Production { lhs: 0, rhs: vec![Letter::NonTerminal(1), Letter::Terminal('b')] },
            Production { lhs: 0, rhs: vec![Letter::Terminal('c')] },
            Production { lhs: 1, rhs: vec![Letter::Terminal('a'), Letter::NonTerminal(1)] },
            Production { lhs: 1, rhs: vec![Letter::Terminal(EPSILON)] },
        ];

        let items = Item::get_itemization(&productions);

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

    #[test]
    fn closure_0 () {
        // S -> (S)
        // S -> A
        // A -> a
        let mut grammar = Grammar::new(
            0,
            vec![
                Production { lhs: 0, rhs: vec![Letter::Terminal('('), Letter::NonTerminal(0), Letter::Terminal(')')] },
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('a')] },
            ],
        );
        grammar.add_fake_initial_state();

        let mut start_item = set![Item {
            production: Production { lhs: 2, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::NonTerminal(0)] },
            look_ahead: None,
        }];
        let closure = Item::closure(&mut start_item, &mut grammar).into_iter()
            .map(|item| item.production)
            .collect::<Vec<_>>();

        let result = vec![
            Production { lhs: 2, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::NonTerminal(0)] },
            Production { lhs: 0, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('('), Letter::NonTerminal(0), Letter::Terminal(')')] },
            Production { lhs: 0, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::NonTerminal(1)] },
            Production { lhs: 1, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('a')] },
        ];

        assert!(closure.iter().all(|item| result.contains(item)));
        assert!(result.iter().all(|item| closure.contains(item)));
    }

    #[test]
    fn closure_1() {
        // S -> CC
        // C -> cC
        // C -> d

        let mut grammar = Grammar::new(
            0,
            vec![
                Production { lhs: 0, rhs: vec![Letter::NonTerminal(1), Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('c'), Letter::NonTerminal(1)] },
                Production { lhs: 1, rhs: vec![Letter::Terminal('d')] },
            ],
        );

        grammar.add_fake_initial_state();

        let mut start_item = set![Item {
            production: Production { lhs: 2, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::NonTerminal(0)] },
            look_ahead: Some(STRING_END),
        }];

        let closure: Vec<Item> = Item::closure(&mut start_item, &mut grammar).into_iter().collect();

        let result = vec![
            Item {
                production: Production { lhs: 2, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::NonTerminal(0)] },
                look_ahead: Some(STRING_END),
            },
            Item {
                production: Production { lhs: 0, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::NonTerminal(1), Letter::NonTerminal(1)] },
                look_ahead: Some(STRING_END),
            },
            Item {
                production: Production { lhs: 1, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('c'), Letter::NonTerminal(1)] },
                look_ahead: Some('c'),
            },
            Item {
                production: Production { lhs: 1, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('c'), Letter::NonTerminal(1)] },
                look_ahead: Some('d'),
            },
            Item {
                production: Production { lhs: 1, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('d')] },
                look_ahead: Some('c'),
            },
            Item {
                production: Production { lhs: 1, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('d')] },
                look_ahead: Some('d'),
            },
        ];

        assert!(closure.iter().all(|item| result.contains(item)));
        assert!(result.iter().all(|item| closure.contains(item)));
    }

    #[test]
    fn goto() {
        // S -> (S)
        // S -> ()
        let mut grammar = Grammar::new(
            0,
            vec![
                Production { lhs: 0, rhs: vec![Letter::Terminal('('), Letter::NonTerminal(0), Letter::Terminal(')')] },
                Production { lhs: 0, rhs: vec![Letter::Terminal('('), Letter::Terminal(')')] },
            ],
        );
        grammar.add_fake_initial_state();

        let mut start_item = set![Item {
            production: Production { lhs: 1, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::NonTerminal(0)] },
            look_ahead: None,
        }];

        let closure = Item::closure(&mut start_item, &mut grammar);
        let goto = Item::goto(&closure, &Letter::Terminal('(')).into_iter()
            .map(|item| item.production)
            .collect::<Vec<_>>();

        // result should be
        // S -> (.S)
        // S -> (.)
        let result = vec![
            Production { lhs: 0, rhs: vec![Letter::Terminal('('), Letter::Terminal(ITEM_SEP), Letter::NonTerminal(0), Letter::Terminal(')')] },
            Production { lhs: 0, rhs: vec![Letter::Terminal('('), Letter::Terminal(ITEM_SEP), Letter::Terminal(')')] },
        ];

        assert!(goto.iter().all(|item| result.contains(item)));
        assert!(result.iter().all(|item| goto.contains(item)));

        // SECOND PART OF TEST, APPLY CLOSURE TO GOTO'S OUTPUT
        let mut goto_items = goto.into_iter()
            .map(|item| Item {
                production: item,
                look_ahead: None,
            })
            .collect::<BTreeSet<_>>();

        let closure = Item::closure(&mut goto_items, &mut grammar)
            .into_iter()
            .map(|item| item.production)
            .collect::<Vec<_>>();

        let result = vec![
            Production { lhs: 0, rhs: vec![Letter::Terminal('('), Letter::Terminal(ITEM_SEP), Letter::NonTerminal(0), Letter::Terminal(')')] },
            Production { lhs: 0, rhs: vec![Letter::Terminal('('), Letter::Terminal(ITEM_SEP), Letter::Terminal(')')] },
            Production { lhs: 0, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('('), Letter::NonTerminal(0), Letter::Terminal(')')] },
            Production { lhs: 0, rhs: vec![Letter::Terminal(ITEM_SEP), Letter::Terminal('('), Letter::Terminal(')')] },
        ];

        assert!(closure.iter().all(|item| result.contains(item)));
        assert!(result.iter().all(|item| closure.contains(item)));

    }
}