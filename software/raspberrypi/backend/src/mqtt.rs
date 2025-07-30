pub mod configuration {
    use secrecy::SecretString;
    use serde::Deserialize;

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct Mqtt {
        pub enable: bool,
        pub url: String,
        pub port: u16,
        pub username: String,
        pub password: SecretString,
    }
}

use std::time::Duration;

use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::{task, time};

pub async fn mqtt(configuration: &configuration::Mqtt, _state: bool) {
    if !configuration.enable {
        return;
    }

    let mut mqttoptions = MqttOptions::new("rumqttc-async", &configuration.url, configuration.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(async move {
        for i in 0..10 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .await
                .unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {notification:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::{configuration, mqtt};

    #[tokio::test]
    async fn test_mqtt_disabled() {
        let mqtt_configuration = configuration::Mqtt::default();
        mqtt(&mqtt_configuration, false).await;
        // // TODO: Check if mqtt is disabled
    }

    #[tokio::test]
    async fn test_mqtt_enabled() {
        let mqtt_configuration = configuration::Mqtt {
            enable: true,
            url: "test.mosquitto.org".to_string(),
            port: 1883,
            username: "username".to_string(),
            password: "password".into(),
        };

        mqtt(&mqtt_configuration, false).await;
        // // TODO: Check if mqtt is enabled
    }

    #[tokio::test]
    async fn test_mqtt() {
        let mqtt_configuration = configuration::Mqtt {
            enable: true,
            url: "test.mosquitto.org".to_string(),
            port: 1883,
            username: "username".to_string(),
            password: "password".into(),
        };

        mqtt(&mqtt_configuration, false).await;
    }
}
