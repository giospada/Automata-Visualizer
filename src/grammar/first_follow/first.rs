use std::collections::BTreeSet;

use crate::grammar::{
    consts::{Terminal, EPSILON},
    Grammar, Letter,
};

use super::FirstTable;

pub fn compute_first(
    grammar: &Grammar,
    num_non_terminal: usize,
    nullable: &Vec<bool>,
) -> FirstTable {
    let mut first_table = vec![BTreeSet::new(); num_non_terminal];
    let productions = grammar.get_productions();
    let mut has_changed = true;

    while has_changed {
        has_changed = false;

        productions.iter().for_each(|production| -> () {
            has_changed |= production.update_first_table(&mut first_table, &nullable);
        });
    }

    (0..num_non_terminal).for_each(|i| -> () {
        if nullable[i] {
            first_table[i].insert(EPSILON);
        }
    });
    first_table
}

pub fn get_first_letter(first: &FirstTable, letter: &Letter) -> BTreeSet<Terminal> {
    match letter {
        Letter::NonTerminal(idx) => first[*idx].clone(),
        Letter::Terminal(ch) => BTreeSet::from([*ch]),
    }
}

//TODO: add tests
pub fn get_first<'a, T>(first: &FirstTable, iter: &mut T) -> BTreeSet<Terminal>
where
    T: Iterator<Item = &'a Letter>,
{
    let mut out = BTreeSet::new();
    let mut nullable = true;
    for letter in iter {
        let mut set = get_first_letter(first, letter);
        if !set.remove(&EPSILON) {
            nullable = false;
        }
        out.append(&mut set);
    }
    if nullable {
        out.insert(EPSILON);
    }
    out
}
