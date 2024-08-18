mod game;

use game::command::Command;
use game::player::Player;
use game::room::{ initialize_rooms, Room };
use game::Describable;

fn main() {
    let rooms = initialize_rooms();
    let mut player = Player::new("Entrance");

    loop {
        let current_room = &rooms[&player.current_room];
        println!("You are in {}.", current_room.describe());
        println!("Connected rooms: {:?}", current_room.connected_rooms);

        let input = player.get_input();
        let command = Command::parse(&input);

        match command {
            Command::Quit => {
                println!("Thanks for playing!");
                break;
            }
            Command::Move(room_name) => {
                match player.move_to(&room_name, &current_room.connected_rooms) {
                    Ok(_) => {} // Move was successful
                    Err(e) => println!("{}", e),
                }
            }
            Command::Take(item) => {
                player.add_item(item);
            }
            Command::Inventory => {
                player.show_inventory();
            }
            Command::Invalid => {
                println!("Invalid command! Try again.");
            }
        }
    }
}
