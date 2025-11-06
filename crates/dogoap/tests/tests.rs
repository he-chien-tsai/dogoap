//! Integration tests

use dogoap::{
    prelude::*,
    simple::{
        simple_action, simple_decrement_action, simple_increment_action, simple_multi_mutate_action,
    },
};

// One action that sets one field
#[test]
fn test_basic_bool_setting() {
    let start = LocalState::new().with_datum("is_hungry", true);

    let goal = Goal::new().with_req("is_hungry", Compare::equals(false));

    let eat_mutator = Mutator::set("is_hungry", false);

    let eat_consequence = Effect {
        action: "eat".to_string(),
        mutators: vec![eat_mutator.clone()],
        state: LocalState::new(),
        cost: 1,
    };

    let eat_action = Action {
        key: "eat".to_string(),
        preconditions: vec![],
        effects: vec![eat_consequence],
    };

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0)
        .collect::<Vec<_>>();
    assert_eq!(1, plan.len());

    let cons = plan.first().unwrap();
    assert_eq!(1, cons.mutators.len());
    assert_eq!(eat_mutator, cons.mutators.first().unwrap().clone());

    let expected_state = LocalState::new().with_datum("is_hungry", false);
    assert_eq!(expected_state, cons.state);
}

// The state is already what we need!
#[test]
fn test_no_actions_needed() {
    let start = LocalState::new().with_datum("is_hungry", false);

    let goal = Goal::new().with_req("is_hungry", Compare::equals(false));

    let eat_mutator = Mutator::set("is_hungry", false);

    let eat_consequence = Effect {
        action: "eat".to_string(),
        mutators: vec![eat_mutator.clone()],
        state: LocalState::new(),
        cost: 1,
    };

    let eat_action = Action {
        key: "eat".to_string(),
        preconditions: vec![],
        effects: vec![eat_consequence],
    };

    let actions: Vec<Action> = vec![eat_action];

    let (plan, plan_cost) = make_plan(&start, &actions[..], &goal).unwrap();
    assert_eq!(1, plan.len());
    assert_eq!(0, plan_cost);

    let expected_state = LocalState::new().with_datum("is_hungry", false);
    assert_eq!(expected_state, plan.first().unwrap().state().clone());
}

// Shorthand for one action that sets one field
#[test]
fn test_simple_action() {
    let start = LocalState::new().with_datum("is_hungry", true);
    let expected_state = LocalState::new().with_datum("is_hungry", false);

    let goal = Goal::new().with_req("is_hungry", Compare::equals(false));

    let eat_action = simple_action("eat", "is_hungry", false);
    let eat_mutator = Mutator::set("is_hungry", false);

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0)
        .collect::<Vec<_>>();
    assert_eq!(1, plan.len());

    let cons = plan.first().unwrap().clone();
    assert_eq!(1, cons.mutators.len());
    assert_eq!(eat_mutator, cons.mutators.first().unwrap().clone());
    assert_eq!(expected_state, cons.state);
}

// State with two fields + two actions each mutating their fields
#[test]
fn test_two_bools() {
    let start = LocalState::new()
        .with_datum("is_hungry", true)
        .with_datum("is_tired", true);

    let expected_state = LocalState::new()
        .with_datum("is_hungry", false)
        .with_datum("is_tired", false);

    let goal = Goal::new()
        .with_req("is_hungry", Compare::equals(false))
        .with_req("is_tired", Compare::equals(false));

    let eat_action = simple_action("eat", "is_hungry", false);
    let sleep_action = simple_action("sleep", "is_tired", false);

    let actions: Vec<Action> = vec![eat_action, sleep_action];

    let plan = make_plan(&start, &actions[..], &goal).unwrap();

    let cons = get_effects_from_plan(plan.0).collect::<Vec<_>>();
    assert_eq!(2, cons.len());

    let first_cons = cons.first().unwrap();
    assert_eq!(1, first_cons.mutators.len());

    let second_cons = cons.get(1).unwrap();
    assert_eq!(1, second_cons.mutators.len());

    assert_eq!(expected_state, second_cons.state);
}

