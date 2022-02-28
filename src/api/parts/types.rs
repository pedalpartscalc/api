use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AvailablePart {
    pub id: i64,
    pub owner_id: i64,
    pub part_name: String,
    pub part_kind: String,
    pub quantity: i32,
}

#[derive(Deserialize)]
pub struct PartId {
    pub id: i64,
}

// Available part minus the ids for when we're creating a new one
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAvailablePart {
    pub part_name: String,
    pub part_kind: String,
    pub quantity: i32,
}
