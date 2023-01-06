mod handlers;

use handlers::{close, open, state};

use motordriver::run_stepper;

use poem::{endpoint::EmbeddedFileEndpoint, get, EndpointExt, Route};
use tokio::sync::mpsc::channel;

// Setup embedded files
#[derive(rust_embed::RustEmbed)]
#[folder = "../frontend/static/"]
struct StaticFiles;

pub fn setup() -> anyhow::Result<Route, anyhow::Error> {
    // Create channel
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
