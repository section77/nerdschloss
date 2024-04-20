use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
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

            let retry_policy = ExponentialBackoff::builder().build_with_max_retries(5);
            let client = ClientBuilder::new(reqwest::Client::new())
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build();

            match client
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
