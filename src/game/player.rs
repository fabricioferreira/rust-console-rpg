use std::io;
use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub current_room: String,
    pub inventory: Vec<String>,
}

pub enum PlayerError {
    InvalidRoom(String),
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerError::InvalidRoom(room_name) => write!(f, "You can't go to the room {}", room_name),
        }
    }
}

impl Player {
    pub fn new(starting_room: &str) -> Player {
        Player {
            current_room: starting_room.to_string(),
            inventory: Vec::new(),
        }
    }

    pub fn get_input(&self) -> String {
        let mut input = String::new();
        println!("Enter a command:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input.trim().to_string()
    }

    pub fn move_to(&mut self, room_name: &str, connected_rooms: &[String]) -> Result<(), PlayerError> {
        if connected_rooms.iter().any(|room| room.to_lowercase() == room_name.to_lowercase()) {
            self.current_room = connected_rooms
                .iter()
                .find(|room| room.to_lowercase() == room_name.to_lowercase())
                .unwrap()
                .clone();
            Ok(())
        } else {
            Err(PlayerError::InvalidRoom(room_name.to_string()))
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.inventory.push(item);
        print!("Item added to inventory");
    }

    pub fn show_inventory(&self) {
        println!("Inventory: {:?}", self.inventory);
    }
}