//! Application settings objects and initialization
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use url::Url;

static DEFAULT_PORT: u16 = 8000;

static KILOBYTE: u32 = 1024;
static MEGABYTE: u32 = KILOBYTE * KILOBYTE;
static DEFAULT_MAX_POST_BYTES: u32 = 2 * MEGABYTE;
static DEFAULT_MAX_POST_RECORDS: u32 = 100;
static DEFAULT_MAX_RECORD_PAYLOAD_BYTES: u32 = 2 * MEGABYTE;
static DEFAULT_MAX_REQUEST_BYTES: u32 = DEFAULT_MAX_POST_BYTES + 4 * KILOBYTE;
static DEFAULT_MAX_TOTAL_BYTES: u32 = 100 * DEFAULT_MAX_POST_BYTES;
static DEFAULT_MAX_TOTAL_RECORDS: u32 = 100 * DEFAULT_MAX_POST_RECORDS;
static PREFIX: &str = "tokenserv";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub port: u16,
    pub host: String,
    pub database_url: String,
    pub database_pool_max_size: Option<u32>,
    #[cfg(any(test, feature = "db_test"))]
    pub database_use_test_transactions: bool,

    /// The master secret, from which are derived
    /// the signing secret and token secret
    /// that are used during Hawk authentication.
    pub human_logs: bool,

    pub statsd_host: Option<String>,
    pub statsd_port: u16,
    pub statsd_label: String,
    pub privkey_path: String,
    pub pubkey_path: String,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            debug: false,
            port: DEFAULT_PORT,
            host: "127.0.0.1".to_string(),
            database_url: "mysql://root@127.0.0.1/tokenstorage".to_string(),
            database_pool_max_size: None,
            #[cfg(any(test, feature = "db_test"))]
            database_use_test_transactions: false,
            statsd_host: None,
            statsd_port: 8125,
            statsd_label: "tokenserver".to_string(),
            human_logs: false,
            privkey_path: "src/private_rsa_key.pem".to_string(),
            pubkey_path: "src/public_rsa_key.pem".to_string(),
        }
    }
}

impl Settings {
    /// Load the settings from the config file if supplied, then the environment.
    pub fn with_env_and_config_file(filename: &Option<String>) -> Result<Self, ConfigError> {
        let mut s = Config::try_from(&Settings::default())?;
        // let mut s = Config::new();
        s.set_default("limits.max_post_bytes", i64::from(DEFAULT_MAX_POST_BYTES))?;
        s.set_default(
            "limits.max_post_records",
            i64::from(DEFAULT_MAX_POST_RECORDS),
        )?;
        s.set_default(
            "limits.max_record_payload_bytes",
            i64::from(DEFAULT_MAX_RECORD_PAYLOAD_BYTES),
        )?;
        s.set_default(
            "limits.max_request_bytes",
            i64::from(DEFAULT_MAX_REQUEST_BYTES),
        )?;
        s.set_default("limits.max_total_bytes", i64::from(DEFAULT_MAX_TOTAL_BYTES))?;
        s.set_default(
            "limits.max_total_records",
            i64::from(DEFAULT_MAX_TOTAL_RECORDS),
        )?;

        // Merge the config file if supplied
        if let Some(config_filename) = filename {
            s.merge(File::with_name(config_filename))?;
        }

        // Merge the environment overrides
        s.merge(Environment::with_prefix(PREFIX))?;

        Ok(match s.try_into::<Self>() {
            Ok(s) => {
                // Adjust the max values if required.
                s
            }
            Err(e) => match e {
                // Configuration errors are not very sysop friendly, Try to make them
                // a bit more 3AM useful.
                ConfigError::Message(v) => {
                    println!("Bad configuration: {:?}", &v);
                    println!("Please set in config file or use environment variable.");
                    println!(
                        "For example to set `database_url` use env var `{}_DATABASE_URL`\n",
                        PREFIX.to_uppercase()
                    );
                    error!("Configuration error: Value undefined {:?}", &v);
                    return Err(ConfigError::NotFound(v));
                }
                _ => {
                    error!("Configuration error: Other: {:?}", &e);
                    return Err(e);
                }
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
