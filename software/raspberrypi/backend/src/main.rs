use std::{boxed::Box, io::IsTerminal};

use anyhow::{Error, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use shadow_rs::shadow;
use tracing::instrument;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt};

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
    verbose: Verbosity,
    #[arg(long, env = "TOKIO_CONSOLE")]
    tokio_console: bool,

    /// Enable the SpaceAPI
    #[arg(short, long)]
    spaceapi: bool,
}

fn init_tracing(verbosity: &Verbosity, enable_console: bool) -> Result<()> {
    let is_interactive = std::io::stdout().is_terminal();
    let level_filter = verbosity.tracing_level_filter();

    // Helper to create the base registry with level filter
    macro_rules! registry {
        () => {
            tracing_subscriber::registry().with(level_filter)
        };
    }

    // Helper to create the interactive formatter
    macro_rules! interactive_fmt {
        () => {
            fmt::layer().with_target(true).with_level(true)
        };
    }

    match (enable_console, is_interactive) {
        (true, true) => {
            // Console + interactive stdout
            registry!()
                .with(console_subscriber::spawn())
                .with(interactive_fmt!())
                .init();
            tracing::info!("Logging: tokio-console (port 6669) + stdout");
        }
        (true, false) => {
            // Console + OS-specific
            init_with_console(level_filter)?;
        }
        (false, true) => {
            // Interactive stdout only
            registry!().with(interactive_fmt!()).init();
            tracing::info!("Logging: interactive mode (stdout)");
        }
        (false, false) => {
            // OS-specific only
            init_os_specific(level_filter)?;
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn init_with_console(level_filter: tracing::level_filters::LevelFilter) -> Result<()> {
    tracing_subscriber::registry()
        .with(level_filter)
        .with(console_subscriber::spawn())
        .with(tracing_journald::layer()?)
        .init();
    tracing::info!("Logging: journald + tokio-console");
    Ok(())
}

#[cfg(target_os = "macos")]
fn init_with_console(level_filter: tracing::level_filters::LevelFilter) -> Result<()> {
    let oslog = tracing_oslog::OsLogger::new("com.yourapp.identifier", "default");
    tracing_subscriber::registry()
        .with(level_filter)
        .with(console_subscriber::spawn())
        .with(oslog)
        .init();
    tracing::info!("Logging: oslog + tokio-console");
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn init_with_console(level_filter: tracing::level_filters::LevelFilter) -> Result<()> {
    tracing_subscriber::registry()
        .with(level_filter)
        .with(console_subscriber::spawn())
        .with(fmt::layer())
        .init();
    tracing::info!("Logging: fallback (stdout) + tokio-console");
    Ok(())
}

#[cfg(target_os = "linux")]
fn init_os_specific(level_filter: tracing::level_filters::LevelFilter) -> Result<()> {
    tracing_subscriber::registry()
        .with(level_filter)
        .with(tracing_journald::layer()?)
        .init();
    tracing::info!("Logging: journald");
    Ok(())
}

#[cfg(target_os = "macos")]
fn init_os_specific(level_filter: tracing::level_filters::LevelFilter) -> Result<()> {
    let oslog = tracing_oslog::OsLogger::new("com.yourapp.identifier", "default");
    tracing_subscriber::registry()
        .with(level_filter)
        .with(oslog)
        .init();
    tracing::info!("Logging: oslog");
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn init_os_specific(level_filter: tracing::level_filters::LevelFilter) -> Result<()> {
    tracing_subscriber::registry()
        .with(level_filter)
        .with(fmt::layer())
        .init();
    tracing::info!("Logging: fallback (stdout)");
    Ok(())
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
    init_tracing(&args.verbose, args.tokio_console)?;

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
