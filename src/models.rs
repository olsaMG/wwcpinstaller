use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WizepassConfig {
    pub wizepassauth: WizepassAuthConfig,
    pub wwcp: WWCPConfig,
    pub wcs: WCSConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WizepassAuthConfig {
    pub url: String,
    pub instance_id: String,
    pub rp_id: String,
    pub cert_path: Option<String>,
    // Not used
    // pub user_identifier_attribute: String,
    // pub user_identifier_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WWCPConfig {
    pub loglevel: Loglevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WCSConfig {
    pub loglevel: Loglevel,
}

//TODO: fix type
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Loglevel(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum TracingLevel {
    Info,
    Debug,
    Error,
    Trace,
    Warn,
}

impl TracingLevel {
    pub fn set_loglevel(&self, wwcp: &mut WWCPConfig, wcs: &mut WCSConfig) {
        wcs.loglevel.0 = self.to_string();
        wwcp.loglevel.0 = self.to_string();
    }
}

impl fmt::Display for TracingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TracingLevel::Info => write!(f, "info"),
            TracingLevel::Debug => write!(f, "debug"),
            TracingLevel::Error => write!(f, "error"),
            TracingLevel::Trace => write!(f, "trace"),
            TracingLevel::Warn => write!(f, "warn"),
        }
    }
}
