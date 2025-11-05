use crate::prelude::*;

pub fn simple_action(
    name: impl Into<String>,
    key_to_mutate: impl Into<String>,
    from_value: impl Into<Datum>,
) -> Action {
    simple_multi_mutate_action(name, vec![(key_to_mutate, from_value)])
}

pub fn simple_multi_mutate_action(
    name: impl Into<String>,
    muts: Vec<(impl Into<String>, impl Into<Datum>)>,
) -> Action {
    let mut mutators = vec![];

    for (key, value) in muts {
        mutators.push(Mutator::set(key, value));
    }

    let name = name.into();
    Action {
        key: name.clone(),
        preconditions: vec![],
        effects: vec![Effect {
            action: name,
            mutators,
            state: LocalState::new(),
            cost: 1,
        }],
    }
}

pub fn simple_increment_action(
    name: &str,
    key_to_mutate: impl Into<String>,
    from_value: impl Into<Datum>,
) -> Action {
    Action::new(name).with_mutator(Mutator::increment(key_to_mutate, from_value))
}

pub fn simple_decrement_action(
    name: &str,
    key_to_mutate: impl Into<String>,
    from_value: impl Into<Datum>,
) -> Action {
    Action::new(name).with_mutator(Mutator::decrement(key_to_mutate, from_value))
}
