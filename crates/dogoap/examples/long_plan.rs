//! This example shows how a plan with lots of steps can be created.

use dogoap::{
    prelude::*,
    simple::{simple_decrement_action, simple_increment_action},
};

fn main() {
    let start = LocalState::new()
        .with_datum("energy", 30_i64)
        .with_datum("hunger", 70_i64)
        .with_datum("gold", 0_i64);

    let expected_state = LocalState::new()
        .with_datum("energy", 50_i64)
        .with_datum("hunger", 50_i64)
        .with_datum("gold", 7_i64);

    let goal = Goal::new().with_req("gold", Compare::equals(7_i64));

    // TOOD should keep the `10 as 64` syntax with .from somehow
    let sleep_action = Action::new("sleep").with_mutator(Mutator::increment("energy", 10_i64));

    let eat_action = simple_decrement_action("eat", "hunger", 10_i64)
        .with_precondition(("energy", Compare::greater_than_equals(26_i64)));

    let rob_people = simple_increment_action("rob", "gold", 1_i64)
        .with_effect(Effect {
            action: "rob".to_string(),
            mutators: vec![
                Mutator::decrement("energy", 5_i64),
                Mutator::increment("hunger", 5_i64),
            ],
            state: LocalState::default(),
            cost: 1,
        })
        .with_precondition(("hunger", Compare::less_than_equals(50_i64)))
        .with_precondition(("energy", Compare::greater_than_equals(50_i64)));

    let actions: Vec<Action> = vec![sleep_action, eat_action, rob_people];

    let plan = make_plan(&start, &actions[..], &goal);
    let effects = get_effects_from_plan(plan.clone().unwrap().0).collect::<Vec<_>>();
    assert_eq!(11, effects.len());

    println!("{}", format_plan(plan.clone().unwrap()));

    // visualize_plan(plan.unwrap(), "my-plan.dot");

    assert_eq!(expected_state, effects.last().unwrap().state);
}
