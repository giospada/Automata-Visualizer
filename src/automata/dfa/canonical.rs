use std::collections::{BTreeMap, BTreeSet};

use crate::grammar::grammar::{Production, Grammar};
use crate::automata::dfa::DFA;

pub type GrammarStates = BTreeSet<Production>;

impl From<&mut Grammar> for DFA<GrammarStates> {
    fn from(grammar: &mut Grammar) -> Self {
        let mut dfa = DFA::new();

        let mut state_to_index: BTreeMap<Vec<Production>, usize> = BTreeMap::new();

        dfa
    }
}