//! Traits used by `bevy_dogoap`. The types here are typically implemented via `#[derive(...)]`.

use std::fmt;

use crate::prelude::*;
use dogoap::prelude::*;

/// A [`Component`] that can insert/remove itself to/from an Entity
/// Used for adding/removing current [`Action`] our planner tells us to perform
#[reflect_trait]
pub trait InserterComponent: Send + Sync + 'static {
    /// Calls [`Commands::try_insert`] with the underlying component
    fn try_insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);

    /// Calls [`Commands::try_remove`] with the underlying component
    fn try_remove(&self, commands: &mut Commands, entity_to_remove_from: Entity);
}

impl<T> InserterComponent for T
where
    T: Component + Clone + Send + Sync + 'static,
{
    fn try_insert(&self, commands: &mut Commands, entity_to_insert_to: Entity) {
        commands
            .entity(entity_to_insert_to)
            .try_insert(T::clone(self));
    }
    fn try_remove(&self, commands: &mut Commands, entity_to_remove_from: Entity) {
        commands.entity(entity_to_remove_from).try_remove::<T>();
    }
}

impl fmt::Debug for dyn InserterComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MarkerComponent [DebugNotImplemented!]",)
    }
}

/// A [`Component` ] that can be used as a [`Mutator`] and [`Precondition`]
///
/// Example:
///
/// ```ignore
/// #[derive(DatumComponent)]
/// struct Hunger(f32);
///
/// // Used as a Mutator:
/// assert_eq!(
///     Hunger::increase(1.0),
///     Mutator::Increment("hunger".to_string(), Datum::F64(2.0))
/// );
///
/// // Used as a Precondition:
/// assert_eq!(
///     Hunger::is_less(10.0),
///     ("hunger".to_string(), Compare::LessThanEquals(Datum::F64(10.0)))
/// )
/// ```
#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait DatumComponent: Send + Sync {
    /// Gets the string identifier for the datum.
    fn field_key(&self) -> String;
    /// Gets the underlying datum.
    fn field_value(&self) -> Datum;
}

/// `ActionComponent` allows you to create Actions directly from your action struct
///
/// Can be derived with `#derive(ActionComponent)`
///
/// Example:
///
/// ```rust
/// # use bevy_dogoap::prelude::*;
/// #[derive(ActionComponent)]
/// struct DrinkAction;
///
/// // Used as a shorter way of creating a new Action with snake_case name
/// assert_eq!(
///     DrinkAction::action(),
///     Action::new("drink_action")
/// );
/// ```
///
/// Combined with [`DatumComponent`] to used as Mutator and Precondition
///
/// ```rust
/// # use bevy_dogoap::prelude::*;
/// # #[derive(ActionComponent)]
/// # struct DrinkAction;
///
/// #[derive(DatumComponent)]
/// struct Thirst(f64);
/// ```
#[bevy_trait_query::queryable]
#[reflect_trait]
pub trait ActionComponent: Send + Sync {
    /// Gets the action key but in `snake_case` ("`AtLocation`" becomes "`at_location`")
    fn key() -> String
    where
        Self: Sized;
    /// Creates a new [`Action`] with our `snake_case` key
    fn action() -> Action
    where
        Self: Sized;
    /// Returns the type name
    fn action_type_name(&self) -> &'static str;
}

/// Derivable trait for enums to be used as [`Datum`].
pub trait EnumDatum: Send + Sync {
    /// Gets the underlying enum value.
    fn datum(self) -> Datum;
}

/// Internal trait implemented by `#[derive(DatumComponent)]`
pub trait Precondition<T> {
    /// Returns the string representation of this type and a comparison for the concept of `==`.
    fn is(val: T) -> (String, Compare);

    /// Returns the string representation of this type and a comparison for the concept of `!=`.
    fn is_not(val: T) -> (String, Compare);

    /// Returns the string representation of this type and a comparison for the concept of `>`.
    fn is_more(val: T) -> (String, Compare);

    /// Returns the string representation of this type and a comparison for the concept of `<`.
    fn is_less(val: T) -> (String, Compare);
}

/// Internal trait implemented by `#[derive(DatumComponent)]` in order to mutate
pub trait MutatorTrait<T> {
    /// Returns a [`Mutator`] that sets the value of the component to the given value.
    fn set(val: T) -> Mutator;
    /// Returns a [`Mutator`] that increases the value of the component by the given value.
    fn increase(val: T) -> Mutator;
    /// Returns a [`Mutator`] that decreases the value of the component by the given value.
    fn decrease(val: T) -> Mutator;
}
