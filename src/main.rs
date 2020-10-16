#![warn(rust_2018_idioms)]
#![allow(clippy::try_err)]

// extern crate diesel;
// extern crate diesel_migrations;
#[macro_use]
extern crate slog_scope;

#[macro_use]
pub mod error;
pub mod logging;
pub mod metrics;
pub mod oauth;
pub mod server;
pub mod settings;
pub mod tags;
pub mod token;

use std::error::Error;

use docopt::Docopt;
use serde_derive::Deserialize;

use logging::init_logging;

const USAGE: &str = "
Usage: tokenserver [options]

Options:
    -h, --help              Show this message
    --config=CONFIGFILE     Tokenserver Configuration file path
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_config: Option<String>,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize Settings:
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let settings = settings::Settings::with_env_and_config_file(&args.flag_config)?;
    init_logging(!settings.human_logs).expect("Logging failed to initialize");
    debug!("Starting up...");

    // Configure sentry error capture
    let curl_transport_factory = |options: &sentry::ClientOptions| {
        Box::new(sentry::transports::CurlHttpTransport::new(&options))
            as Box<dyn sentry::internals::Transport>
    };
    let sentry = sentry::init(sentry::ClientOptions {
        transport: Box::new(curl_transport_factory),
        release: sentry::release_name!(),
        ..sentry::ClientOptions::default()
    });
    if sentry.is_enabled() {
        sentry::integrations::panic::register_panic_handler();
    }

    // run server...
    println!("Hello, world!");
    let server = server::Server::with_settings(settings).expect("Could not start server");
    server.await?;

    // shutdown
    info!("Server closing");
    logging::reset_logging();
    Ok(())
}
