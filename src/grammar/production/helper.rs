use std::collections::{BTreeSet, VecDeque};

use crate::grammar::{
    consts::{NonTerminal, EPSILON},
    get_first, FollowTable,
};

use super::{Letter, Production};
use crate::grammar::first_follow::FirstTable;

impl Production {
    pub fn check_is_nullable(&self, nullable: &BTreeSet<NonTerminal>) -> bool {
        for letter in self.expand_rule.iter() {
            match letter {
                Letter::NonTerminal(idx) => {
                    if !nullable.contains(idx) {
                        return false;
                    }
                }
                Letter::Terminal(ch) => {
                    if *ch != EPSILON {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn update_first_table(&self, first_table: &mut FirstTable, nullable: &Vec<bool>) -> bool {
        let mut has_changed = false;
        for letter in self.expand_rule.iter() {
            match letter {
                Letter::NonTerminal(idx) => {
                    let mut set_to_join = first_table[*idx].clone();

                    if !set_to_join.is_subset(&first_table[self.start_symbol]) {
                        first_table[self.start_symbol].append(&mut set_to_join);
                        has_changed = true;
                    }
                    if !nullable[*idx] {
                        break;
                    }
                }
                Letter::Terminal(ch) => {
                    if *ch != EPSILON {
                        has_changed |= first_table[self.start_symbol].insert(*ch);
                        break;
                    }
                }
            }
        }
        has_changed
    }

    /// updates the follow table with the given production
    /// returns true if the follow table has changed, false otherwise
    pub fn update_follow_table(&self, first: &FirstTable, follow: &mut FollowTable) -> bool {
        let mut has_changed = false;
        let mut production = self
            .expand_rule
            .clone()
            .into_iter()
            .collect::<VecDeque<Letter>>();

        for letter in self.expand_rule.iter() {
            production.pop_front();
            if let Letter::NonTerminal(idx) = letter {
                let mut res = get_first(first, &mut production.iter());
                if res.remove(&EPSILON) {
                    res.append(&mut follow[self.start_symbol].clone());
                }
                if !res.is_subset(&follow[*idx]) {
                    follow[*idx].append(&mut res);
                    has_changed = true;
                }
            }
        }
        has_changed
    }
}
