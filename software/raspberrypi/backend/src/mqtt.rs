pub mod configuration {
    use secrecy::SecretString;
    use serde::Deserialize;

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct Mqtt {
        pub enable: bool,
        pub host: String,
        pub port: u16,
        pub username: String,
        pub password: SecretString,
    }
}

use std::time::{Duration, SystemTime};

use retry_policies::{policies::ExponentialBackoff, Jitter, RetryDecision, RetryPolicy};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use secrecy::ExposeSecret;
use tokio::{
    sync::watch::{channel, Sender},
    task,
};
use tracing::{debug, error};

use crate::notifyer::Notifier;

pub struct MqttNotifier {
    configuration: configuration::Mqtt,
    sender: Sender<bool>,
}

impl Notifier for MqttNotifier {
    type Configuration = configuration::Mqtt;

    fn new_with_configuration(configuration: &Self::Configuration) -> Self {
        let (sender, mut receiver) = channel(false);

        let mut mqttoptions =
            MqttOptions::new("nerdschloss", &configuration.host, configuration.port);
        mqttoptions.set_credentials(
            &configuration.username,
            configuration.password.expose_secret(),
        );

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        task::Builder::new()
            .name("mqtt_poll")
            .spawn(async move {
                let backoff = ExponentialBackoff::builder()
                    .retry_bounds(Duration::from_secs(1), Duration::from_secs(60))
                    .jitter(Jitter::Bounded)
                    .base(2)
                    .build_with_total_retry_duration(Duration::from_secs(60 * 60 * 24 * 7 * 365));
                let start = SystemTime::now();
                let mut attempts = 0u32;

                loop {
                    loop {
                        match eventloop.poll().await {
                            Ok(v) => {
                                if 0 < attempts {
                                    attempts = 0;
                                }
                                debug!("Event= {v:?}")
                            }
                            Err(e) => {
                                error!("{e:?}");
                                break;
                            }
                        }
                    }

                    match backoff.should_retry(start, attempts) {
                        RetryDecision::Retry { execute_after } => {
                            if let Ok(wait) = execute_after.duration_since(SystemTime::now()) {
                                if !wait.is_zero() {
                                    tokio::time::sleep(wait).await;
                                }
                            }
                            attempts += 1;
                        }
                        RetryDecision::DoNotRetry => break,
                    }
                }
            })
            .unwrap();

        task::Builder::new()
            .name("mqtt_publish")
            .spawn(async move {
                loop {
                    if receiver.changed().await.is_err() {
                        break;
                    }
                    let state = *receiver.borrow_and_update();
                    let state = if state {
                        String::from("open")
                    } else {
                        String::from("closed")
                    };

                    match client
                        .publish(
                            "gleis0/nerdschloss/lockswitch",
                            QoS::ExactlyOnce,
                            true,
                            state.as_bytes(),
                        )
                        .await
                    {
                        Ok(v) => {
                            // TODO: Send notify state
                            debug!("Published = {v:?}")
                        }
                        Err(e) => {
                            // TODO: Send notify state
                            error!("Error = {e:?}")
                        }
                    }
                }
            })
            .unwrap();

        Self {
            configuration: configuration.clone(),
            sender,
        }
    }

    fn notify(&self, state: bool) {
        if !self.configuration.enable {
            return;
        }

        match self.sender.send(state) {
            Ok(v) => debug!("Sent = {v:?}"),
            Err(e) => error!("Error = {e:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use rumqttd::{Broker, Notification};

    use crate::notifyer::Notifier;

    use super::{configuration, MqttNotifier};

    #[tokio::test]
    async fn test_mqtt_disabled() {
        thread::spawn(move || {
            // As examples are compiled as seperate binary so this config is current path dependent. Run it
            // from root of this crate
            let config = config::Config::builder()
                .add_source(config::File::with_name("../rumqttd.toml"))
                .build()
                .unwrap();
            let config = config.try_deserialize().unwrap();

            let mut broker = Broker::new(config);

            let (mut link_tx, mut link_rx) = broker.link("tester").unwrap();

            thread::spawn(move || {
                broker.start().unwrap();
            });

            link_tx.subscribe("gleis0/nerdschloss/lockswitch").unwrap();

            let mut count = 0;
            loop {
                let notification = match link_rx.recv().unwrap() {
                    Some(v) => v,
                    None => continue,
                };

                match notification {
                    Notification::Forward(forward) => {
                        count += 1;
                        println!(
                            "Topic = {:?}, Count = {}, Payload = {} bytes",
                            forward.publish.topic,
                            count,
                            forward.publish.payload.len()
                        );
                    }
                    v => {
                        println!("{v:?}");
                    }
                }
            }
        });

        let mqtt_configuration = configuration::Mqtt::default();
        let mqtt_notifier = MqttNotifier::new_with_configuration(&mqtt_configuration);
        mqtt_notifier.notify(true);
    }

    #[tokio::test]
    async fn test_mqtt_enabled() {
        let mqtt_configuration = configuration::Mqtt {
            enable: true,
            host: "localhost".to_string(),
            port: 1883,
            username: "nerdschloss".to_string(),
            password: "nerdschloss".into(),
        };
        let mqtt_notifier = MqttNotifier::new_with_configuration(&mqtt_configuration);
        mqtt_notifier.notify(true);
    }
}
