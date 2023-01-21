use std::collections::BTreeSet;

use crate::grammar::{
    consts::{NonTerminal, Terminal, EPSILON},
    Grammar, Letter, Production
};

mod first;
mod follow;

pub type FirstTable = Vec<BTreeSet<Terminal>>;
pub type FollowTable = Vec<BTreeSet<Terminal>>;

pub use first::get_first;

pub struct FirstFollowTable {
    first: FirstTable,
    follow: FollowTable,
    nullable: Vec<bool>,
}

impl FirstFollowTable {
    pub fn get_follow(&self, non_terminal: NonTerminal) -> BTreeSet<Terminal> {
        self.follow[non_terminal].clone()
    }

    pub fn get_first(&self, letter: &Letter) -> BTreeSet<Terminal> {
        first::get_first_letter(&self.first, letter)
    }

    /// checks if the rest of the iterator is all nullable.
    /// assumes the nullable set has been initialized.
    fn is_nullable<'a, T: Iterator<Item = &'a Letter>>(&self, iter: &mut T) -> bool {
        iter.all(|letter| -> bool {
            match letter {
                Letter::NonTerminal(idx) => self.nullable[*idx],
                Letter::Terminal(ch) => *ch == EPSILON,
            }
        })
    }
}

impl From<&Grammar> for FirstFollowTable {
    fn from(grammar: &Grammar) -> Self {
        let num_non_terminal = grammar.get_non_terminal().len();

        let nullable = compute_nullable(grammar, num_non_terminal);
        let first = first::compute_first(grammar, num_non_terminal, &nullable);
        let follow = follow::compute_follow(grammar, num_non_terminal, &first);

        FirstFollowTable {
            first,
            follow,
            nullable,
        }
    }
}

fn compute_nullable(grammar: &Grammar, num_non_terminal: usize) -> Vec<bool> {
    let nullable = grammar.get_nullable();
    let mut out = vec![false; num_non_terminal];

    nullable.iter().for_each(|non_terminal| -> () {
        out[*non_terminal] = true;
    });
    out
}

#[cfg(test)]
mod test {
    use crate::grammar::consts::STRING_END;

    use super::*;

    fn get_test_grammar() -> Grammar {
        // S -> Ab | c
        // A -> aA | ε
        // S = 0
        // A = 1
        Grammar::new(
            0,
            vec![
                Production {
                    start_symbol: 0,
                    expand_rule: vec![Letter::NonTerminal(1), Letter::Terminal('b')],
                },
                Production {
                    start_symbol: 0,
                    expand_rule: vec![Letter::Terminal('c')],
                },
                Production {
                    start_symbol: 1,
                    expand_rule: vec![Letter::Terminal('a'), Letter::NonTerminal(1)],
                },
                Production {
                    start_symbol: 1,
                    expand_rule: vec![Letter::Terminal(EPSILON)],
                },
            ],
        )
    }

    #[test]
    fn first_test() {
        let grammar = get_test_grammar();

        let first_follow = FirstFollowTable::from(&grammar);
        let first = first_follow.get_first(&Letter::NonTerminal(0));

        assert_eq!(first.len(), 3);
        assert!(first.contains(&'a'));
        assert!(first.contains(&'b'));
        assert!(first.contains(&'c'));

        let first = first_follow.get_first(&Letter::NonTerminal(1));
        assert_eq!(first.len(), 2);
        assert!(first.contains(&'a'));
        assert!(first.contains(&EPSILON));
    }

    #[test]
    fn first_cycle_test() {
        let grammar = Grammar::new(
            0,
            vec![
                Production {
                    start_symbol: 0,
                    expand_rule: vec![Letter::NonTerminal(1)],
                },
                Production {
                    start_symbol: 1,
                    expand_rule: vec![Letter::NonTerminal(0)],
                },
            ],
        );

        let first_follow = FirstFollowTable::from(&grammar);
        let first = first_follow.get_first(&Letter::NonTerminal(0));

        assert_eq!(first.len(), 0);
    }

    #[test]
    fn follow_test() {
        let grammar = get_test_grammar();
        let first_follow = FirstFollowTable::from(&grammar);

        let follow = first_follow.get_follow(0);
        assert_eq!(follow.len(), 1);
        assert!(follow.contains(&STRING_END));

        let follow = first_follow.get_follow(1);
        assert_eq!(follow.len(), 1);
        assert!(follow.contains(&'b'));
    }
}
