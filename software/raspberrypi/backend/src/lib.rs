pub mod configuration;
mod handlers;
mod logic;
mod notifyer;

use anyhow::Result;
use poem::{
    endpoint::EmbeddedFileEndpoint, get, listener::TcpListener, EndpointExt, Route, Server,
};
use tokio::sync::mpsc::{channel, Sender};

use self::{
    configuration::ConfigurationRef,
    handlers::{close, open, state},
    logic::logic,
};

// Setup embedded files
#[derive(rust_embed::RustEmbed)]
#[folder = "../frontend/static/"]
struct StaticFiles;

pub fn setup_routes(sender: Sender<hardware::Direction>) -> Result<Route> {
    // Setup the routs
    let routes = Route::new()
        .at("/", EmbeddedFileEndpoint::<StaticFiles>::new("index.html"))
        .at("/state", get(state))
        .at("/open", get(open).data(sender.clone()))
        .at("/close", get(close).data(sender));

    Ok(routes)
}

pub async fn run(configuration: ConfigurationRef) -> anyhow::Result<()> {
    // Create channel
    let (spaceapi_sender, spaceapi_receiver) = channel(1);
    let (sender, receiver) = channel(1);

    // Listen for new connections
    let listener = TcpListener::bind(std::net::SocketAddr::new(
        configuration.server.ipaddress,
        configuration.server.port,
    ));

    tokio::spawn(async move { notifyer::notify(configuration, spaceapi_receiver).await });

    tokio::spawn(async move { logic(configuration, receiver, spaceapi_sender).await });

    // Serve the application
    let server = Server::new(listener);
    let route = setup_routes(sender)?;
    server.run(route).await?;

    Ok(())
}
