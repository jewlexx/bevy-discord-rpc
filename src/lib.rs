#![warn(missing_docs)]

//! A Bevy plugin that allows the developer to interact with the Discord Presence API with ease
//!
//! This plugin is a Bevy wrapper around the [Discord Presence](https://docs.rs/crate/discord-presence) crate which in turn is a wrapper around the [Discord Presence API](https://discordapp.com/developers/docs/game-sdk/discord-presence).
//! # Examples
//!
//! ```rust no_run
//! use bevy::prelude::*;
//! use bevy_discord_presence::{state::ActivityState, config::{RPCConfig, RPCPlugin}};
//!
//! fn main() {
//!     println!("hello world!");
//!     let mut app = App::new();
//!     app.add_plugins(DefaultPlugins);
//!     app.add_plugin(RPCPlugin(RPCConfig {
//!         app_id: 425407036495495169,
//!         show_time: true,
//!     }));
//!     app.add_system(update_presence);
//!
//!     app.run();
//! }
//!
//! fn update_presence(mut state: ResMut<ActivityState>) {
//!     state.details = Some("Hello World".to_string());
//! }
//! ```

use std::time::{SystemTime, UNIX_EPOCH};

use bevy::{log::prelude::*, prelude::*};
use discord_presence::{models::ActivityTimestamps, Client, Event};

/// The Discord configuration
mod config;
/// The state that holds the Discord activity
mod state;

pub use config::{RPCConfig, RPCPlugin};
pub use state::ActivityState;

/// Implements the Bevy plugin trait
impl Plugin for RPCPlugin {
    fn build(&self, app: &mut App) {
        let client_config = self.config;

        app.add_startup_system(startup_client);
        app.add_system(check_activity_changed);
        debug!("Added systems");

        app.insert_resource::<RPCConfig>(client_config);

        app.init_resource::<ActivityState>();
        app.insert_resource::<Client>(Client::new(client_config.app_id));

        debug!("Initialized resources");
    }

    fn name(&self) -> &str {
        "Discord Presence"
    }
}

/// Initializes the client and starts it running
fn startup_client(
    mut activity: ResMut<ActivityState>,
    mut client: ResMut<Client>,
    config: Res<RPCConfig>,
) {
    use strum::IntoEnumIterator;

    if config.show_time {
        activity.timestamps = Some(ActivityTimestamps {
            start: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time has gone backwards")
                    .as_secs(),
            ),
            end: None,
        });
    }

    for event in Event::iter() {
        client.on_event(event, {
            let events = activity.events.clone();

            move |_| {
                events.lock().add(event);
                debug!("Added event: {:?}", event);
            }
        });
    }

    client.start();
    debug!("Client has started");
}

/// Runs whenever the activity has been changed, and at startup
fn check_activity_changed(activity: Res<ActivityState>, mut client: ResMut<Client>) {
    if activity.is_changed() {
        let res = client.set_activity(|_| activity.clone().into());

        if let Err(why) = res {
            error!("Failed to set presence: {}", why);
        }
    }
}
