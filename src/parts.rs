use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]  
pub struct Part {
    value: String,
    quantity: u32,
}

type PartsMap = HashMap<String, Vec<Part>>;

// store a couple of documents
// store a current parts doc with 

// {
//     "transistors": [
//         {
//             "value": "2n5908",
//             "quantity": 1,
//         },
//         {
//             "value": "2n5914",
//             "quantity": 1,
//         },
//     ],
//     "diodes": [],
//     "resistor": [],
//     "capacitor": [],
//     "potentiometers": [],
// }

pub fn create_example_parts_list() -> PartsMap {
    let mut example_parts = HashMap::new();
    example_parts.insert("transistors".to_string(), vec![
        Part {
            value: "2n5908".to_string(),
            quantity: 1,
        },
        Part {
            value: "2n5914".to_string(),
            quantity: 1,
        },
    ]);
    return example_parts;
}