use backend::setup;

use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    // Listen for new connections
    let listener = TcpListener::bind("127.0.0.1:8080");

    // Serve the application
    let server = Server::new(listener);
    let route = setup().await?;
    server.run(route).await?;
    Ok(())
}
