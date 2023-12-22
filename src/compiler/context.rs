use std::any::Any;

use super::TimeZone;

use super::{state::RuntimeState, Target};

pub struct Context<'a> {
    target: &'a mut dyn Target,
    state: &'a mut RuntimeState,
    timezone: &'a TimeZone,
    dynamic_state: Option<&'a dyn Any>,
}

impl<'a> Context<'a> {
    /// Create a new [`Context`].
    pub fn new(
        target: &'a mut dyn Target,
        state: &'a mut RuntimeState,
        timezone: &'a TimeZone,
    ) -> Self {
        Self {
            target,
            state,
            timezone,
            dynamic_state: None,
        }
    }

    pub fn with_dynamic_state(mut self, state: &'a dyn Any) -> Self {
        self.dynamic_state = Some(state);
        self
    }

    pub fn dynamic_state(&self) -> Option<&'a dyn Any> {
        self.dynamic_state
    }

    /// Get a reference to the [`Target`].
    #[must_use]
    pub fn target(&self) -> &dyn Target {
        self.target
    }

    /// Get a mutable reference to the [`Target`].
    pub fn target_mut(&mut self) -> &mut dyn Target {
        self.target
    }

    /// Get a reference to the [`runtime state`](Runtime).
    #[must_use]
    pub fn state(&self) -> &RuntimeState {
        self.state
    }

    /// Get a mutable reference to the [`runtime state`](Runtime).
    pub fn state_mut(&mut self) -> &mut RuntimeState {
        self.state
    }

    /// Get a reference to the [`TimeZone`]
    #[must_use]
    pub fn timezone(&self) -> &TimeZone {
        self.timezone
    }
}
