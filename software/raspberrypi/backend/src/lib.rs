pub mod configuration;
mod handlers;
mod logic;
mod notify;

use poem::{endpoint::EmbeddedFileEndpoint, get, EndpointExt, Route};
use tokio::sync::mpsc::channel;

use self::{
    configuration::Configuration,
    handlers::{close, open, state},
    logic::logic,
};

// Setup embedded files
#[derive(rust_embed::RustEmbed)]
#[folder = "../frontend/static/"]
struct StaticFiles;

#[tracing::instrument]
pub fn setup(configuration: Configuration) -> anyhow::Result<Route, anyhow::Error> {
    let (spaceapi_sender, spaceapi_receiver) = channel(1);
    let spaceapi_config = configuration.spaceapi.clone();
    tokio::spawn(async move { notify::spaceapi(&spaceapi_config, spaceapi_receiver).await });

    // Create channel
    let (sender, receiver) = channel(1);

    // Start logic stuff
    tokio::task::spawn(async move { logic(configuration, receiver, spaceapi_sender).await });

    // Setup the routs
    let routes = Route::new()
        .at("/", EmbeddedFileEndpoint::<StaticFiles>::new("index.html"))
        .at("/state", get(state))
        .at("/open", get(open).data(sender.clone()))
        .at("/close", get(close).data(sender));

    Ok(routes)
}
