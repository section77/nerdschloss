use std::boxed::Box;

use anyhow::{Error, Result};
use clap::Parser;
use shadow_rs::shadow;
use tracing::instrument;
use tracing_log::{AsTrace, LogTracer};
use tracing_subscriber::FmtSubscriber;

use backend::{configuration, run};

shadow!(build);

/// nerdschloss
#[derive(Parser, Debug)]
#[command(version, author, about, long_about)]
struct Args {
    /// Show the configuration
    #[arg(short = 'c', long)]
    show_config: bool,
    /// Show build details
    #[arg(short = 'b', long)]
    show_build_details: bool,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    /// Enable the SpaceAPI
    #[arg(short, long)]
    spaceapi: bool,
}

fn load_configuration() -> Result<(configuration::ConfigurationRef, Args), Error> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Read configuration from files and environment
    let configuration = Box::leak(Box::new(configuration::Configuration::new()?));

    // Read commandline arguments
    let args = Args::parse();

    Ok((configuration, args))
}

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Error> {
    let (configuration, args) = load_configuration()?;

    // Setup logging
    LogTracer::init()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default tracing subscriber failed");

    if args.show_config {
        dbg!(&configuration);
        return Ok(());
    }
    if args.show_build_details {
        build::print_build_in();
        return Ok(());
    }

    run(configuration).await?;

    Ok(())
}
