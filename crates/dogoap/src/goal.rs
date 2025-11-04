use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use crate::compare::Compare;

/// Goal is a map of what we want our final [`LocalState`](crate::localstate::LocalState) to be, using String as
/// keys and [`Compare`] to assert what we want the [`Datum`](crate::datum::Datum) to be
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Goal {
    /// All the requirements needed to be met in order to consider us to be at our final state
    pub requirements: BTreeMap<String, Compare>,
}

impl Hash for Goal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.requirements.len().hash(state);
        for (key, value) in &self.requirements {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Default for Goal {
    fn default() -> Self {
        Self::new()
    }
}

impl Goal {
    /// Create a new empty goal
    pub fn new() -> Self {
        Self {
            requirements: BTreeMap::new(),
        }
    }

    /// Create a new goal with a single requirements
    pub fn with_req(mut self, key: &str, compare: Compare) -> Self {
        self.requirements.insert(key.to_string(), compare);
        self
    }

    /// Create a new goal from a list of requirements
    pub fn from_reqs(preconditions: &[(String, Compare)]) -> Goal {
        let mut goal = Goal::new();
        for (k, v) in preconditions {
            goal = goal.with_req(k, v.clone());
        }
        goal
    }
}
