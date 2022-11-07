/// Configuration for the RPC plugin
#[derive(Debug, Copy, Clone)]
pub struct RPCConfig {
    /// The Discord application ID
    pub app_id: u64,
    /// Whether to show the current time in the activity
    pub show_time: bool,
}

impl Default for RPCConfig {
    fn default() -> Self {
        Self {
            app_id: 425407036495495169,
            show_time: true,
        }
    }
}

// TODO: Add guide on how to get `app_id`

/// The main RPC plugin
///
/// # Arguments
///
/// * `config` - The configuration for the plugin. Vital field is `app_id`, as the Discord interactions cannot work without it.
pub struct RPCPlugin {
    /// The Discord config used by the plugin
    pub config: RPCConfig,
}

impl RPCPlugin {
    /// Create a new plugin instance from config values
    pub fn new(app_id: u64, show_time: bool) -> Self {
        Self {
            config: RPCConfig { app_id, show_time },
        }
    }

    /// Create a new plugin instance from the given config
    ///
    /// Alias of [`RPCPlugin::from`]
    pub fn from_config(config: RPCConfig) -> Self {
        Self::from(config)
    }
}

impl From<RPCConfig> for RPCPlugin {
    fn from(config: RPCConfig) -> Self {
        Self { config }
    }
}
