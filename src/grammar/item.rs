use std::collections::{BTreeSet, BTreeMap, VecDeque};

use crate::grammar::grammar::{Grammar, Letter, Terminal, NonTerminal, Production};
use crate::grammar::consts::{EPSILON, ITEM_SEP};

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
        let mut non_terminals = Self::compute_closure_queue(items, &mut used_non_term);
        
        let dot_production = Self::add_initial_sep(grammar.transitions_to_adj_list());

        // apply the closure to all the non terminals in non_terminals
        while let Some((non_terminal, letter_first)) = non_terminals.pop_front() {
            dot_production.get(&non_terminal)
            .unwrap()
            .iter()
            .for_each(|rhs| {
                // with dot_production, the dot is always at 0, so the first letter is 1
                if rhs.len() >= 1 {
                    let letter = &rhs[1];
    
                    if let Letter::NonTerminal(non_term) = letter {
                        if !used_non_term[*non_term as usize] {
                            used_non_term[*non_term as usize] = true;

                            if rhs.len() >= 2 {
                                non_terminals.push_back((*non_term, Some(rhs[2].clone())));
                            } else {
                                non_terminals.push_back((*non_term, None));
                            }
                        }
                    }

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

    }
}