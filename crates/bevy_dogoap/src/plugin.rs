use bevy_ecs::intern::Interned;
use bevy_ecs::schedule::ScheduleLabel;

use crate::planner;
use crate::prelude::*;

/// Setups the [`Planner`](planner::Planner) systems in [`DogoapSystems`] to run a specified schedule.
/// Uses [`FixedPreUpdate`] by default.
pub struct DogoapPlugin {
    schedule: Interned<dyn ScheduleLabel>,
}

impl Default for DogoapPlugin {
    fn default() -> Self {
        Self {
            schedule: FixedPreUpdate.intern(),
        }
    }
}

impl DogoapPlugin {
    /// Sets the schedule for running the plugin. Defaults to
    /// [`FixedPreUpdate`].
    #[must_use]
    pub fn in_schedule(mut self, schedule: impl ScheduleLabel) -> Self {
        self.schedule = schedule.intern();
        self
    }
}
impl Plugin for DogoapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            self.schedule,
            (
                planner::update_planner_local_state,
                planner::create_planner_tasks,
                planner::handle_planner_tasks,
            )
                .chain()
                .in_set(DogoapSystems::RunPlanner),
        )
        .configure_sets(self.schedule, DogoapSystems::RunPlanner);
    }
}

/// System set for [`DogoapPlugin`]
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DogoapSystems {
    /// Updates the planner
    RunPlanner,
}
