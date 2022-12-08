use std::collections::{BTreeMap, BTreeSet};

use crate::utils::DisjointUnionFind::DisjointUnionFind;

mod canonical;
mod from_into;

pub type NfaStates = BTreeSet<usize>;

#[derive(Debug, Clone)]
pub struct DFA<T> {
    num_states: usize,
    start_state: usize,
    end_states: Vec<usize>,
    transitions: Vec<BTreeMap<char, usize>>,
    alphabet: Vec<char>,

    // TODO: use this in visualization
    idx_to_data: Option<BTreeMap<usize, T>>,
}

const INVALID_STATE: i32 = -1;

impl<T> DFA<T> {
    fn new() -> Self {
        Self {
            num_states: 0,
            start_state: 0,
            end_states: Vec::new(),
            transitions: Vec::new(),
            alphabet: Vec::new(),
            idx_to_data: None,
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
            idx_to_data: None,
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
        
        // create new end states, these should be unique
        let mut new_end_states = BTreeSet::new();
        for end_state in self.end_states.iter() {
            let head = unequal_sets.find(*end_state);
            new_end_states.insert(*head_to_idx.get(&head).unwrap());
        }

        Self {
            num_states,

            start_state: *head_to_idx.get(&unequal_sets.find(self.start_state)).unwrap(),
            end_states: new_end_states.into_iter().collect(),
            transitions: new_transitions,
            alphabet: self.alphabet.clone(),
            idx_to_data: None,
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

                    for alphabet_ch in &self.alphabet {
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

    fn add_state(dfa: &mut DFA<T>, states: T) -> usize {
        dfa.transitions.push(BTreeMap::new());

            
        if let Some(map) = &mut dfa.idx_to_data {
            map.insert(dfa.num_states, states);
        } else {
            dfa.idx_to_data = Some(BTreeMap::new());
            dfa.idx_to_data.as_mut().unwrap().insert(dfa.num_states, states);
        }

        dfa.num_states += 1;
        dfa.num_states - 1
    }
}
