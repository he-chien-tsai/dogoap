use crate::{localstate::LocalState, mutator::Mutator};
use std::hash::{Hash, Hasher};

/// The effect is what happens when an Action is applied.
///
/// It's separated from Action in order to separate the
/// data structures for the Planner's Node that is used
/// for the pathfinding part.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Effect {
    /// The name of the action that caused this effect.
    pub action: String,
    /// The [`Mutator`] that are active when this effect is applied.
    pub mutators: Vec<Mutator>,
    /// An internal state used while path finding
    pub state: LocalState,
    /// The cost of applying this effect. Default is 1.
    pub cost: usize,
}

impl Effect {
    /// Creates a new effect with the given action name.
    pub fn new(action_name: &str) -> Self {
        Self {
            action: action_name.to_string(),
            mutators: vec![],
            state: LocalState::new(),
            cost: 1,
        }
    }

    /// Adds a mutator to the effect. An effect can have an arbitrary nonzero number of mutators.
    pub fn with_mutator(mut self, mutator: Mutator) -> Self {
        self.mutators.push(mutator);
        self
    }
}

impl Hash for Effect {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mutators.hash(state);
        self.state.hash(state);
        self.cost.hash(state);
    }
}
