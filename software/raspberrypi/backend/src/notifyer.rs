use tokio::sync::mpsc::Receiver;

use crate::{configuration::ConfigurationRef, mattermost::mattermost, mqtt, spaceapi::spaceapi};

pub async fn notify(configuration: ConfigurationRef, mut receiver: Receiver<bool>) {
    while let Some(state) = receiver.recv().await {
        let mqtt = mqtt::mqtt(&configuration.mqtt, state);
        let mattermost = mattermost(&configuration.mattermost, state);
        let spaceapi = spaceapi(&configuration.spaceapi, state);
        tokio::join!(mqtt, mattermost, spaceapi);
    }
}
