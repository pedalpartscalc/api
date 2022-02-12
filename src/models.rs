use serde::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct AvailablePart {
    pub id: i64,
    pub owner_id: i64,
    pub part_name: String,
    pub part_kind: String,
    pub quantity: i32,
}

use super::schema::available_parts;

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "available_parts"]
pub struct NewAvailablePart<'a> {
    pub part_name: &'a str,
    pub part_kind: &'a str,
    pub quantity: i32,
    pub owner_id: i64,
}
