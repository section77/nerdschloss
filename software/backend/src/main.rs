use anyhow::{Error, Result};
use clap::Parser;
use poem::{listener::TcpListener, Server};
use shadow_rs::shadow;

use backend::{configuration, setup};

shadow!(build);

/// nerdschloss
#[derive(Parser, Debug)]
#[command(version, author, about, long_about)]
struct Args {
    #[clap(short, long)]
    verbosity: Option<usize>,
    /// Show the configuration
    #[arg(short = 'c', long)]
    show_config: bool,
    /// Show build details
    #[arg(short = 'b', long)]
    show_build_details: bool,
    /// Enable the SpaceAPI
    #[arg(short, long)]
    spaceapi: bool,
}

fn load_configuration() -> Result<(configuration::Configuration, Args), Error> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Read configuration from files and environment
    let configuration = configuration::Configuration::new()?;

    // Read commandline arguments
    let args = Args::parse();

    Ok((configuration, args))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (configuration, args) = load_configuration()?;

    if args.show_config {
        dbg!(&configuration);
        return Ok(());
    }
    if args.show_build_details {
        build::print_build_in();
        return Ok(());
    }

    // Listen for new connections
    let listener = TcpListener::bind(std::net::SocketAddr::new(
        configuration.server.ipaddress,
        configuration.server.port,
    ));

    // Serve the application
    let server = Server::new(listener);
    let route = setup(configuration)?;
    server.run(route).await?;
    Ok(())
}
