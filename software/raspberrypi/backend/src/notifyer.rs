use tokio::sync::mpsc::Receiver;

use crate::{configuration::ConfigurationRef, mattermost::mattermost, mqtt, spaceapi::spaceapi};

pub async fn notify(configuration: ConfigurationRef, mut notify_receiver: Receiver<bool>) {
    let mqtt_notifier = mqtt::MqttNotifier::new_with_configuration(&configuration.mqtt);

    while let Some(state) = notify_receiver.recv().await {
        let mattermost = mattermost(&configuration.mattermost, state);
        let spaceapi = spaceapi(&configuration.spaceapi, state);

        mqtt_notifier.notify(state);
        tokio::join!(mattermost, spaceapi);
    }
}

pub trait Notifier {
    type Configuration;

    #[must_use]
    fn new_with_configuration(configuration: &Self::Configuration) -> Self;
    fn notify(&self, state: bool);
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum NotifierState {
    Setting,
    Succeeded,
    Failed,
}
