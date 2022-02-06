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
    example_parts.insert(
        "transistors".to_string(),
        vec![
            Part {
                value: "2n5908".to_string(),
                quantity: 1,
            },
            Part {
                value: "2n5914".to_string(),
                quantity: 1,
            },
        ],
    );
    return example_parts;
}

pub fn create_required_parts_list() -> PartsMap {
    let mut example_parts = HashMap::new();
    example_parts.insert(
        "transistors".to_string(),
        vec![
            Part {
                value: "2n5908".to_string(),
                quantity: 2,
            },
            Part {
                value: "2n5914".to_string(),
                quantity: 1,
            },
        ],
    );
    return example_parts;
}

pub fn has_sufficient_parts(available_parts: PartsMap, required_parts: PartsMap) -> bool {
    // TODO: copilot wrote all of this
    for (part_type, parts) in required_parts {
        println!("{}", part_type);
        let available_parts_of_type = available_parts.get(&part_type);
        if available_parts_of_type.is_none() {
            return false;
        }
        let available_parts_of_type = available_parts_of_type.unwrap();
        for part in parts {
            let mut found_part = false;
            for available_part in available_parts_of_type {
                if available_part.value == part.value {
                    if available_part.quantity >= part.quantity {
                        println!("Found {} {}", part.quantity, part.value);
                        found_part = true;
                    } else {
                        println!(
                            "Not enough {}, need {} and have {}",
                            part.value, part.quantity, available_part.quantity
                        );
                    }
                }
            }
            if !found_part {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
#[test]
fn it_works() {
    assert!(!has_sufficient_parts(
        create_example_parts_list(),
        create_required_parts_list()
    ));
}
