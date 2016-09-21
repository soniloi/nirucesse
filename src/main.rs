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

pub const SCORE_PUZZLE: u32 = 20; // The score the player gets for every puzzle solved
pub const SCORE_TREASURE: u32 = 10; // The score the player gets for each treasure stowed
pub const PENALTY_DEATH: u32 = 25; // The value deducted from player's score for every death
pub const PENALTY_HINT: u32 = 10; // The value deducted from player's score for every hint they request

// ID numbers of specific locations
pub const LOCATION_ID_AIRLOCKE: u32 = 31; // The airlock just off the Recreation Hub
pub const LOCATION_ID_AIRLOCKEOUT: u32 = 36; // The area immediately outside Airlock East
pub const LOCATION_ID_TREASURESTORE: u32 = 23; // Where the player must bring treasure to
pub const LOCATION_ID_TELEPORT_0: u32 = 107; // Location of teleporter connected to Experiment Area
pub const LOCATION_ID_TELEPORT_1: u32 = 128; // Location of teleporter connected to Chasm

// ID numbers of specific items
pub const ITEM_ID_AQUA: u32 = 1084;
pub const ITEM_ID_BREAD: u32 = 1010;
pub const ITEM_ID_DRAGON: u32 = 1027;
pub const ITEM_ID_KOHLRABI: u32 = 1042;
pub const ITEM_ID_LAMP: u32 = 1043;
pub const ITEM_ID_LION: u32 = 1045;
pub const ITEM_ID_MATCHES: u32 = 1048;
pub const ITEM_ID_POTION: u32 = 1059;
pub const ITEM_ID_ROBOT: u32 = 1061;
pub const ITEM_ID_TOAST: u32 = 1069;
pub const ITEM_ID_TOOTH: u32 = 1070;
pub const ITEM_ID_TROLL: u32 = 1073;
pub const ITEM_ID_WATER: u32 = 1076;
pub const ITEM_ID_WHISTLE: u32 = 1077;
pub const ITEM_ID_WOLF: u32 = 1080;

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
