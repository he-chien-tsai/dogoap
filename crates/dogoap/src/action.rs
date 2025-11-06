// crate "dogoap" file action.rs
use std::hash::{Hash, Hasher};

use crate::compare::Compare;
use crate::effect::Effect;
use crate::mutator::Mutator;

/// An `Action` represents something your Entity can do, granted the `LocalState`
/// is as defined in the `preconditions`. It has a list of `Effect`s that apply
/// if the NPC successfully executed the task.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Action {
    /// String like `eat_action`
    pub key: String,
    // TODO arguments coupled with Effects, maybe
    // pub argument: Option<Datum>,
    /// What preconditions need to be true before we can execute this action
    pub preconditions: Vec<(String, Compare)>,
    /// What is the outcome from doing this action
    // TODO temporarily plural effects, as maybe we want to implement arguments with many effects...
    pub effects: Vec<Effect>,
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.preconditions.hash(state);
        self.effects.hash(state);
    }
}

impl Action {
    /// Create a new action with the given key.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            preconditions: vec![],
            effects: vec![],
        }
    }

    /// Add an effect to the action, i.e. something that will be true after the action is executed.
    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.effects.push(effect);
        self
    }

    /// Add a precondition to the action, i.e. something that must be true before the action can be executed.
    pub fn with_precondition(mut self, (key, compare): (impl Into<String>, Compare)) -> Self {
        self.preconditions.push((key.into(), compare));
        self
    }

    // TODO currently only handles one effect
    /// Adds a mutator to the action's effect, i.e. something that will be mutated after the action is executed.
    /// An effect can have an arbritrary nonzero amount of mutators.
    pub fn with_mutator(mut self, mutator: Mutator) -> Self {
        if self.effects.is_empty() {
            self.effects = vec![Effect::new(&self.key.clone()).with_mutator(mutator)];
        } else {
            let mut effect = self.effects[0].clone();
            effect.mutators.push(mutator);
            self.effects[0] = effect;
        }
        self
    }

    /// Set the effect's cost.
    pub fn set_cost(mut self, new_cost: usize) -> Self {
        let mut effect = self.effects[0].clone();
        effect.cost = new_cost;
        self.effects[0] = effect;
        self
    }
}
