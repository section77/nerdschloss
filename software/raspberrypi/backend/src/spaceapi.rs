pub mod configuration {
    use secrecy::SecretString;
    use serde::Deserialize;

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct SpaceAPI {
        pub enable: bool,
        pub url: String,
        pub username: String,
        pub password: SecretString,
    }
}

use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use secrecy::ExposeSecret;
use tracing::{error, info};

use configuration::SpaceAPI;

pub async fn spaceapi(configuration: &SpaceAPI, state: bool) {
    info!("SpaceAPI {state:?}");

    if !configuration.enable {
        return;
    }

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

#[cfg(test)]
mod tests {
    use fake::{
        faker::internet::en::{Password, Username},
        Fake,
    };
    use wiremock::{
        http::Method,
        matchers::{basic_auth, method, query_param},
        {Mock, MockServer, ResponseTemplate},
    };

    use super::{configuration::SpaceAPI, spaceapi};

    #[tokio::test]
    async fn set_spaceapi_disabled() {
        let mock_server = MockServer::start().await;

        Mock::given(method(Method::PUT))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&mock_server)
            .await;

        let config = SpaceAPI {
            enable: false,
            ..Default::default()
        };
        spaceapi(&config, true).await;
    }

    #[tokio::test]
    async fn set_spaceapi_enabled() {
        let mock_server = MockServer::start().await;

        let username = Username().fake::<String>();
        let password = Password(0..40).fake::<String>();

        Mock::given(method(Method::PUT))
            .and(basic_auth(&username, &password))
            .and(query_param("status", "open"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let config = SpaceAPI {
            enable: true,
            url: mock_server.uri(),
            username,
            password: secrecy::SecretString::from(password),
        };
        spaceapi(&config, true).await;
    }
}
