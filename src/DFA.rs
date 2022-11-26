use std::collections::{BTreeMap, BTreeSet};
use crate::DisplayGraph::{DisplayGraph};
use crate::NFA::NFA;
use crate::RegularExpression as RE;
use crate::Log::log;

#[derive(Debug)]
pub struct DFA {
    num_states: usize,
    start_state: usize,
    end_states: Vec<usize>,
    transitions: Vec<BTreeMap<char, usize>>,

    // TODO: use this in visualization
    idx_to_nfa_states: Option<BTreeMap<usize, BTreeSet<usize>>>,
}

impl DFA {
    fn new() -> Self {
        Self {
            num_states: 0,
            start_state: 0,
            end_states: Vec::new(),
            transitions: Vec::new(),
            idx_to_nfa_states: None,
        }
    }

    fn add_state(dfa: &mut DFA, states: BTreeSet<usize>) -> usize {
        dfa.transitions.push(BTreeMap::new());
            
        if let Some(map) = &mut dfa.idx_to_nfa_states {
            map.insert(dfa.num_states, states.clone());
        } else {
            dfa.idx_to_nfa_states = Some(BTreeMap::new());
            dfa.idx_to_nfa_states.as_mut().unwrap().insert(dfa.num_states, states.clone());
        }
        
        dfa.num_states += 1; 
        dfa.num_states - 1
    }
}

impl From<&NFA> for DFA {
    fn from(nfa: &NFA) -> Self {
        let mut dfa = DFA::new();
        let mut map_to_used = BTreeMap::new();

        let start = nfa.epsilon_closure(&vec![nfa.get_start_state()]);
        let state_num = Self::add_state(&mut dfa, start);
        dfa.start_state = state_num;
        let mut queue = vec![state_num];

        while !queue.is_empty() {
            let current_state = queue.pop().unwrap();
            let current_set = dfa.idx_to_nfa_states.as_ref().unwrap()[&current_state].clone();

            for alphabet_char in nfa.get_alphabet() {
                let mut next_set = BTreeSet::new();

                for state in current_set.iter() {
                    if nfa.get_transitions()[*state].contains_key(&alphabet_char) {
                        for next_state in &nfa.get_transitions()[*state][&alphabet_char] {
                            next_set.insert(*next_state);
                        }
                    }
                }

                let next_set = nfa.epsilon_closure(&next_set.into_iter().collect());
                if !map_to_used.contains_key(&next_set) {
                    let next_state = Self::add_state(&mut dfa, next_set.clone());
                    map_to_used.insert(next_set.clone(), next_state);
                    queue.push(next_state);
                }
                let next_state = map_to_used.get(&next_set).unwrap();
                dfa.transitions[current_state].insert(alphabet_char, *next_state);
            }
        }

        dfa
    }
}

impl From<&RE::ReOperator> for DFA {
    fn from(regex: &RE::ReOperator) -> Self {
        let nfa = NFA::from(regex);
        DFA::from(&nfa)
    }
}

impl Into<DisplayGraph> for DFA {
    fn into(self) -> DisplayGraph {
        let mut visited = vec![false; self.num_states];
        let mut to_visit = vec![];
        let mut graph = vec![];
        let mut labels = vec![];

        graph.push(vec![self.start_state as usize]);        
        to_visit.push(self.start_state);
        visited[self.start_state] = true;

        while !to_visit.is_empty() {
            let mut current_nodes=vec![];
            let mut new_to_visit =vec![];    
            for index in to_visit{
                current_nodes.push(index);
                labels.push(index.to_string());

                for i in self.transitions[index].keys(){
                    if !visited[self.transitions[index][i]] {
                        visited[self.transitions[index][i]] = true;
                        new_to_visit.push(self.transitions[index][i]);
                    }
                }
            }

            graph.push(current_nodes);
            to_visit = new_to_visit;
        }

        let mut edge: Vec<(usize,usize,Option<String>)> = Vec::new();

        for (from, _edgemap) in self.transitions.iter().enumerate() {
            let mut collect_edge:BTreeMap<usize,String> = BTreeMap::new();

            for (label,to) in self.transitions[from].iter() {
                collect_edge.entry(*to)
                    .and_modify(|x| *x = format!("{},{}", *x, label))
                    .or_insert(format!("{}", *label));
            }
            for (to,label) in collect_edge.iter() {
                edge.push((from,*to,Some(label.clone())))
            }
        }

        labels[self.start_state] = format!("s:{}",labels[self.start_state]);
        for end_state in &self.end_states {
            labels[*end_state] = format!("e:{}",labels[*end_state]);
        }

        DisplayGraph::new(edge,labels,graph)
    }
}
