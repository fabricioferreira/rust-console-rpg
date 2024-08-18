pub enum Command {
    Quit,
    Move(String),
    Take(String),
    Inventory,
    Invalid,
}

impl Command {
    pub fn parse(input: &str) -> Command {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match input.to_lowercase().as_str() {
            "quit" => Command::Quit,
            "inventory" => Command::Inventory,
            "take" if parts.len() > 1 => Command::Take(parts[1].to_string()),
            room_name => Command::Move(room_name.to_string()),
        }
    }
}