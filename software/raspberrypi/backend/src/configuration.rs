use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

use hardware::{DoorSwitchConfiguration, LockMotorConfiguration, LockSwitchConfiguration};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Server {
    pub ipaddress: std::net::IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceAPI {
    pub enable: bool,
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub server: Server,

    pub spaceapi: SpaceAPI,

    // hardware
    pub lockmotor: LockMotorConfiguration,
    pub lockswitch: LockSwitchConfiguration,
    pub doorswitch: DoorSwitchConfiguration,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        const APP_NAME: &str = "NERDSCHLOSS";

        let config_path = ProjectDirs::from("de", "MathPeterIT", &APP_NAME.to_lowercase())
            .expect("User configuration directory not found");
        let user_config_path = config_path
            .config_dir()
            .to_str()
            .expect("User configuration directory not found");

        // Parse settings from configuration files and environment
        let c = Config::builder()
            .add_source(
                File::with_name(&format!("/etc/{}", APP_NAME.to_lowercase())).required(false),
            )
            .add_source(File::with_name(user_config_path).required(false))
            .add_source(File::with_name(&APP_NAME.to_lowercase()).required(false))
            .add_source(
                Environment::with_prefix(APP_NAME)
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()
            .expect("grrr");

        c.try_deserialize()
    }
}