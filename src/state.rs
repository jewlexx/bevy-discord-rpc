use std::{collections::VecDeque, sync::Arc};

use bevy::prelude::Resource;
use discord_presence::{
    models::{
        Activity, ActivityAssets, ActivityButton, ActivityParty, ActivitySecrets,
        ActivityTimestamps,
    },
    Event,
};
use parking_lot::Mutex;

#[derive(Debug, Default, Clone)]
pub struct Events(pub(crate) VecDeque<Event>);

pub trait EventHandler {
    /// Return and remove the earliest event in the queue
    ///
    /// If it returns `None`, then the queue is empty
    fn respond(&mut self) -> Option<Event>;

    /// Return and remove the most recent event in the queue
    ///
    /// If it returns `None`, then the queue is empty
    fn respond_latest(&mut self) -> Option<Event>;

    /// Check if the given event has fired, removing it from the queue if so
    fn respond_specific(&mut self, event: Event) -> bool;

    /// Ignore all events, removing them
    fn clear(&mut self);
}

impl EventHandler for Events {
    fn respond(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    fn respond_latest(&mut self) -> Option<Event> {
        self.0.pop_back()
    }

    fn respond_specific(&mut self, event: Event) -> bool {
        if let Some(index) = self.0.iter().position(|e| *e == event) {
            self.0.remove(index);
            true
        } else {
            false
        }
    }

    fn clear(&mut self) {
        self.0.clear()
    }
}

impl EventHandler for Arc<Mutex<Events>> {
    fn respond(&mut self) -> Option<Event> {
        self.lock().respond()
    }

    fn respond_latest(&mut self) -> Option<Event> {
        self.lock().respond_latest()
    }

    fn respond_specific(&mut self, event: Event) -> bool {
        self.lock().respond_specific(event)
    }

    fn clear(&mut self) {
        self.lock().clear()
    }
}

/// The state that holds the Discord activity
#[derive(Debug, Resource, Default, Clone)]
pub struct ActivityState {
    /// The player's current party status
    pub state: Option<String>,
    /// What the player is currently doing
    pub details: Option<String>,
    /// Whether this activity is an instanced context, like a match
    pub instance: Option<bool>,
    /// Helps create elapsed/remaining timestamps on a player's profile
    pub timestamps: Option<ActivityTimestamps>,
    /// Assets to display on the player's profile
    pub assets: Option<ActivityAssets>,
    /// Information about the player's party. NOTE: Joining a party is not currently supported
    pub party: Option<ActivityParty>,
    /// Secret passwords for joining and spectating the player's game. NOTE: Joining a party is not currently supported
    pub secrets: Option<ActivitySecrets>,
    /// The events that have fired for this activity
    pub events: Arc<Mutex<Events>>,
    /// The buttons to be displayed on the player's profile
    pub buttons: Vec<ActivityButton>,
}

impl From<ActivityState> for Activity {
    /// Converts the ActivityState into a Discord Presence
    fn from(state: ActivityState) -> Self {
        Activity {
            state: state.state,
            assets: state.assets,
            details: state.details,
            party: state.party,
            secrets: state.secrets,
            timestamps: state.timestamps,
            instance: state.instance,
            buttons: state.buttons,
        }
    }
}
