use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Debug, Serialize, Clone)]
pub struct RequiredPart {
    pub id: i64,
    pub pedal_id: i64,
    pub part_name: String,
    pub part_kind: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRequiredPart {
    pub part_name: String,
    pub part_kind: String,
    pub quantity: i32,
}


#[derive(Debug, Serialize)]
pub struct PedalPartRow {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub build_doc_link: Option<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,

    // Associated Parts
    pub part_id: Option<i64>,
    pub part_name: Option<String>,
    pub part_kind: Option<String>,
    pub part_quantity: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct Pedal {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub build_doc_link: Option<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,

    pub required_parts: Vec<RequiredPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPedal {
    pub name: String,
    pub kind: String,
    pub build_doc_link: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ClosePedal {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub short_parts: Vec<RequiredPart>,
    pub required_parts: Vec<RequiredPart>
}