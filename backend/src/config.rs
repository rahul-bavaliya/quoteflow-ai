// backend/src/config.rs

//! Application configuration loaded from environment variables.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const DEFAULT_HOST: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED); // 0.0.0.0
const DEFAULT_PORT: u16 = 3000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConfigError {
    pub key: &'static str,
    pub value: String,
    pub reason: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}={:?}: {}", self.key, self.value, self.reason)
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    /// Load configuration from the real process environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    /// Load configuration using any key -> value lookup (used by tests).
    pub fn from_lookup<F>(lookup: F) -> Result<Self, ConfigError>
    where
        F: Fn(&str) -> Option<String>,
    {
        let host = match lookup("APP_HOST") {
            Some(value) => parse_host(value)?,
            None => DEFAULT_HOST,
        };

        let port = match lookup("APP_PORT") {
            Some(value) => parse_port(value)?,
            None => DEFAULT_PORT,
        };

        Ok(Self { host, port })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

fn parse_host(value: String) -> Result<IpAddr, ConfigError> {
    match value.trim().parse::<IpAddr>() {
        Ok(ip) => Ok(ip),
        Err(err) => Err(ConfigError {
            key: "APP_HOST",
            reason: err.to_string(),
            value,
        }),
    }
}

fn parse_port(value: String) -> Result<u16, ConfigError> {
    match value.trim().parse::<u16>() {
        Ok(0) => Err(ConfigError {
            key: "APP_PORT",
            reason: "port must be between 1 and 65535".into(),
            value,
        }),
        Ok(port) => Ok(port),
        Err(err) => Err(ConfigError {
            key: "APP_PORT",
            reason: err.to_string(),
            value,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Build a fake environment from key/value pairs.
    fn env(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |key| map.get(key).cloned()
    }

    #[test]
    fn uses_defaults_when_nothing_is_set() {
        let config = Config::from_lookup(env(&[])).unwrap();
        assert_eq!(config.host, DEFAULT_HOST);
        assert_eq!(config.port, DEFAULT_PORT);
    }

    #[test]
    fn reads_host_and_port() {
        let config =
            Config::from_lookup(env(&[("APP_HOST", "127.0.0.1"), ("APP_PORT", "8080")])).unwrap();
        assert_eq!(config.socket_addr().to_string(), "127.0.0.1:8080");
    }

    #[test]
    fn rejects_non_numeric_port() {
        let err = Config::from_lookup(env(&[("APP_PORT", "abc")])).unwrap_err();
        assert_eq!(err.key, "APP_PORT");
    }

    #[test]
    fn rejects_port_zero() {
        let err = Config::from_lookup(env(&[("APP_PORT", "0")])).unwrap_err();
        assert_eq!(err.key, "APP_PORT");
    }

    #[test]
    fn rejects_out_of_range_port() {
        let err = Config::from_lookup(env(&[("APP_PORT", "70000")])).unwrap_err();
        assert_eq!(err.key, "APP_PORT");
    }

    #[test]
    fn rejects_invalid_host() {
        let err = Config::from_lookup(env(&[("APP_HOST", "not-an-ip")])).unwrap_err();
        assert_eq!(err.key, "APP_HOST");
    }
}
