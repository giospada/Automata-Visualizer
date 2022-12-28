use std::collections::BTreeSet;

use crate::grammar::{consts::STRING_END, Grammar};

use super::{FirstTable, FollowTable};

pub fn compute_follow(
    grammar: &Grammar,
    num_non_terminal: usize,
    first: &FirstTable,
) -> FollowTable {
    let mut follow_table = vec![BTreeSet::new(); num_non_terminal];
    follow_table[grammar.get_start_symbol()].insert(STRING_END);

    let productions = grammar.get_productions();
    let mut has_changed = true;

    while has_changed {
        has_changed = false;

        productions.iter().for_each(|production| {
            has_changed |= production.update_follow_table(first, &mut follow_table);
        });
    }
    follow_table
}
