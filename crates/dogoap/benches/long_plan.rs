#![expect(missing_docs, reason = "No need for docs in benchmarks")]
//! Benchmark for the `long_plan` example.

use criterion::{Criterion, criterion_group, criterion_main};
use dogoap::{
    prelude::*,
    simple::{simple_decrement_action, simple_increment_action},
};
use std::hint::black_box;

fn long_plan(strategy: PlanningStrategy) {
    let start = LocalState::new()
        .with_datum("energy", 30_i64)
        .with_datum("hunger", 70_i64)
        .with_datum("gold", 0_i64);

    let expected_state = LocalState::new()
        .with_datum("energy", 50_i64)
        .with_datum("hunger", 50_i64)
        .with_datum("gold", 7_i64);

    let goal = Goal::new().with_req("gold", Compare::equals(7_i64));

    let sleep_action = simple_increment_action("sleep", "energy", 10_i64);

    let eat_action = simple_decrement_action("eat", "hunger", 10_i64)
        .with_precondition(("energy", Compare::greater_than_equals(25_i64)));

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

    let plan = make_plan_with_strategy(strategy, &start, &actions[..], &goal);
    let effects = get_effects_from_plan(plan.clone().unwrap().0).collect::<Vec<_>>();

    assert_eq!(11, effects.len());
    assert_eq!(expected_state, effects.last().unwrap().state);
}

fn bench_start_to_goal_strategy(c: &mut Criterion) {
    c.bench_function("Start To Goal", |b| {
        b.iter(|| long_plan(black_box(PlanningStrategy::StartToGoal)));
    });
}

criterion_group!(benches, bench_start_to_goal_strategy);
criterion_main!(benches);
