pub mod configuration;

// mod client;

use std::process::Command;

use configuration::MatterMost;

pub fn mattermost(configuration: &MatterMost, state: bool) {
    if configuration.enable {
        // let code = std::include_str!("../../backend/python/mattermost.py");
        let _cmd = Command::new("./ve/bin/python")
            .args([
                "./backend/python/mattermost.py ",
                configuration.url.as_str(),
                configuration.loginid.as_str(),
                configuration.apitoken.as_str(),
                configuration.scheme.as_str(),
                &configuration.port.to_string(),
                &state.to_string(),
            ])
            .output()
            .expect("Error: Failed to set MatterMost message");
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        http::Method,
        matchers::{any, method},
        {Mock, MockServer, ResponseTemplate},
    };

    use crate::mattermost::configuration::MatterMost;

    use super::mattermost;

    #[tokio::test]
    async fn mattermost_disabled() {
        let mock_server = MockServer::start().await;

        Mock::given(method(Method::PUT))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&mock_server)
            .await;

        let mm_config = MatterMost {
            enable: false,
            ..Default::default()
        };
        mattermost(&mm_config, true);
    }

    #[ignore = "TODO"]
    #[tokio::test]
    async fn send_mattermost_message() {
        let mock_server = MockServer::start().await;

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let mm_config = MatterMost {
            enable: true,
            url: mock_server.uri(),
            loginid: String::new(),
            apitoken: String::new(),
            scheme: String::from("http"),
            port: mock_server.address().port(),
        };
        mattermost(&mm_config, true);
    }
}
