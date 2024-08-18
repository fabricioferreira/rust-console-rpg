use std::collections::HashMap;

use super::Describable;

#[derive(Debug)]
pub struct Room {
    pub description: String,
    pub connected_rooms: Vec<String>,
}

impl Describable for Room {
    fn describe(&self) -> String {
        format!("You're in {}", self.description)
    }
}

pub fn initialize_rooms() -> HashMap<String, Room> {
    let mut rooms = HashMap::new();

    rooms.insert(
        "Entrance".to_string(),
        Room {
            description: "the entrance of a dark cave".to_string(),
            connected_rooms: vec!["Hall".to_string()],
        },
    );

    rooms.insert(
        "Hall".to_string(),
        Room {
            description: "a long hall filled with ancient paintings".to_string(),
            connected_rooms: vec!["Entrance".to_string(), "Chamber".to_string()],
        },
    );

    rooms.insert(
        "Chamber".to_string(),
        Room {
            description: "a misterious chamber with a faint light glowing".to_string(),
            connected_rooms: vec!["Hall".to_string()],
        },
    );

    rooms
}