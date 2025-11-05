#[macro_export]
macro_rules! create_planner {
    ({
        actions: [$(($action_type:ty, $action:expr)),* $(,)?],
        state: [$($state:expr),* $(,)?],
        goals: [$($goal:expr),* $(,)?],
    }) => {{
        use bevy_platform::collections::HashMap;
        use bevy_dogoap::prelude::InserterComponent;
        let actions_map: HashMap<String, (Action, Box<dyn InserterComponent>)> = HashMap::from([
            $(
                (
                    <$action_type>::key(),
                    (
                        $action.clone(),
                        Box::new(<$action_type>::default()) as Box<dyn InserterComponent>,
                    ),
                )
            ),*
        ]);

        let components = Vec::from([
            $(
                Box::new($state.clone()) as Box<dyn DatumComponent>,
            )*
        ]);

        let planner = Planner::new(components, vec![$($goal.clone()),*], actions_map);

        let component_entities = ($($state.clone()),*);

        (planner, component_entities)
    }};
}

#[macro_export]
macro_rules! register_components {
    ($app:ident, [$($comp:ty),*]) => {
        $(
            $app.register_component_as::<dyn DatumComponent, $comp>();
        )*
    };
}

#[macro_export]
macro_rules! register_actions {
    ($app:ident, [$($comp:ty),*]) => {
        $(
            $app.register_component_as::<dyn ActionComponent, $comp>();
        )*
    };
}
