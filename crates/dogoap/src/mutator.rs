use crate::{datum::Datum, localstate::InternalData};

/// Describes a change in [`LocalState`](crate::localstate::LocalState), based on
/// the String key + a [`Datum`]
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum Mutator {
    /// Set a value for a key
    Set(String, Datum), // :key, :value
    /// Increment a value for a key by a given amount
    Increment(String, Datum), // :key, :increment-by
    /// Decrement a value for a key by a given amount
    Decrement(String, Datum), // :key, :decrement-by
}

pub fn apply_mutator(data: &mut InternalData, mutator: &Mutator) {
    match mutator {
        Mutator::Set(key, value) => {
            data.insert(key.to_string(), *value);
        }
        Mutator::Increment(key, value) => {
            if let Some(current_value) = data.get_mut(key) {
                *current_value += *value;
            }
        }
        Mutator::Decrement(key, value) => {
            if let Some(current_value) = data.get_mut(key) {
                *current_value -= *value;
            }
        }
    }
}

/// Formats a human-readable version of a list of [`Mutator`]s.
/// Used in [`format_plan`](crate::prelude::format_plan).
pub fn format_mutators(mutators: Vec<Mutator>) -> String {
    let mut output = String::new();
    for mutator in mutators {
        match mutator {
            Mutator::Set(k, v) => {
                output.push_str(&format!("\t\t{k} = {v}\n"));
            }
            Mutator::Increment(k, v) => {
                output.push_str(&format!("\t\t{k} + {v}\n"));
            }
            Mutator::Decrement(k, v) => {
                output.push_str(&format!("\t\t{k} - {v}\n"));
            }
        }
    }
    output
}