// State with two fields + two actions each mutating their fields
#[test]
fn test_four_bools() {
    let start = LocalState::new()
        .with_datum("is_hungry", true)
        .with_datum("is_tired", true)
        .with_datum("is_fit", false)
        .with_datum("is_dirty", false);

    // We want to be fit, but not hungry, tired or dirty
    let goal = Goal::new()
        .with_req("is_hungry", Compare::equals(false))
        .with_req("is_tired", Compare::equals(false))
        .with_req("is_fit", Compare::equals(true))
        .with_req("is_dirty", Compare::equals(false));

    // Actions
    // eat => no longer hungry
    // sleep => no longer tired but now hungry
    // train => now fit but now dirty and tired
    // shower => no longer dirty but now tired
    let eat_action = simple_action("eat", "is_hungry", false);

    let sleep_action =
        simple_multi_mutate_action("sleep", vec![("is_tired", false), ("is_hungry", true)]);
    let train_action = simple_multi_mutate_action(
        "train",
        vec![("is_tired", true), ("is_dirty", true), ("is_fit", true)],
    );
    let shower_action =
        simple_multi_mutate_action("shower", vec![("is_tired", true), ("is_dirty", false)]);

    let actions: Vec<Action> = vec![eat_action, sleep_action, train_action, shower_action];

    let plan = make_plan(&start, &actions[..], &goal).unwrap();

    let cons = get_effects_from_plan(plan.0).collect::<Vec<_>>();
    assert_eq!(4, cons.len());

    let first_cons = cons.first().unwrap();
    assert_eq!(3, first_cons.mutators.len());

    let second_cons = cons.get(1).unwrap();
    assert_eq!(2, second_cons.mutators.len());

    let third_cons = cons.get(2).unwrap();
    assert_eq!(2, third_cons.mutators.len());

    let fourth_cons = cons.get(3).unwrap();
    assert_eq!(1, fourth_cons.mutators.len());

    let expected_state = LocalState::new()
        .with_datum("is_hungry", false)
        .with_datum("is_tired", false)
        .with_datum("is_fit", true)
        .with_datum("is_dirty", false);
    assert_eq!(expected_state, cons.last().unwrap().state);
}

enum TestLocation {
    House,
    Outside,
    Market,
    RamenShop,
}

#[test]
fn test_enums() {
    let loc_house = TestLocation::House as usize;
    let loc_outside = TestLocation::Outside as usize;
    let loc_market = TestLocation::Market as usize;
    let loc_ramen = TestLocation::RamenShop as usize;

    let start = LocalState::new().with_datum("at_location", loc_house);

    let expected_state = LocalState::new().with_datum("at_location", loc_ramen);

    let goal = Goal::new().with_req("at_location", Compare::equals(loc_ramen));

    let go_outside_action = simple_action("go_outside", "at_location", loc_outside)
        .with_precondition(("at_location", Compare::equals(loc_house)));

    let go_to_market_action = simple_action("go_to_market", "at_location", loc_market)
        .with_precondition(("at_location", Compare::equals(loc_outside)));

    let go_to_ramen_action = simple_action("go_to_ramen", "at_location", loc_ramen)
        .with_precondition(("at_location", Compare::equals(loc_market)));

    let actions: Vec<Action> = vec![go_outside_action, go_to_market_action, go_to_ramen_action];

    let plan = make_plan(&start, &actions[..], &goal);
    let effects = get_effects_from_plan(plan.unwrap().0).collect::<Vec<_>>();

    assert_eq!(3, effects.len());

    let cons = effects.first().unwrap();
    assert_eq!(1, cons.mutators.len());

    let cons = effects.get(1).unwrap();
    assert_eq!(1, cons.mutators.len());

    let cons = effects.get(2).unwrap();
    assert_eq!(1, cons.mutators.len());

    // Take only the last one
    assert_eq!(expected_state, cons.state);
}

// // eat action can only be done with not tired
#[test]
fn test_preconditions() {
    let start = LocalState::new()
        .with_datum("is_hungry", true)
        .with_datum("is_tired", true);

    let expected_state = LocalState::new()
        .with_datum("is_hungry", false)
        .with_datum("is_tired", false);

    let goal = Goal::new()
        .with_req("is_hungry", Compare::equals(false))
        .with_req("is_tired", Compare::equals(false));

    let eat_action =
        simple_multi_mutate_action("eat", vec![("is_hungry", false), ("is_tired", true)])
            .with_precondition(("is_tired", Compare::equals(false)));

    let sleep_action = simple_action("sleep", "is_tired", false);

    let actions: Vec<Action> = vec![eat_action, sleep_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0)
        .collect::<Vec<_>>();
    assert_eq!(3, plan.len());

    let first_cons = plan.first().unwrap();
    assert_eq!(1, first_cons.mutators.len());

    let second_cons = plan.get(1).unwrap();
    assert_eq!(2, second_cons.mutators.len());

    let third_cons = plan.get(2).unwrap();
    assert_eq!(1, third_cons.mutators.len());

    assert_eq!(
        expected_state, third_cons.state,
        "Final state wasn't what we expected"
    );
}

// We can use ints too!
#[test]
fn test_int_increment() {
    let start = LocalState::new().with_datum("energy", 50_i64);
    let expected_state = LocalState::new().with_datum("energy", 100_i64);

    let goal = Goal::new().with_req("energy", Compare::equals(100_i64));

    let eat_action = simple_increment_action("eat", "energy", 10_i64);
    let eat_mutator = Mutator::increment("energy", 10_i64);

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0)
        .collect::<Vec<_>>();
    assert_eq!(5, plan.len());

    for cons in &plan {
        assert_eq!(1, cons.mutators.len());
        assert_eq!(eat_mutator, cons.mutators.first().unwrap().clone());
    }

    assert_eq!(expected_state, plan.last().unwrap().state);
}

