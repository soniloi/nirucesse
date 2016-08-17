extern crate rand;

mod actions;
mod command;
mod command_collection;
mod data_collection;
mod file_buffer;
mod file_util;
mod game;
mod inventory;
mod item;
mod item_collection;
mod location;
mod location_collection;
mod player;
mod string_collection;
mod terminal;

use std::env;
use std::process;

use data_collection::DataCollection;
use file_buffer::FileBuffer;
use game::Game;
use player::Player;

// ID numbers of specific items
pub const ITEM_ID_MATCHES: u32 = 1048;
pub const ITEM_ID_ROBOT: u32 = 1061;

fn main() {

    let filename = get_filename();

    let data = init_data(&filename);
    let player = init_player(&data);

    let mut game = Game::new(data, player);
    game.play();

	terminal::reset();
}

fn get_filename() -> String {
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		println!("Filename parameter missing, fail.");
		process::exit(1);
	}
    args[1].clone()
}

fn init_data(filename: &String) -> DataCollection {
    let mut buffer = FileBuffer::new(&filename);
    let mut data = DataCollection::new();
    data.init(&mut buffer);
    data
}

fn init_player(data: &DataCollection) -> Player {
	let start_loc = data.get_location_wake();
	Player::new(start_loc.clone())
}
