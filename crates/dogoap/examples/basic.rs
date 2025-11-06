//! This example shows the most basic use of dogoap
//! It's a bit overly verbose (check examples/simple.rs for a "not as verbose" example)
//! but shows the data structures needed for the planner

use dogoap::prelude::*;

fn main() {
    let start = LocalState::new().with_datum("is_hungry", true);

    let goal = Goal::new().with_req("is_hungry", Compare::equals(false));

    let eat_action = Action {
        key: "eat".to_string(),
        preconditions: vec![],
        effects: vec![Effect {
            action: "eat".to_string(),
            mutators: vec![Mutator::set("is_hungry", false)],
            state: LocalState::new(),
            cost: 1,
        }],
    };

    let actions: Vec<Action> = vec![eat_action];

    let plan = make_plan(&start, &actions[..], &goal);

    println!("{}", format_plan(plan.unwrap()));

    println!();
    println!("[Everything went as expected!]");
}
