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

    pub fn move_to(&mut self, room_name: &String, connected_rooms: &[String]) -> Result<(), PlayerError> {
        let actual_room = connected_rooms
            .iter()
            .find(|room| room.to_lowercase() == room_name.to_lowercase())
            .ok_or_else(|| PlayerError::InvalidRoom(room_name.to_string()))?;

            self.current_room = actual_room.clone();

        Ok(())
    }

    pub fn add_item(&mut self, item: String) {
        self.inventory.push(item);
        println!("Item added to inventory");
    }

    pub fn drop_item(&mut self, item: String) -> Result<(), String> {
        let item_index = self.inventory
            .iter()
            .position(|i| i.to_lowercase() == item.to_lowercase())
            .ok_or_else(|| format!("You don't have {} in your inventory!", item))?;

        self.inventory.remove(item_index);

        println!("You dropped {} from your inventory.", item);
        Ok(())
    }

    pub fn use_item(&mut self, item: String) -> Result<(), String> {
        let actual_item = self.inventory
            .iter()
            .find(|i| i.to_lowercase() == item.to_lowercase())
            .ok_or_else(|| format!("You can't use {} because it is not in your inventory!", item))?;

        println!("You used {} from your inventory.", actual_item);
        Ok(())
    }
    
    pub fn show_inventory(&self) {
        println!("Inventory: {:?}", self.inventory);
    }
}
