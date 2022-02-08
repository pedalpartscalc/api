table! {
    available_parts (id) {
        id -> Int8,
        owner_id -> Int8,
        part_name -> Varchar,
        part_kind -> Varchar,
        quantity -> Int4,
    }
}