#[test]
fn test_int_decrement() {
    let start = LocalState::new().with_datum("hunger", 80_i64);
    let expected_state = LocalState::new().with_datum("hunger", 10_i64);

    let goal = Goal::new().with_req("hunger", Compare::equals(10_i64));

    let eat_action = simple_decrement_action("eat", "hunger", 10_i64);
    let eat_mutator = Mutator::decrement("hunger", 10_i64);

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0)
        .collect::<Vec<_>>();
    assert_eq!(7, plan.len());

    for cons in &plan {
        assert_eq!(1, cons.mutators.len());
        assert_eq!(eat_mutator, cons.mutators.first().unwrap().clone());
    }

    assert_eq!(expected_state, plan.last().unwrap().state);
}

#[test]
fn test_float_increment() {
    let start = LocalState::new().with_datum("energy", 50.0_f64);
    let expected_state = LocalState::new().with_datum("energy", 100.0_f64);

    let goal = Goal::new().with_req("energy", Compare::equals(100.0_f64));

    let eat_action = simple_increment_action("eat", "energy", 10.0_f64);
    let eat_mutator = Mutator::increment("energy", 10.0_f64);

    let actions: Vec<Action> = vec![eat_action];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0)
        .collect::<Vec<_>>();
    assert_eq!(5, plan.len());

    for cons in &plan {
        assert_eq!(1, cons.mutators.len());
        assert_eq!(eat_mutator, cons.mutators.first().unwrap().clone());
    }

    assert_eq!(expected_state, plan.last().unwrap().state);
}

// GreaterThanEquals can be useful sometimes too!
#[test]
fn test_greater_than_equals() {
    let start = LocalState::new().with_datum("energy", 0_i64);
    let expected_state = LocalState::new().with_datum("energy", 54_i64);

    let goal = Goal::new().with_req("energy", Compare::greater_than_equals(50_i64));

    let eat_action = simple_increment_action("eat", "energy", 6_i64);

    let actions: Vec<Action> = vec![eat_action];

    let plan = make_plan(&start, &actions[..], &goal).unwrap();
    let effects = get_effects_from_plan(plan.0.clone()).collect::<Vec<_>>();

    assert_eq!(9, effects.len());

    for cons in &effects {
        assert_eq!(1, cons.mutators.len());
        assert_eq!(
            Mutator::increment("energy", 6_i64),
            cons.mutators.first().unwrap().clone()
        );
    }

    assert_eq!(expected_state, effects.last().unwrap().state);
}

#[test]
fn test_long_plan() {
    let start = LocalState::new()
        .with_datum("energy", 30_i64)
        .with_datum("hunger", 70_i64)
        .with_datum("gold", 0_i64);

    let expected_state = LocalState::new()
        .with_datum("energy", 50_i64)
        .with_datum("hunger", 50_i64)
        .with_datum("gold", 10_i64);

    let goal = Goal::new().with_req("gold", Compare::equals(10_i64));

    let sleep_action = simple_increment_action("sleep", "energy", 1_i64);

    let eat_action = simple_decrement_action("eat", "hunger", 1_i64)
        .with_precondition(("energy", Compare::greater_than_equals(50_i64)));

    let rob_people = simple_increment_action("rob", "gold", 1_i64)
        .with_effect(Effect {
            action: "rob".to_string(),
            mutators: vec![
                Mutator::decrement("energy", 20_i64),
                Mutator::increment("hunger", 20_i64),
            ],
            state: LocalState::default(),
            cost: 1,
        })
        .with_precondition(("hunger", Compare::less_than_equals(50_i64)))
        .with_precondition(("energy", Compare::greater_than_equals(50_i64)));
    let actions: Vec<Action> = vec![sleep_action, eat_action, rob_people];

    let plan = get_effects_from_plan(make_plan(&start, &actions[..], &goal).unwrap().0)
        .collect::<Vec<_>>();
    println!("b");

    assert_eq!(50, plan.len());

    assert_eq!(expected_state, plan.last().unwrap().state);
}

#[test]
fn test_prefer_lower_cost_plan() {
    // Planner should prefer cheaper plans based on cost
    //
    // Cheap action adds 1 gold and costs 1
    // Expensive action adds 3 gold and costs 5
    //
    // Planner should only use cheap action 10 times instead of using expensive action as
    // it'll be cheaper
    let start = LocalState::new().with_datum("gold", 0_i64);
    let expected_state = LocalState::new().with_datum("gold", 10_i64);

    let goal = Goal::new().with_req("gold", Compare::equals(10_i64));

    let cheap_action = Action::new("cheap_action")
        .with_mutator(Mutator::increment("gold", 1_i64))
        .set_cost(1); // Cost/gold is lower than expensive_action

    let expensive_action = Action::new("expensive_action")
        .with_mutator(Mutator::increment("gold", 3_i64))
        .set_cost(4); // Cost/gold is higher than cheap_action

    let actions = [cheap_action, expensive_action];

    let plan = make_plan(&start, &actions[..], &goal).unwrap();
    let effects = get_effects_from_plan(plan.0.clone()).collect::<Vec<_>>();

    println!("Found plan:");
    println!("{plan:#?}");

    assert_eq!(10, effects.len());
    assert_eq!(expected_state, effects.last().unwrap().state);
}
