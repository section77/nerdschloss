use motordriver::{run_stepper, Direction};

use poem::{
    endpoint::EmbeddedFileEndpoint, get, handler, listener::TcpListener, web::Data, EndpointExt,
    IntoResponse, Route, Server,
};
use tokio::sync::mpsc::{channel, Sender};

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

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let (sender, receiver) = channel(1);
    let _ = tokio::task::spawn_blocking(|| {
        run_stepper(receiver);
    });

    let app = Route::new()
        .at("/", EmbeddedFileEndpoint::<StaticFiles>::new("index.html"))
        .at("/state", get(state))
        .at("/open", get(open).data(sender.clone()))
        .at("/close", get(close).data(sender));

    let listener = TcpListener::bind("127.0.0.1:8080");
    let server = Server::new(listener);
    server.run(app).await?;
    Ok(())
}
