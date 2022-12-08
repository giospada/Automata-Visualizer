use std::collections::{BTreeMap, BTreeSet};

use crate::automata::nfa::NFA;
use crate::automata::regular_expression as RE;
use crate::utils::graph::{Graph, IndEdge, IndNode};
use crate::automata::dfa::{DFA, NfaStates};

impl From<&NFA> for DFA<NfaStates> {
    fn from(nfa: &NFA) -> Self {
        let mut dfa = DFA::new();
        let alphabet = nfa.get_alphabet();

        let mut state_to_index: BTreeMap<NfaStates, usize> = BTreeMap::new();

        let start = nfa.epsilon_closure(&vec![nfa.get_start_state()]);
        let state_num = Self::add_state(&mut dfa, start);
        dfa.start_state = state_num;
        let mut queue = vec![state_num];

        while !queue.is_empty() {
            let current_state = queue.pop().unwrap();
            let current_set: NfaStates = dfa.idx_to_data.as_ref().unwrap()[&current_state].clone();

            if nfa.contains_final_state(&current_set) {
                dfa.end_states.push(current_state);
            }

            for alphabet_char in &alphabet {
                let mut next_set = nfa.make_move(&current_set, alphabet_char.clone());
                next_set = nfa.epsilon_closure(&next_set.into_iter().collect());

                if !state_to_index.contains_key(&next_set) {
                    let next_state = Self::add_state(&mut dfa, next_set.clone());
                    state_to_index.insert(next_set.clone(), next_state);
                    queue.push(next_state);
                }
                let next_state = state_to_index.get(&next_set).unwrap();
                dfa.transitions[current_state].insert(*alphabet_char, *next_state);
            }
        }
        dfa.alphabet = alphabet;

        dfa
    }
}

impl From<&RE::ReOperator> for DFA<NfaStates> {
    fn from(regex: &RE::ReOperator) -> Self {
        let nfa = NFA::from(regex);
        DFA::from(&nfa)
    }
}


impl<T> Into<Graph> for DFA<T> {
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
        // map the node_id in the dfa to the node id in the graph
        let translate_table = (0..self.num_states)
            .map(|node| (node, graph.add_node(Some(get_label(node)))))
            .collect::<BTreeMap<usize, IndNode>>();

        self.transitions.iter().enumerate().for_each(|(from, adj)| {
            // we compact all edge that go to the same node
            // and we upate the label for all the node added
            let mut added_edge: BTreeMap<IndNode, IndEdge> = BTreeMap::new();
            adj.iter().for_each(|(label, to)| {
                let ind = added_edge.entry(*to).or_insert(graph.add_edge(
                    translate_table[&from],
                    translate_table[to],
                    None,
                ));
                let old_label = graph.modify_edge_label(*ind);
                *old_label = match old_label {
                    Some(val) => Some(format!("{},{}", val, label)),
                    None => Some(format!("{}", label)),
                }
            })
        });
        let _start_node = translate_table[&self.start_state];
        graph
    }
}
