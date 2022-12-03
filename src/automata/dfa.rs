use std::collections::{BTreeMap, BTreeSet};

use crate::display::display_graph::{DisplayGraph};
use crate::automata::nfa::NFA;
use crate::automata::regular_expression as RE;
use crate::utils::DisjointUnionFind::DisjointUnionFind;


#[derive(Debug, Clone)]
pub struct DFA {
    num_states: usize,
    start_state: usize,
    end_states: Vec<usize>,
    transitions: Vec<BTreeMap<char, usize>>,
    alphabet: Vec<char>,


    // TODO: use this in visualization
    idx_to_nfa_states: Option<BTreeMap<usize, BTreeSet<usize>>>,
}

const INVALID_STATE: i32 = -1;

impl DFA {
    fn new() -> Self {
        Self {
            num_states: 0,
            start_state: 0,
            end_states: Vec::new(),
            transitions: Vec::new(),
            alphabet: Vec::new(),
            idx_to_nfa_states: None,
        }
    }

    /// Returns a DFA described by the input parameters.
    /// if alphabet is None, the alphabet is inferred from the transitions.
    pub fn from_state(
        num_states: usize,
        start_state: usize, 
        end_states: Vec<usize>, 
        transitions: Vec<BTreeMap<char, usize>>,
        alphabet: Option<Vec<char>>,
    ) -> Self {
        let alphabet = alphabet.unwrap_or_else(|| {
            let mut alphabet = Vec::new();
            for transition in &transitions {
                for c in transition.keys() {
                    // NOTE: this is O(n) but check for contain, but alphabet is small
                    // so it wouldn't be a problem, remember if later you need to optimize
                    if !alphabet.contains(c) {
                        alphabet.push(*c);
                    }
                }
            }
            alphabet
        });

        Self {
            num_states: num_states,
            start_state,
            end_states,
            transitions,
            alphabet,
            idx_to_nfa_states: None,
        }
    }

    pub fn get_start_state(&self) -> usize {
        self.start_state
    }

    pub fn get_end_states(&self) -> &Vec<usize> {
        &self.end_states
    }

    pub fn get_transitions(&self) -> &Vec<BTreeMap<char, usize>> {
        &self.transitions
    }

    pub fn get_minimized_dfa(&self) -> Self {
        let equivalent_states = self.get_equivalent_states();
        let mut unequal_sets = DisjointUnionFind::new(self.num_states);

        // join all equivalent states
        for (first, second) in equivalent_states {
            unequal_sets.join(first, second);
        }

        // create mapping from old index, to new index
        let mut head_to_idx = BTreeMap::new();
        let mut curr_idx = 0;
        for i in 0..self.num_states {
            if unequal_sets.is_head(&i) {
                head_to_idx.insert(i, curr_idx);
                curr_idx += 1;
            }
        }
        
        // create new transitions mapper
        let num_states = unequal_sets.get_size();
        let mut new_transitions = vec![BTreeMap::new(); num_states];
        for (head, idx) in head_to_idx.iter() {
            for (transition_ch, dest) in self.transitions[*head].iter() {
                let dest_head = unequal_sets.find(*dest);
                new_transitions[*idx].insert(*transition_ch, *head_to_idx.get(&dest_head).unwrap());
            }
        }
        
        // create new end states
        let mut new_end_states = vec![];
        for end_state in self.end_states.iter() {
            let head = unequal_sets.find(*end_state);
            new_end_states.push(*head_to_idx.get(&head).unwrap());
        }
        
        Self {
            num_states,
            start_state: *head_to_idx.get(&unequal_sets.find(self.start_state)).unwrap(),
            end_states: new_end_states,
            transitions: new_transitions,
            alphabet: self.alphabet.clone(),
            idx_to_nfa_states: None,
        }
    }


    pub fn make_move(&self, state: usize, input: char) -> usize {
        if state > self.num_states {
            panic!("Invalid state");
        } else if !self.alphabet.contains(&input) {
            panic!("Invalid input character");
        } 

        self.transitions[state].get(&input).unwrap().clone()
    }

    fn get_equivalent_states(&self) -> Vec<(usize, usize)> {
        let minimize_table = self.compute_minimize_table();

        let mut equivalent_states = vec![];
        for i in 0..self.num_states {
            for j in i + 1..self.num_states {
                if minimize_table[i][j] == INVALID_STATE {
                    equivalent_states.push((i, j));
                }
            }
        }

        equivalent_states
    }

    fn compute_minimize_table(&self) -> Vec<Vec<i32>> {
        // more than half of the space in the minimize table is wasted
        // because we only need to store the upper triangle
        // buts its easier to index into the table this way
        let mut minimize_table = self.initialize_minimize_table();

        let mut has_changed = true;
        let mut curr_iter = 1;
        while has_changed {
            has_changed = false;

            for i in 0..self.num_states {
                for j in i + 1..self.num_states {
                    if minimize_table[i][j] != INVALID_STATE {
                        continue;
                    }

                    for  alphabet_ch in &self.alphabet {
                        let mut next_i = self.make_move(i, *alphabet_ch);
                        let mut next_j = self.make_move(j, *alphabet_ch);

                        // so i always index in the upper triangle
                        if next_i > next_j {
                            std::mem::swap(&mut next_i, &mut next_j);
                        }

                        if minimize_table[next_i][next_j] != INVALID_STATE {
                            minimize_table[i][j] = curr_iter;
                            has_changed = true;
                            break;
                        }
                    }
                }
            }

            curr_iter += 1;
        }

        minimize_table
    }

    /// @returns the initialized minimized table and vector of states to merge, with
    /// just stage-0 un-equal states marked
    fn initialize_minimize_table(&self) -> Vec<Vec<i32>> {
        let mut minimize_table = vec![vec![INVALID_STATE; self.num_states]; self.num_states];

        for i in 0..self.num_states {
            for j in i + 1..self.num_states {
                let i_is_final = self.is_final_state(i);
                let j_is_final = self.is_final_state(j);

                if i_is_final != j_is_final {
                    minimize_table[i][j] = 0;
                }
            }
        }

        minimize_table
    }

    pub fn is_final_state(&self, state: usize) -> bool {
        self.end_states.contains(&state)
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
        let alphabet = nfa.get_alphabet();

        let mut state_to_index: BTreeMap<BTreeSet<usize>, usize> = BTreeMap::new();

        let start = nfa.epsilon_closure(&vec![nfa.get_start_state()]);
        let state_num = Self::add_state(&mut dfa, start);
        dfa.start_state = state_num;
        let mut queue = vec![state_num];

        while !queue.is_empty() {
            let current_state = queue.pop().unwrap();
            let current_set: BTreeSet<usize> = dfa.idx_to_nfa_states.as_ref().unwrap()[&current_state].clone();

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
