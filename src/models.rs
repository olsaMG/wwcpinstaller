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
