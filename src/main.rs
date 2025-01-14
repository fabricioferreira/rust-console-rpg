mod game;

use game::command::Command;
use game::player::Player;
use game::room::initialize_rooms;
use game::Describable;

fn main() {
    let rooms = initialize_rooms();
    let mut player = Player::new("Entrance");

    loop {
        let current_room = &rooms[&player.current_room];
        println!("{}.", current_room.describe());
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
            Command::Drop(item) => {
                match player.drop_item(item) {
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
            }
            Command::Use(item) => {
                match player.use_item(item) {
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
            }
            Command::Inventory => {
                player.show_inventory();
            }
        }
    }
}
