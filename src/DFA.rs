use std::collections::{BTreeMap, BTreeSet};
use crate::DisplayGraph::{DisplayGraph, ToDisplayGraph};
use crate::NFA::NFA;

#[derive(Debug)]
pub struct DFA {
    num_states: usize,
    start_state: usize,
    end_states: Vec<usize>,
    transitions: Vec<BTreeMap<char, usize>>,

    // TODO: use this in visualization
    map_to_nfa_states: Option<BTreeMap<usize, BTreeSet<usize>>>,
}

fn to_set(vector: Vec<usize>) -> BTreeSet<usize> {
    let mut set = BTreeSet::new();
    for i in vector {
        set.insert(i);
    }
    set
}

impl DFA {
    fn new() -> Self {
        Self {
            num_states: 0,
            start_state: 0,
            end_states: Vec::new(),
            transitions: Vec::new(),
            map_to_nfa_states: None,
        }
    }

    pub fn from_nfa(nfa: &NFA) -> Self {
        let mut dfa = DFA::new();
        let mut map_to_used = BTreeMap::new();

        let start = nfa.epsilon_closure(&vec![nfa.get_start_state()]);
        let state_num = Self::add_state(&mut dfa, to_set(start));
        dfa.start_state = state_num;
        let mut queue = vec![state_num];

        while !queue.is_empty() {
            let current_state = queue.pop().unwrap();
            let current_set = dfa.map_to_nfa_states.as_ref().unwrap()[&current_state].clone();

            for c in NFA::get_alphabet().chars() {
                let mut next_set = BTreeSet::new();

                for state in current_set.iter() {
                    if nfa.get_transitions()[*state].contains_key(&c) {
                        for next_state in &nfa.get_transitions()[*state][&c] {
                            next_set.insert(*next_state);
                        }
                    }
                }

                let next_set = nfa.epsilon_closure(&next_set.into_iter().collect());
                let next_set = to_set(next_set);
                if !map_to_used.contains_key(&next_set) {
                    let next_state = Self::add_state(&mut dfa, next_set.clone());
                    map_to_used.insert(next_set.clone(), next_state);
                    queue.push(next_state);
                }
                let next_state = map_to_used.get(&next_set).unwrap();
                dfa.transitions[current_state].insert(c, *next_state);
            }
        }

        dfa
    }

    fn add_state(dfa: &mut DFA, states: BTreeSet<usize>) -> usize {
        dfa.transitions.push(BTreeMap::new());
            
        if let Some(map) = &mut dfa.map_to_nfa_states {
            map.insert(dfa.num_states, states.clone());
        } else {
            dfa.map_to_nfa_states = Some(BTreeMap::new());
            dfa.map_to_nfa_states.as_mut().unwrap().insert(dfa.num_states, states.clone());
        }
        
        dfa.num_states += 1; 
        dfa.num_states - 1
    }
}


impl ToDisplayGraph for DFA{
    fn to_display_graph(&self) -> DisplayGraph {
        let mut done=vec![false;self.num_states];
        let mut child =vec![];
        let mut graph=vec![];
        let mut labels=vec![];
        let mut edge:Vec<(usize,usize,Option<char>)>=Vec::new();
        graph.push(vec![0 as usize]);        
        child.push(0);
        done[0]=true;
        while !child.is_empty() {
            let mut current_nodes=vec![];
            let mut newchild =vec![];    
            for index in child{
                current_nodes.push(index);
                labels.push(index.to_string());

                for i in self.transitions[index].keys(){
                    edge.push((index,self.transitions[index][i],Some(*i)));
                    if !done[self.transitions[index][i]] {
                        done[self.transitions[index][i]]=true;
                        newchild.push(self.transitions[index][i]);
                    }
                }

            }
            graph.push(current_nodes);
            child=newchild;
        }
        labels[self.start_state] = format!("s:{}",labels[self.start_state]);
        for end_state in &self.end_states {
            labels[*end_state] = format!("e:{}",labels[*end_state]);
        }
        DisplayGraph::new(edge,labels,graph)
    }
}