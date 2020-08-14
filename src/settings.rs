//! Application settings objects and initialization
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use url::Url;

static DEFAULT_PORT: u16 = 8000;

/*
static KILOBYTE: u32 = 1024;
static MEGABYTE: u32 = KILOBYTE * KILOBYTE;
static DEFAULT_MAX_POST_BYTES: u32 = 2 * MEGABYTE;
static DEFAULT_MAX_POST_RECORDS: u32 = 100;
static DEFAULT_MAX_RECORD_PAYLOAD_BYTES: u32 = 2 * MEGABYTE;
static DEFAULT_MAX_REQUEST_BYTES: u32 = DEFAULT_MAX_POST_BYTES + 4 * KILOBYTE;
static DEFAULT_MAX_TOTAL_BYTES: u32 = 100 * DEFAULT_MAX_POST_BYTES;
static DEFAULT_MAX_TOTAL_RECORDS: u32 = 100 * DEFAULT_MAX_POST_RECORDS;
*/
static PREFIX: &str = "tokenserv";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub port: u16,
    pub host: String,
    pub database_url: String,
    pub statsd_host: Option<String>,
    pub statsd_port: u16,
    pub statsd_label: String,
    pub human_logs: bool,
    pub privkey_path: String,
    pub pubkey_path: String,
    pub shared_secret: String,
    pub auth_endpoint: Option<String>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            debug: false,
            port: DEFAULT_PORT,
            host: "127.0.0.1".to_string(),
            database_url: "mysql://root@127.0.0.1/tokenstorage".to_string(),
            statsd_host: None,
            statsd_port: 8125,
            statsd_label: "tokenserver".to_string(),
            human_logs: false,
            privkey_path: "src/private_rsa_key.pem".to_string(),
            pubkey_path: "src/public_rsa_key.pem".to_string(),
            shared_secret: "".to_owned(),
            auth_endpoint: None,
        }
    }
}

impl Settings {
    /// Load the settings from the config file if supplied, then the environment.
    pub fn with_env_and_config_file(filename: &Option<String>) -> Result<Self, ConfigError> {
        // there appears to be a bug with `Config::try_from`
        // https://github.com/mehcode/config-rs/issues/144
        // let mut s = Config::try_from(&Settings::default())?;
        let mut config = Config::new();

        // Merge the config file if supplied
        if let Some(config_filename) = filename {
            config.merge(File::with_name(config_filename))?;
        }

        // Merge the environment overrides
        // Environment variables are usually upper case. Let's uppercase the
        // PREFIX just to make sure.
        config.merge(Environment::with_prefix(&PREFIX.to_ascii_uppercase()))?;

        // hand merge the configuration and defaults into a consistent settings.
        // bleh.
        let default = Self::default();
        Ok(Self {
            debug: config.get_bool("config").unwrap_or(default.debug),
            port: config.get_int("port").unwrap_or(default.port as i64) as u16,
            host: config.get_str("host").unwrap_or(default.host),
            database_url: config
                .get_str("database_url")
                .unwrap_or(default.database_url),
            statsd_host: match config.get_str("statsd_host") {
                Ok(value) => Some(value),
                Err(_) => default.statsd_host,
            },
            statsd_port: config
                .get_int("statsd_port")
                .unwrap_or(default.statsd_port as i64) as u16,
            statsd_label: config
                .get_str("statsd_label")
                .unwrap_or(default.statsd_label),
            human_logs: config.get_bool("human_logs").unwrap_or(default.human_logs),
            privkey_path: config
                .get_str("privkey_path")
                .unwrap_or(default.privkey_path),
            pubkey_path: config.get_str("pubkey_path").unwrap_or(default.pubkey_path),
            shared_secret: config
                .get_str("shared_secret")
                .unwrap_or(default.shared_secret),
            auth_endpoint: match config.get_str("auth_endpoint") {
                Ok(value) => Some(value),
                Err(_) => default.auth_endpoint,
            },
        })
    }

    /// A simple banner for display of certain settings at startup
    pub fn banner(&self) -> String {
        let db = Url::parse(&self.database_url)
            .map(|url| url.scheme().to_owned())
            .unwrap_or_else(|_| "<invalid db>".to_owned());
        format!("http://{}:{} ({})", self.host, self.port, db)
    }
}
