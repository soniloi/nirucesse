extern crate rand;

mod actions;
mod command;
mod command_collection;
mod data_collection;
mod file_buffer;
mod file_util;
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
use player::Player;

fn main() {

	// Get command-line args
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		println!("Filename parameter missing, fail.");
		process::exit(1);
	}
    let filename = &args[1];

    let mut buffer = FileBuffer::new(filename);
    let mut data = DataCollection::new();
    data.init(&mut buffer);

	let start_loc = match data.get_location(9u32) {
		None => panic!("Unable to set starting location number: {}", 9u32),
		Some(loc) => loc,
	};
	let mut player = Box::new(Player::new(start_loc.clone()));

	terminal::write_full(data.get_response("initial"));
	while player.is_alive() && player.is_playing() {
		let inputs: Vec<String> = terminal::read_stub((*player).get_location().borrow().get_stubname());
		let cmd_name = inputs[0].clone();
		if !cmd_name.is_empty() {
			player.increment_instructions();
			match data.get_command(cmd_name.clone()) {
				Some(cmd) => {
					let arg: String = if inputs.len() > 1 { inputs[1].clone() } else { String::from("") };
					(**cmd).execute(&data, arg, &mut player)
				},
				None => {
					terminal::write_full(data.get_response("notuigin"));
				},
			}
		}
		// Something in this move killed the player; see whether they want to continue
		if !player.is_alive() {
			terminal::write_full(data.get_response("desreinc"));

			let reincarnate: bool = get_yes_no(data.get_response("askreinc"), data.get_response("notuigse"));
			match reincarnate {
				true => {
					terminal::write_full(data.get_response("doreinc"));
					player.set_alive(true);
				},
				false => {
					terminal::write_full(data.get_response("ok"));
				},
			}
		}

		else if player.is_playing() && !player.has_light() {
			terminal::write_full(data.get_response("lampno"));
		}
	}

	// Clean
	terminal::reset();
}

// Look for an answer to a yes-no question
fn get_yes_no(question: &str, default: &str) -> bool {

	loop {
		let mut response: Vec<String> = terminal::read_question(question);
		while response.is_empty() {
			response = terminal::read_question(question);
		}

		match response[0].as_ref() {
			"yes" | "y" | "true" => return true,
			"no" | "n" | "false" => return false,
			_ => terminal::write_full(default),
		}
	}
}