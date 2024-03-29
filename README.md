# Bevy Discord Presence Plugin

[![crates.io](https://img.shields.io/crates/v/bevy-discord-presence)](https://crates.io/crates/bevy-discord-presence)
[![crates.io](https://img.shields.io/crates/d/bevy-discord-presence)](https://crates.io/crates/bevy-discord-presence)
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/book/plugin-development/#main-branch-tracking)[![docs.rs](https://img.shields.io/docsrs/bevy-discord-presence/latest)](https://docs.rs/bevy-discord-presence)

> [!WARNING]
> This project has been archived, as I do not have the time or motivation to maintain it currently. The currently released version should continue to work for the foreseeable future, but will inevitably be broken by a future Bevy update.
> The underlying [discord-presence](https://github.com/jewlexx/discord-presence) library is still under active development, you are welcome to use this repo as a reference for integrating it into your own Bevy app, or fork it, but I will not continue updating this plugin for the ever changing Bevy ecosystem.

A simplistic bevy plugin for discord presence integration within the bevy game engine

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bevy-discord-presence = "0.5"
```

or run:

```shell
cargo add bevy-discord-presence
```

## Example

```rust
use bevy::prelude::*;

use bevy_discord_presence::config::{RPCConfig, RPCPlugin};

fn main() {
    println!("hello world!");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RPCPlugin(RPCConfig {
        app_id: 965125975941709834,
        show_time: true,
    }));

    app.run();
}
```

> More examples can be found in the examples directory.

## Changelog

See [CHANGELOG.md](CHANGELOG.md)

## Contributions

See [CONTRIBUTING.md](/CONTRIBUTING.md)
