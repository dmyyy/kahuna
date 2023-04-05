use crate::{CollapseRule, Final, InvertDelta, SetState, Space, State};
use bevy_utils::HashMap;
use rand::{thread_rng, Rng};
use std::hash::Hash;

pub trait SetCollapseObserver<S: State> {
    fn observe(&self, cell: &mut S, neighbors: &[Option<S>]);
}

#[derive(Clone)]
pub struct UniformSetCollapseObserver;

impl<S: SetState + State + Clone> SetCollapseObserver<S> for UniformSetCollapseObserver {
    fn observe(&self, cell: &mut S, _: &[Option<S>]) {
        let mut final_states = Vec::new();
        cell.collect_final_states(&mut final_states);
        *cell = final_states[thread_rng().gen_range(0..final_states.len())].clone();
    }
}

#[derive(Clone)]
pub struct WeightedSetCollapseObserver<T: Eq + Hash + Clone> {
    pub weights: HashMap<T, u32>,
}

// States are chosen based on their relative weight compared to the sum of weights of all states.
// States with a weight of 0 will never be chosen by the observer (they can still be picked by the
// algorithm if that's the only possible remaining state)
impl<S: SetState + State + Clone + Final<T>, T: Eq + Hash + Clone> SetCollapseObserver<S>
    for WeightedSetCollapseObserver<T>
{
    fn observe(&self, cell: &mut S, _: &[Option<S>]) {
        let mut final_states = Vec::new();
        cell.collect_final_states(&mut final_states);

        // calculate running sum of all weights for each state
        let mut weight_vec: Vec<u32> = vec![0; final_states.len()];
        if !final_states.is_empty() {
            let state = final_states[0].get().unwrap();
            weight_vec[0] = *self.weights.get(&state).unwrap();
        }
        for (i, final_state) in final_states.iter().enumerate().skip(1) {
            let state = final_state.get().unwrap();
            weight_vec[i] = *self.weights.get(&state).unwrap() + weight_vec[i - 1];
        }

        let rand = thread_rng().gen_range(0..*weight_vec.last().unwrap());
        let mut prev = 0;
        for (i, weight) in weight_vec.into_iter().enumerate() {
            if weight >= rand && weight - prev != 0 {
                *cell = final_states[i].clone();
                return;
            }
            prev = weight;
        }
    }
}

pub struct SetCollapseRule<S: SetState + State + Sized, Sp: Space<S>, O: SetCollapseObserver<S>> {
    neighbor_offsets: Box<[Sp::CoordinateDelta]>,
    state_rules: Box<[(S, Box<[Option<S>]>)]>,
    observer: O,
}

struct StateRule<S> {
    state: S,
    allowed_neighbors: Vec<Option<S>>,
}

impl<S: SetState + Clone> StateRule<S> {
    fn add_allowed(&mut self, neighbor_index: usize, allowed: &S) {
        while self.allowed_neighbors.len() <= neighbor_index {
            self.allowed_neighbors.push(None);
        }
        if let Some(allowed_neighbors) = &mut self.allowed_neighbors[neighbor_index] {
            allowed_neighbors.set_states(allowed);
        } else {
            self.allowed_neighbors[neighbor_index] = Some(allowed.clone());
        }
    }
}

// Builder for SetCollapseRule
pub struct SetCollapseRuleBuilder<
    S: SetState + State,
    Sp: Space<S>,
    O: SetCollapseObserver<S> + Clone,
> {
    neighbor_offsets: Vec<Sp::CoordinateDelta>,
    state_rules: Vec<StateRule<S>>,
    observer: O,
    all_state: S,
}

impl<S: SetState + State + PartialEq, Sp: Space<S>, O: SetCollapseObserver<S> + Clone>
    SetCollapseRuleBuilder<S, Sp, O>
