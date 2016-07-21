mod actions;
mod command;
mod command_collection;
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

use command_collection::CommandCollection;
use file_buffer::FileBuffer;
use item_collection::ItemCollection;
use location_collection::LocationCollection;
use player::Player;
use string_collection::StringCollection;

fn main() {

	// Get command-line args
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		println!("Filename parameter missing, fail.");
		process::exit(1);
	}
    let filename = &args[1];

    let mut buffer = FileBuffer::new(filename);

    let mut cmd_coll = CommandCollection::new();
    cmd_coll.init(&mut buffer);

    let mut loc_coll = LocationCollection::new();
    loc_coll.init(&mut buffer);

    let mut item_coll = ItemCollection::new();
    item_coll.init(&mut buffer, &mut loc_coll);

    let mut hints = StringCollection::new();
    hints.init(&mut buffer);

    let mut explanations = StringCollection::new();
    explanations.init(&mut buffer);

    let mut general = StringCollection::new();
    general.init(&mut buffer);

    let mut events = StringCollection::new();
    events.init(&mut buffer);

/*
    while !buffer.eof() {
		println!("{}", buffer.get_line());
    }
*/

	match hints.get(String::from("troll")) {
		None => terminal::write_full("Error: no hint for troll."),
		Some(hint) => terminal::write_full(hint),
	}

	match hints.get(String::from("cinnamon")) {
		None => terminal::write_full("Pass: no hint found for cinnamon."),
		_ => terminal::write_full("Error: hint found for unknown question: cinnamon."),
	}

	match explanations.get(String::from("Taznassa")) {
		None => terminal::write_full("Error: no explanation for Taznassa."),
		Some(exp) => terminal::write_full(exp),
	}

	match general.get(String::from("drink")) {
		None => terminal::write_full("Error: no string for drink."),
		Some(st) => terminal::write_full(st),
	}

	match events.get(String::from("elf")) {
		None => terminal::write_full("Error: no event for elf."),
		Some(event) => terminal::write_full(event),
	}

	let start_loc = match loc_coll.get(9u32) {
		None => panic!("Unable to set starting location number: {}", 9u32),
		Some(loc) => loc,
	};
	let mut player = Box::new(Player::new(start_loc.clone()));

	terminal::write_full("You awaken. You feel ill and dazed. Slowly you raise your head. You try to look around. You are intermittently blinded by flickering light. Groggily and warily you flail around.");
	while player.is_playing() {
		let inputs: Vec<String> = terminal::read_stub((*player).get_location().borrow().get_stubname());
		let cmd_name = inputs[0].clone();
		if !cmd_name.is_empty() {
			match cmd_coll.get(cmd_name.clone()) {
				Some(cmd) => {
					let arg: String = if inputs.len() > 1 { inputs[1].clone() } else { String::from("") };
					(**cmd).execute(&item_coll, arg, &mut player)
				},
				None => {
					println!("No such command [{}]", cmd_name);
					terminal::write_full("I do not understand that instruction");
				},
			}
		}
	}

	// Clean
	terminal::reset();
}
