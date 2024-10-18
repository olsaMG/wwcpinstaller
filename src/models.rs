use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WizepassConfig {
    wizepassauth: WizepassAuthConfig,
    wwcp: WWCPConfig,
    wcs: WCSConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WizepassAuthConfig {
    pub url: String,
    pub instance_id: String,
    pub rp_id: String,
    pub cert_path: Option<String>,
    // Not used
    // pub user_identifier_attribute: String,
    // pub user_identifier_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WWCPConfig {
    loglevel: Loglevel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WCSConfig {
    loglevel: Loglevel,
}

#[derive(Serialize, Deserialize, Debug)]
struct Loglevel(String);
