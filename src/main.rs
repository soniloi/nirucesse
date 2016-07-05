mod command;
mod command_collection;
mod file_util;
mod inventory;
mod item;
mod item_collection;
mod location;
mod player;
mod terminal;

use std::cell::RefCell;
use std::env;
use std::process;
use std::rc::Rc;

use command::Command;
use command_collection::CommandCollection;
use item::Item;
use item_collection::ItemCollection;
use location::Location;
use player::Player;

fn main() {

	// Get command-line args
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		println!("Filename parameter missing, fail.");
		process::exit(1);
	}
    let filename = &args[1];

    // Read and decompress data file
    let raw = file_util::read_compressed(filename);
    let decompressed = file_util::decompress(&raw);

    // Test print
	let str_contents: Vec<String> = to_str_arr(decompressed);
	for str in str_contents {
		//print!("{}\n", str);
	}

	// Test item
	let bowl: Rc<Box<Item>> = Rc::new(Box::new(Item::new(17u64, 123u32, 2u32, String::from("bowl"), String::from("a bowl"), String::from("a small wooden bowl"), String::from("Made in Lanta"))));
	let medallion: Rc<Box<Item>> = Rc::new(Box::new(Item::new(75u64, 128u32, 2u32, String::from("medallion"), String::from("an asterium medallion"), String::from("a large asterium medallion, engraved with pirate symbolism"), String::from("arr!"))));
	let radishes: Rc<Box<Item>> = Rc::new(Box::new(Item::new(28u64, 132u32, 2u32, String::from("radishes"), String::from("a bunch of radishes"), String::from("some tasty-looking radishes"), String::from(""))));

	let mut item_coll = ItemCollection::new();
	item_coll.put("bowl", bowl.clone());
	item_coll.put("medal", medallion.clone());
	item_coll.put("medallion", medallion.clone());
	item_coll.put("radishes", radishes.clone());

	// Test location
	let kitchen = Rc::new(RefCell::new(Box::new(Location::new(91u64, 765u32, String::from("Kitchen"), String::from("in the kitchen"), String::from(". A lovely aroma of lentil soup lingers in the air. There are doors to the north and southeast")))));
	let store = Rc::new(RefCell::new(Box::new(Location::new(92u64, 763u32, String::from("Store"), String::from("in the food store"), String::from(". The area is filled with sacks, tins, jars, barrels, and casks of the finest food and drink this side of the Etenar Nebula")))));
	let garden = Rc::new(RefCell::new(Box::new(Location::new(93u64, 760u32, String::from("Garden"), String::from("in the garden"), String::from(", a large, high-roofed dome filled with all manner of trees and plants. In the centre, where there is most room for it to grow, stands a particularly large tree")))));
	let ward = Rc::new(RefCell::new(Box::new(Location::new(9u64, 0x70Fu32, String::from("Ward"), String::from("in a medical ward"), String::from(". The faint electric light is flickering on and off, but it is enough to see by. The exit is to the south")))));

	kitchen.borrow_mut().set_direction(String::from(""), store.clone());
	store.borrow_mut().set_direction(String::from(""), kitchen.clone());
	store.borrow_mut().set_direction(String::from(""), garden.clone());
	garden.borrow_mut().set_direction(String::from(""), store.clone());
	ward.borrow_mut().insert_item(bowl);

	// Test command
	let take: Rc<Box<Command>> = Rc::new(Box::new(Command::new(String::from("take"), 0x0c, do_take)));
	let drop: Rc<Box<Command>> = Rc::new(Box::new(Command::new(String::from("drop"), 0x0e, do_drop)));
	let quit: Rc<Box<Command>> = Rc::new(Box::new(Command::new(String::from("quit"), 0x00, do_quit)));

	let mut cmd_coll = CommandCollection::new();
	cmd_coll.put("take", take.clone());
	cmd_coll.put("t", take.clone());
	cmd_coll.put("drop", drop.clone());
	cmd_coll.put("dr", drop.clone());
	cmd_coll.put("quit", quit.clone());
	cmd_coll.put("q", quit.clone());
	cmd_coll.put("end", quit.clone());

	// Test player
	let mut player = Box::new(Player::new(ward.clone()));
	(*player).insert_item(radishes);
	(*player).write_out();

	// Test terminal
	terminal::write_full("You awaken. You feel ill and dazed. Slowly you raise your head. You try to look around. You are intermittently blinded by flickering light. Groggily and warily you flail around.");

	while (*player).is_playing() {
		let inputs: Vec<String> = terminal::read_stub((*player).get_location().borrow().get_stubname());
		match cmd_coll.get(&inputs[0]) {
			Some(cmd) => {
				let arg: &str = if inputs.len() > 1 { &inputs[1] } else { "" };
				(**cmd).execute(&item_coll, arg, &mut player)
			},
			None => {
				println!("No such command [{}]", inputs[0])
			},
		}
		let mut output: String = String::from("Your input was [ ");
		for input in inputs {
			output = output + &input + " ";
		}
		output = output + "]";
		terminal::write_full(&output);

		// Check that the things got moved
		player.write_out();
		ward.borrow().write_out();
	}

	// Clean
	terminal::reset();
}

// Test converter
fn to_str_arr(contents: Vec<char>) -> Vec<String> {

	let mut strs: Vec<String> = vec![];

	let mut current_str: String = String::from("");
	for ch in contents {
		if ch == '\n' {
			strs.push(current_str);
			current_str = String::from("");
		} else {
			current_str.push(ch);
		}
	}

	strs
}

fn do_take(items: &ItemCollection, arg: &str, player: &mut Player) {
	match items.get(arg) {
		None => {
			terminal::write_full("I do not know who or what that is.");
			return;
		},
		Some(item_ptr) => {
			player.pick_up(item_ptr);
		}
	}
}

fn do_drop(items: &ItemCollection, arg: &str, player: &mut Player) {
	match items.get(arg) {
		None => {
			terminal::write_full("I do not know who or what that is.");
			return;
		},
		Some(item_ptr) => {
			player.drop(item_ptr);
		}
	}
}

fn do_quit(items: &ItemCollection, arg: &str, player: &mut Player) {
	player.set_playing(false);
}
