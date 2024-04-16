use secrecy::ExposeSecret;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info};

use crate::configuration::SpaceAPI;

pub async fn spaceapi(configuration: &SpaceAPI, mut receiver: Receiver<bool>) {
    while let Some(state) = receiver.recv().await {
        info!("SpaceAPI {state:?}");
        if configuration.enable {
            let status = if state {
                String::from("open")
            } else {
                String::from("closed")
            };
            info!("Set SpaceAPI status");
            match reqwest::Client::new()
                .put(format!("{}?status={status}", configuration.url))
                .basic_auth(
                    &configuration.username,
                    Some(&configuration.password.expose_secret()),
                )
                .send()
                .await
            {
                Ok(_) => {
                    info!("Successfully set SpaceAPI status");
                }
                Err(e) => {
                    error!("Failed to set SpaceAPI status: {e:?}");
                }
            };
        }
    }
}
