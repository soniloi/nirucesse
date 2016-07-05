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
	let bowl_box: Rc<Box<Item>> = Rc::new(Box::new(Item::new(17u64, 123u32, 2u32, String::from("bowl"), String::from("a bowl"), String::from("a small wooden bowl"), String::from("Made in Lanta"))));
	let medallion_box: Rc<Box<Item>> = Rc::new(Box::new(Item::new(75u64, 128u32, 2u32, String::from("medallion"), String::from("an asterium medallion"), String::from("a large asterium medallion, engraved with pirate symbolism"), String::from("arr!"))));
	let radishes_box: Rc<Box<Item>> = Rc::new(Box::new(Item::new(28u64, 132u32, 2u32, String::from("radishes"), String::from("a bunch of radishes"), String::from("some tasty-looking radishes"), String::from(""))));

	let mut item_coll = ItemCollection::new();
	item_coll.put("bowl", bowl_box.clone());
	item_coll.put("medal", medallion_box.clone());
	item_coll.put("medallion", medallion_box.clone());
	item_coll.put("radishes", radishes_box.clone());
	(*bowl_box).write_out();

	// Test location
	let kitchen_box = Rc::new(RefCell::new(Box::new(Location::new(91u64, 765u32, String::from("Kitchen"), String::from("in the kitchen"), String::from(". A lovely aroma of lentil soup lingers in the air. There are doors to the north and southeast")))));
	let store_box = Rc::new(RefCell::new(Box::new(Location::new(92u64, 763u32, String::from("Store"), String::from("in the food store"), String::from(". The area is filled with sacks, tins, jars, barrels, and casks of the finest food and drink this side of the Etenar Nebula")))));
	let garden_box = Rc::new(RefCell::new(Box::new(Location::new(93u64, 760u32, String::from("Garden"), String::from("in the garden"), String::from(", a large, high-roofed dome filled with all manner of trees and plants. In the centre, where there is most room for it to grow, stands a particularly large tree")))));
	let ward_box = Rc::new(RefCell::new(Box::new(Location::new(9u64, 0x70Fu32, String::from("Ward"), String::from("in a medical ward"), String::from(". The faint electric light is flickering on and off, but it is enough to see by. The exit is to the south")))));

	kitchen_box.borrow_mut().set_direction(String::from(""), store_box.clone());
	store_box.borrow_mut().set_direction(String::from(""), kitchen_box.clone());
	store_box.borrow_mut().set_direction(String::from(""), garden_box.clone());
	garden_box.borrow_mut().set_direction(String::from(""), store_box.clone());
	ward_box.borrow_mut().insert_item(bowl_box);

	// Test command
	let take_box: Rc<Box<Command>> = Rc::new(Box::new(Command::new(String::from("take"), 0x0c, do_take)));
	let drop_box: Rc<Box<Command>> = Rc::new(Box::new(Command::new(String::from("drop"), 0x0e, do_drop)));

	let mut cmd_coll = CommandCollection::new();
	cmd_coll.put("take", take_box.clone());
	cmd_coll.put("t", take_box.clone());
	cmd_coll.put("drop", drop_box.clone());
	cmd_coll.put("dr", drop_box.clone());

	// Test player
	let mut player = Box::new(Player::new(ward_box.clone()));
	(*player).insert_item(radishes_box);
	(*player).write_out();

	// Test terminal
	terminal::write_full("You awaken. You feel ill and dazed. Slowly you raise your head. You try to look around. You are intermittently blinded by flickering light. Groggily and warily you flail around.");

	let inputs: Vec<String> = terminal::read_location(kitchen_box.borrow().get_stubname());
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
	player.write_out();
	ward_box.borrow().write_out();

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
