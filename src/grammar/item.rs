use std::collections::{BTreeSet, BTreeMap};

use crate::grammar::grammar::{Letter, NonTerminal, Production};
use crate::grammar::consts::{EPSILON, STRING_END, ITEM_SEP};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    production: Production,
    look_ahead: String,

    /// number of the lookahead for the item
    num: usize,
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
    fn closure(
        items: &BTreeSet<Item>,
        productions: BTreeMap<NonTerminal, BTreeSet<Vec<Letter>>>, 
        look_ahead: usize) -> BTreeSet<Item> 
    {
        let mut closure_items = (*items).clone();
        let dot_production = Self::add_initial_sep(productions);

        items.iter().for_each(|item| {
            let item_sep_pos = item.production.rhs.iter().position(|letter| *letter == Letter::Terminal(ITEM_SEP));
            if item_sep_pos.is_none() {
                return;
            }

            let item_sep_pos = item_sep_pos.unwrap();
            if item_sep_pos == item.production.rhs.len() - 1 {
                return;
            }

            let next_letter = &item.production.rhs[item_sep_pos + 1];
            if let Letter::NonTerminal(non_terminal) = next_letter {
                let mut new_items = dot_production.get(non_terminal)
                    .unwrap()
                    .iter()
                    .map(|rhs| {
                    Item {
                        production: Production {
                            lhs: *non_terminal,
                            rhs: rhs.clone(),
                        },
                        look_ahead: item.look_ahead.clone(),
                        num: look_ahead,
                    }
                }).collect::<BTreeSet<Item>>();

                closure_items.append(&mut new_items);
            }
        });

        closure_items
    }

    fn goto(items: &BTreeSet<Item>) -> BTreeSet<Item> {
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

            let next_letter = &item.production.rhs[item_sep_pos + 1];
            if *next_letter == Letter::Terminal(STRING_END) {
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

#[cfg(tests)]
mod tests {
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
}