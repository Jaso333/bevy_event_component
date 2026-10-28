use std::marker::PhantomData;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

/// Holds the last event of a given type that occurred on the entity.
#[derive(Component, Clone)]
#[component(on_add, on_remove)]
pub struct EventComponent<E>
where
    E: EntityEvent + Clone,
{
    /// The event that ocurred. When paired with change detection, systems can consider this event "triggered" when changed.
    event: Option<E>,
}

impl<E> EventComponent<E>
where
    E: EntityEvent + Clone,
{
    /// Gets the contained event.
    pub fn event(&self) -> Option<&E> {
        self.event.as_ref()
    }

    /// Causes the observer to be updated.
    fn on_add(mut world: DeferredWorld, _: HookContext) {
        world.commands().run_system_cached(Self::update_observer);
    }

    /// Causes the observer to be updated.
    fn on_remove(mut world: DeferredWorld, _: HookContext) {
        world.commands().run_system_cached(Self::update_observer);
    }

    /// The observer for this type of component, forwards the event to the component.
    fn on_event(event: On<E>, mut query: Query<&mut EventComponent<E>>) {
        let Ok(mut component_event) = query.get_mut(event.event_target()) else {
            return;
        };

        component_event.event = Some(event.clone());
    }

    /// The system used to manage observers for the component types.
    fn update_observer(
        observer_query: Query<(), With<EventComponentObserver<E>>>,
        event_query: Query<(), With<EventComponent<E>>>,
        mut commands: Commands,
    ) where
        E: EntityEvent + Clone,
    {
        // spawn or despawn the observer depending on conditions
        if observer_query.is_empty() && !event_query.is_empty() {
            commands.spawn((
                Observer::new(Self::on_event),
                EventComponentObserver::<E>::default(),
            ));
        } else if !observer_query.is_empty() && event_query.is_empty() {
            commands.despawn_all::<With<EventComponentObserver<E>>>();
        }
    }
}

impl<E> Default for EventComponent<E>
where
    E: EntityEvent + Clone,
{
    fn default() -> Self {
        Self {
            event: Default::default(),
        }
    }
}

/// Required to make BSN happy.
impl<E> Unpin for EventComponent<E> where E: EntityEvent + Clone {}

/// Extends [`Ref`] to [`ComponentEvent`] allowing the system to determine when the event is triggered.
pub trait EventComponentRefExt<E> {
    /// Gets the event only if it has been triggered.
    fn triggered_event(&self) -> Option<&E>;
}

impl<E> EventComponentRefExt<E> for Ref<'_, EventComponent<E>>
where
    E: EntityEvent + Clone,
{
    fn triggered_event(&self) -> Option<&E> {
        // if it has changed and there is some event
        self.is_changed().then(|| self.event())?
    }
}

/// Provides less verbosity when used in query data.
pub type CEvent<'w, E> = Ref<'w, EventComponent<E>>;

/// Marks the observer used to update the component.
#[derive(Component)]
struct EventComponentObserver<E> {
    _marker: PhantomData<E>,
}

impl<E> Default for EventComponentObserver<E> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}
