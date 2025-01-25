pub mod configuration;

mod client;
// mod types;

use std::process::Command;

use configuration::MatterMost;

pub fn mattermost(configuration: &MatterMost, state: bool) {
    if configuration.enable {
        let code = std::include_str!("../../../backend/python/mattermost.py");
        let output = Command::new("python3")
            // let output = Command::new("./ve/bin/python3")
            .args([
                // "python/mattermost.py",
                "-c",
                code,
                configuration.url.as_str(),
                configuration.loginid.as_str(),
                configuration.apitoken.as_str(),
                configuration.scheme.as_str(),
                &configuration.port.to_string(),
                &state.to_string(),
            ])
            .output()
            .expect("Error: Failed to set MatterMost message");

        println!("output start");
        use std::io::{self, Write};
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        println!("output end");
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        http::Method,
        matchers::{body_json, header, method, path},
        {Mock, MockServer, ResponseTemplate},
    };

    use crate::mattermost::configuration::MatterMost;

    use super::mattermost;

    #[tokio::test]
    async fn mattermost_disabled() {
        let mock_server = MockServer::start().await;

        Mock::given(method(Method::GET))
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

    // #[tokio::test]
    // async fn send_mattermost_message() {
    //     let mock_server = MockServer::start().await;

    //     let user_responce = super::types::user::UserResponce {
    //         id: "bla".to_string(),
    //         ..Default::default()
    //     };
    //     Mock::given(method(Method::GET))
    //         .and(path("api/v4/users/me"))
    //         .and(header("content-type", "application/json"))
    //         .and(body_json(&user_responce))
    //         .respond_with(ResponseTemplate::new(200))
    //         .expect(1)
    //         .mount(&mock_server)
    //         .await;

    //     // Mock::given(method(Method::GET))
    //     //     .and(path("api/v4/teams/name/section77"))
    //     //     .and(bearer_token(""))
    //     //     .respond_with(ResponseTemplate::new(200))
    //     //     .expect(1)
    //     //     .mount(&mock_server)
    //     //     .await;

    //     let mm_config = MatterMost {
    //         enable: true,
    //         url: mock_server.address().ip().to_string(),
    //         loginid: String::from("clubstatus"),
    //         apitoken: String::from("apitoken"),
    //         scheme: String::from("http"),
    //         port: mock_server.address().port(),
    //     };
    //     mattermost(&mm_config, true);
    // }
}