where
    Sp::CoordinateDelta: Eq + Clone + InvertDelta,
{
    pub fn new(observer: O, all_state: S) -> Self {
        Self {
            neighbor_offsets: Vec::new(),
            state_rules: Vec::new(),
            observer,
            all_state,
        }
    }

    // Set the allowed neighbors for a cell based on their coordinate deltas
    //
    // Rules aren't added symmetrically - only provided rules will be added
    //
    // States which do not have any allowed neighbors for a given coordinate
    // delta will require that those coordinates are outside of world-space.
    pub fn allow(mut self, state: &S, neighbors: &[(Sp::CoordinateDelta, S)]) -> Self {
        let mut states = Vec::new();
        state.collect_final_states(&mut states);
        for state in states {
            for (delta, neighbor) in neighbors {
                let mut neighbor_states = Vec::new();
                neighbor.collect_final_states(&mut neighbor_states);
                for n_state in neighbor_states {
                    let offset_index = self.get_offset_index(delta.clone());
                    self.get_rule(&state).add_allowed(offset_index, &n_state);
                }
            }
        }
        self
    }

    fn get_offset_index(&mut self, offset: Sp::CoordinateDelta) -> usize {
        for i in 0..self.neighbor_offsets.len() {
            if self.neighbor_offsets[i] == offset {
                return i;
            }
        }
        let i = self.neighbor_offsets.len();
        self.neighbor_offsets.push(offset);
        i
    }

    fn get_rule(&mut self, state: &S) -> &mut StateRule<S> {
        for i in 0..self.state_rules.len() {
            if &self.state_rules[i].state == state {
                return &mut self.state_rules[i];
            }
        }
        self.state_rules.push(StateRule {
            state: state.clone(),
            allowed_neighbors: Vec::new(),
        });
        let index = self.state_rules.len() - 1;
        &mut self.state_rules[index]
    }

    pub fn build(self) -> SetCollapseRule<S, Sp, O> {
        let mut state_rules = Vec::new();
        let mut remaining_state = self.all_state;
        for mut proto_rule in self.state_rules {
            while proto_rule.allowed_neighbors.len() < self.neighbor_offsets.len() {
                proto_rule.allowed_neighbors.push(None);
            }
            remaining_state.clear_states(&proto_rule.state);
            state_rules.push((
                proto_rule.state,
                proto_rule.allowed_neighbors.into_boxed_slice(),
            ));
        }
        let mut remaining_states = Vec::new();
        remaining_state.collect_final_states(&mut remaining_states);
        for remaining_state in remaining_states {
            state_rules.push((
                remaining_state,
                vec![None; self.neighbor_offsets.len()].into_boxed_slice(),
            ));
        }
        SetCollapseRule {
            neighbor_offsets: self.neighbor_offsets.into_boxed_slice(),
            state_rules: state_rules.into_boxed_slice(),
            observer: self.observer,
        }
    }
}

// A collapse rule implementation that works with implementors of [crate::SetState<T>]
impl<S: SetState + State, Sp: Space<S>, O: SetCollapseObserver<S>> CollapseRule<S, Sp>
    for SetCollapseRule<S, Sp, O>
where
    Sp::CoordinateDelta: Clone,
{
    fn neighbor_offsets(&self) -> Box<[<Sp as Space<S>>::CoordinateDelta]> {
        self.neighbor_offsets.clone()
    }

    fn collapse(&self, cell: &mut S, neighbors: &[Option<S>]) {
        for (state, allowed_neighbors) in &self.state_rules[..] {
            if cell.has_any_of(state) {
                for i in 0..neighbors.len() {
                    if let Some(neighbor_state) = &neighbors[i] {
                        let allow = if let Some(allowed_state) = &allowed_neighbors[i] {
                            neighbor_state.has_any_of(allowed_state)
                        } else {
                            false
                        };
                        if !allow {
                            cell.clear_states(state)
                        }
                    }
                }
            }
        }
    }

    fn observe(&self, cell: &mut S, neighbors: &[Option<S>]) {
        self.observer.observe(cell, neighbors);
    }
}
