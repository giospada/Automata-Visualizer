use log::info;
use std::collections::{BTreeMap, BTreeSet};

use crate::automata::regular_expression as RE;
use crate::display::DisplayGraph;
use crate::utils::Graph;

mod from_string;

#[derive(Debug)]
pub struct NFA {
    start_state: usize,
    num_states: usize,
    end_states: Vec<usize>,
    transitions: Vec<BTreeMap<char, Vec<usize>>>,

    // subset of the alphabet that is used in the regular expression
    // used to reduce the number of transitions in the DFA conversion
    used_alphabet: BTreeSet<char>,
}

impl NFA {
    fn new() -> Self {
        Self {
            num_states: 0,
            start_state: 0,
            end_states: Vec::new(),
            transitions: Vec::new(),
            used_alphabet: BTreeSet::new(),
        }
    }

    pub fn get_start_state(&self) -> usize {
        self.start_state
    }

    pub fn get_alphabet(&self) -> Vec<char> {
        self.used_alphabet.iter().cloned().collect()
    }

    pub fn is_final_state(&self, state: usize) -> bool {
        self.end_states.contains(&state)
    }

    pub fn contains_final_state(&self, states: &BTreeSet<usize>) -> bool {
        for state in states {
            if self.is_final_state(*state) {
                return true;
            }
        }

        false
    }

    pub fn epsilon_closure(&self, states: &Vec<usize>) -> BTreeSet<usize> {
        let mut closure = BTreeSet::new();
        let mut visited = vec![false; self.num_states];
        let mut states_to_visit = vec![];

        for state in states {
            closure.insert(*state);
            states_to_visit.push(*state);
            visited[*state] = true;
        }

        while !states_to_visit.is_empty() {
            let mut new_states_to_visit = vec![];

            for state in states_to_visit {
                let epsilon_states = self.get_epsilon_transitions(state);

                for epsilon_state in epsilon_states {
                    if !visited[epsilon_state] {
                        visited[epsilon_state] = true;
                        new_states_to_visit.push(epsilon_state);
                        closure.insert(epsilon_state);
                    }
                }
            }

            states_to_visit = new_states_to_visit;
        }

        closure
    }

    pub fn make_move(&self, states: &BTreeSet<usize>, c: char) -> BTreeSet<usize> {
        let mut new_states = BTreeSet::new();

        for state in states {
            if self.transitions[*state].contains_key(&c) {
                for next_state in &self.transitions[*state][&c] {
                    new_states.insert(*next_state);
                }
            }
        }

        new_states
    }

    fn get_epsilon_transitions(&self, state: usize) -> Vec<usize> {
        let mut transitions = vec![];
        for i in self.transitions[state].keys() {
            if *i == 'ε' {
                for j in &self.transitions[state][i] {
                    transitions.push(*j);
                }
            }
        }

        transitions
    }

    fn recursive_from_regex(
        &mut self,
        regex: &RE::ReOperator,
        first_option: Option<usize>,
    ) -> (usize, usize) {
        let add_state = |nfa: &mut NFA| {
            nfa.num_states += 1;
            nfa.transitions.push(BTreeMap::new());
            nfa.num_states - 1
        };

        let add_start_end = |nfa: &mut NFA| {
            (
                if let Some(start) = first_option {
                    start
                } else {
                    add_state(nfa)
                },
                add_state(nfa),
            )
        };

        let (start, end) = match regex {
            RE::ReOperator::Concat(left, right) => {
                let (l_start, l_end) = self.recursive_from_regex(left, first_option);
                let (_r_start, r_end) = self.recursive_from_regex(right, Some(l_end));

                (l_start, r_end)
            }
            RE::ReOperator::Or(left, right) => {
                let (start, end) = add_start_end(self);

                let (l_start, l_end) = self.recursive_from_regex(left, None);
                let (r_start, r_end) = self.recursive_from_regex(right, None);

                self.transitions[start]
                    .entry('ε')
                    .or_insert(Vec::new())
                    .push(l_start);
                self.transitions[start]
                    .entry('ε')
                    .or_insert(Vec::new())
                    .push(r_start);
                self.transitions[r_end]
                    .entry('ε')
                    .or_insert(Vec::new())
                    .push(end);
                self.transitions[l_end]
                    .entry('ε')
                    .or_insert(Vec::new())
                    .push(end);

                (start, end)
            }
            RE::ReOperator::KleeneStar(inner) => {
                let (start, end) = add_start_end(self);
                let (i_start, i_end) = self.recursive_from_regex(inner, None);

                self.transitions[start]
                    .entry('ε')
                    .or_insert(Vec::new())
                    .push(end);
                self.transitions[i_end]
                    .entry('ε')
                    .or_insert(Vec::new())
                    .push(i_start);
                self.transitions[start]
                    .entry('ε')
                    .or_insert(Vec::new())
                    .push(i_start);
                self.transitions[i_end]
                    .entry('ε')
                    .or_insert(Vec::new())
                    .push(end);

                (start, end)
            }
            RE::ReOperator::Char(c) => {
                let (start, end) = add_start_end(self);
                self.transitions[start]
                    .entry(*c)
                    .or_insert(Vec::new())
                    .push(end);

                self.used_alphabet.insert(*c);

                (start, end)
            }
        };

        (start, end)
    }
}

impl From<&RE::ReOperator> for NFA {
    fn from(regex: &RE::ReOperator) -> Self {
        let mut nfa = Self::new();
        let (start, end) = nfa.recursive_from_regex(regex, None);
        nfa.start_state = start;
        nfa.end_states.push(end);
        nfa
    }
}

impl Into<Graph> for NFA {
    fn into(self) -> Graph {
        let mut graph = Graph::new();

        let finals_nodes = self
            .end_states
            .clone()
            .into_iter()
            .collect::<BTreeSet<usize>>();

        let get_label = |node| {
            if node == self.start_state {
                format!("s:{}", node)
            } else if finals_nodes.contains(&node) {
                format!("e:{}", node)
            } else {
                format!("{}", node)
            }
        };

        let translate_table = (0..self.num_states)
            .map(|node| (node, graph.add_node(Some(get_label(node)))))
            .collect::<BTreeMap<usize, usize>>();

        self.transitions.iter().enumerate().for_each(|(from, adj)| {
            adj.iter().for_each(|(label, to_list)| {
                to_list.iter().for_each(|to| {
                    graph.add_edge(
                        translate_table[&from],
                        translate_table[to],
                        Some(format!("{}", label)),
                    );
                });
            })
        });
        let _start_node = translate_table[&self.start_state];
        // TODO: attenzione a quellli che vanno nello stesso nodo
        graph
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn display_test() {
        let regex = RE::ReOperator::Or(
            Box::new(RE::ReOperator::Char('a')),
            Box::new(RE::ReOperator::Char('b')),
        );
        let nfa = NFA::from(&regex);
        println!("{:?}", nfa);
    }
}
