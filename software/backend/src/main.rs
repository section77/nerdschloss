use clap::Parser;
use poem::{listener::TcpListener, Server};

use backend::setup;

/// nerdschloss
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    // Parse command line args
    let _args = Args::parse();

    // Listen for new connections
    let listener = TcpListener::bind("127.0.0.1:8080");

    // Serve the application
    let server = Server::new(listener);
    let route = setup()?;
    server.run(route).await?;
    Ok(())
}
