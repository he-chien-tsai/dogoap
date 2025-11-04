use crate::{datum::Datum, localstate::InternalData};

use bevy_reflect::*;

/// Describes a change in [`LocalState`](crate::localstate::LocalState), based on
/// the String key + a [`Datum`]
#[derive(Reflect, Clone, Debug, PartialEq, Hash, Eq)]
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

pub fn print_mutators(mutators: Vec<Mutator>) {
    for mutator in mutators {
        match mutator {
            Mutator::Set(k, v) => {
                info!("\t\t{k} = {v}");
            }
            Mutator::Increment(k, v) => {
                info!("\t\t{k} + {v}");
            }
            Mutator::Decrement(k, v) => {
                info!("\t\t{k} - {v}");
            }
        }
    }
}
