use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub client_origin_url: String,
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("Provide missing environment variables for Config")
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    pub message: String,
}


// Wrapper for an ID value returned by many queries
#[derive(Deserialize)]
pub struct Id {
    pub id: i64,
}