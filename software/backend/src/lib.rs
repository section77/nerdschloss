use motordriver::{run_stepper, Direction};

use poem::{
    endpoint::EmbeddedFileEndpoint, get, handler, web::Data, EndpointExt, IntoResponse, Route,
};

use tokio::sync::mpsc::{channel, Sender};

// Setup embedded files
#[derive(rust_embed::RustEmbed)]
#[folder = "../frontend/static/"]
struct StaticFiles;

#[handler]
async fn state() -> impl IntoResponse {
    #[cfg(debug_assertions)]
    dbg!("state");
}

#[handler]
async fn open(channel: Data<&Sender<Direction>>) -> impl IntoResponse {
    #[cfg(debug_assertions)]
    dbg!("open");
    channel.0.send(Direction::Open).await.unwrap();
}

#[handler]
async fn close(channel: Data<&Sender<Direction>>) -> impl IntoResponse {
    #[cfg(debug_assertions)]
    dbg!("close");
    channel.0.send(Direction::Close).await.unwrap();
}

pub async fn setup() -> anyhow::Result<Route, anyhow::Error> {
    let (sender, receiver) = channel(1);

    // Start logic stuff
    let _ = tokio::task::spawn_blocking(|| {
        run_stepper(receiver);
    });

    // Setup the routs
    let app = Route::new()
        .at("/", EmbeddedFileEndpoint::<StaticFiles>::new("index.html"))
        .at("/state", get(state))
        .at("/open", get(open).data(sender.clone()))
        .at("/close", get(close).data(sender));

    Ok(app)
}
