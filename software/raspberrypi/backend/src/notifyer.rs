use tokio::sync::mpsc::Receiver;

use crate::{configuration::ConfigurationRef, spaceapi::spaceapi};

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


fn mattermost(configuration: &'static MatterMost, state: bool) {
    use pyo3::{ffi::c_str, prelude::*, types::PyTuple};

    let code = c_str!(std::include_str!("../../backend/python/mattermost.py"));
    Python::with_gil(|py| {
        let fun = PyModule::from_code(py, code, c_str!("mattermost.py"), c_str!("mattermost"))
            .unwrap()
            .getattr("main")
            .unwrap();

        // pass object with Rust tuple of positional arguments
        let args = PyTuple::new(
            py,
            [
                configuration.url.as_str(),
                configuration.loginid.as_str(),
                configuration.apitoken.as_str(),
                configuration.scheme.as_str(),
                &configuration.port.to_string(),
                &state.to_string(),
            ],
        )
        .unwrap();

        let _ = fun.call1(args).unwrap();
    })
}

pub async fn notify(configuration: ConfigurationRef, mut receiver: Receiver<bool>) {
    while let Some(state) = receiver.recv().await {
        // let mqtt = mqtt(state);
        let _spaceapi = spaceapi(&configuration.spaceapi, state).await;
        mattermost(&configuration.mattermost, state);
        // tokio::join!(mqtt, spaceapi);
    }
}
