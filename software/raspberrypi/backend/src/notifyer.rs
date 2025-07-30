use tokio::sync::mpsc::Receiver;

use crate::{configuration::ConfigurationRef, mattermost::mattermost, spaceapi::spaceapi};

// async fn mqtt(state: bool) {
//     info!("MQTT");

//     use rumqttc::{AsyncClient, MqttOptions, QoS};
//     use std::time::Duration;
//     // use tokio::{task, time};

//     let mut mqttoptions = MqttOptions::new(
//         "nerdschloss",
//         "mqtt://homeassistant.gleis0.section77.localdomain",
//         1883,
//     );
//     mqttoptions.set_keep_alive(Duration::from_secs(5));

//     let (client, _) = AsyncClient::new(mqttoptions, 10);
//     client
//         .subscribe("lock/state", QoS::AtMostOnce)
//         .await
//         .unwrap();

//     // task::spawn(async move {
//     //     for i in 0..10 {
//     client
//         .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![state.into()])
//         .await
//         .unwrap();
//     // time::sleep(Duration::from_millis(100)).await;
//     //     }
//     // });

//     // loop {
//     //     let notification = eventloop.poll().await.unwrap();
//     //     println!("Received = {:?}", notification);
//     // }
// }

pub async fn notify(configuration: ConfigurationRef, mut receiver: Receiver<bool>) {
    while let Some(state) = receiver.recv().await {
        // let mqtt = mqtt(state);
        let mattermost = mattermost(&configuration.mattermost, state);
        let spaceapi = spaceapi(&configuration.spaceapi, state);
        tokio::join!(mattermost, spaceapi);
    }
}
