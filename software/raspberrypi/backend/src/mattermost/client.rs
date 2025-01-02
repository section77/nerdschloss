use anyhow::Result;
use reqwest::{
    header,
    header::{HeaderMap, HeaderValue},
    Method,
};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::{error, info, instrument, trace};

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
pub enum Error {
    #[error("Some HTTP error {0:?}")]
    Http(String),
    #[error("Some Responce error {0:?}")]
    Responce(String),
    #[error("Some JSON error {0:?}")]
    Json(String),
    #[error("Some API error {0:?}")]
    Api(ApiError),
}

// a type wrapping the API's errors
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiError {
    pub error_code: i64,
    pub message: String,
}

#[derive(Debug)]
pub struct SendRequestInput<B: Serialize + std::fmt::Debug> {
    pub method: Method,
    pub url: String,
    pub body: B,
}

#[derive(Debug)]
pub struct Client {
    http_client: reqwest_middleware::ClientWithMiddleware,
    api_base_url: String,
    _api_key: SecretString,
}

impl Client {
    #[instrument]
    pub fn new(api_base_url: String, api_key: SecretString) -> Result<Self> {
        trace!("Create HTTP headers");
        let mut header_map = HeaderMap::new();
        let mut acesstoken_header_value =
            HeaderValue::from_str(&format!("Token {}", api_key.expose_secret()))?;
        acesstoken_header_value.set_sensitive(true);
        header_map.insert(header::AUTHORIZATION, acesstoken_header_value);
        header_map.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );
        header_map.insert(
            header::ACCEPT,
            HeaderValue::from_str("application/json").unwrap(),
        );

        trace!("Create HTTP retry policy");
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(5);
        trace!("Create HTTP client");
        let http_client = ClientBuilder::new(
            reqwest::Client::builder()
                .default_headers(header_map)
                .build()?,
        )
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .with(TracingMiddleware::default())
        .build();

        Ok(Client {
            http_client,
            api_base_url: format!("{}api/", api_base_url),
            _api_key: api_key,
        })
    }

    #[instrument]
    pub(crate) async fn send_request<B: Serialize + std::fmt::Debug, R: DeserializeOwned>(
        &self,
        input: SendRequestInput<B>,
    ) -> Result<R, Error> {
        let url = format!("{}{}", &self.api_base_url, input.url);
        let res = self
            .http_client
            .request(input.method, url)
            .json(&input.body)
            .send()
            .await
            .map_err(|err| Error::Http(format!("error sending request: {err}")))?;

        if 399 < res.status().as_u16() {
            let bytes = res.bytes().await.map_err(|err| {
                error!("error getting bytes from error response: {err}");
                Error::Responce(format!("error getting bytes from error response: {err}"))
            })?;

            let text = String::from_utf8(bytes.clone().to_vec()).map_err(|err| {
                error!("error getting bytes from error response: {err}");
                Error::Responce(format!("error getting bytes from error response: {err}"))
            })?;
            info!("Response text: {}", text);

            let err = serde_json::from_slice::<ApiError>(&bytes).map_err(|err| {
                error!("error parsing error response: {err}");
                Error::Json(format!("error parsing error response: {err}"))
            })?;

            return Err(Error::Api(err));
        }

        let res: R = res.json().await.map_err(|err| {
            error!("error parsing responce {err}");
            Error::Json(format!("error parsing responce {err}"))
        })?;

        Ok(res)
    }
}
