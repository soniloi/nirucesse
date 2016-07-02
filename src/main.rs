mod command;
mod command_collection;
mod file_util;
mod inventory;
mod item;
mod item_collection;
mod location;
mod player;
mod terminal;

use std::env;
use std::process;

use command::Command;
use command_collection::CommandCollection;
use inventory::Inventory;
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
	let bowl_box: Box<Item> = Box::new(Item::new(17u64, 123u32, 2u32, String::from("bowl"), String::from("a bowl"), String::from("a small wooden bowl"), String::from("Made in Lanta")));
	let medallion_box: Box<Item> = Box::new(Item::new(75u64, 128u32, 2u32, String::from("medallion"), String::from("an asterium medallion"), String::from("a large asterium medallion, engraved with pirate symbolism"), String::from("arr!")));
	let bowl_ptr = Box::into_raw(bowl_box);
	let medallion_ptr = Box::into_raw(medallion_box);

	let mut item_coll = ItemCollection::new();
	item_coll.put("bowl", bowl_ptr);
	item_coll.put("medal", medallion_ptr);
	item_coll.put("medallion", medallion_ptr);
	unsafe { (*bowl_ptr).write_out(); }

	// Test location
	let mut kitchen = Location::new(91u64, 765u32, String::from("Kitchen"), String::from("in the kitchen"), String::from(". A lovely aroma of lentil soup lingers in the air. There are doors to the north and southeast"));
	let mut store = Location::new(92u64, 763u32, String::from("Store"), String::from("in the food store"), String::from(". The area is filled with sacks, tins, jars, barrels, and casks of the finest food and drink this side of the Etenar Nebula"));
	let mut garden = Location::new(93u64, 760u32, String::from("Garden"), String::from("in the garden"), String::from(", a large, high-roofed dome filled with all manner of trees and plants. In the centre, where there is most room for it to grow, stands a particularly large tree"));
	let mut ward = Location::new(9u64, 0x70Fu32, String::from("Ward"), String::from("in a medical ward"), String::from(". The faint electric light is flickering on and off, but it is enough to see by. The exit is to the south"));

	kitchen.set_direction(String::from("southeast"), &mut store as *mut Location);
	store.set_direction(String::from("north"), &mut kitchen as *mut Location);
	store.set_direction(String::from("west"), &mut garden as *mut Location);
	garden.set_direction(String::from("northeast"), &mut store as *mut Location);

	ward.insert_item(bowl_ptr);

	// Test command
	let take_fn: fn(items: &ItemCollection, arg: &str, player: &mut Player) = do_take;
	let drop_fn: fn(items: &ItemCollection, arg: &str, player: &mut Player) = do_drop;
	let take_cmd = Command::new(String::from("take"), 0x0c, do_take);
	let drop_cmd = Command::new(String::from("drop"), 0x0e, do_drop);

	let mut cmd_coll = CommandCollection::new();
	cmd_coll.put("take", &take_cmd as *const Command);
	cmd_coll.put("t", &take_cmd as *const Command); // Alias
	cmd_coll.put("drop", &drop_cmd as *const Command);
	cmd_coll.put("dr", &drop_cmd as *const Command);

	// Test player
	let mut player = Player::new(&mut ward as *mut Location);
	player.write_out();

	// Test terminal
	terminal::write_full("You awaken. You feel ill and dazed. Slowly you raise your head. You try to look around. You are intermittently blinded by flickering light. Groggily and warily you flail around.");

	let inputs: Vec<String> = terminal::read_location(kitchen.get_stubname());

	match cmd_coll.get(&inputs[0]) {
		Some(cmd) => {
			unsafe{
				let arg: &str = if inputs.len() > 1 { &inputs[1] } else { "" };
				(**cmd).execute(&item_coll, arg, &mut player)
			}
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
	ward.write_out();

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
			unsafe {
				if player.contains_item(*item_ptr) {
					terminal::write_full("You are already carrying that.");
					return;
				}
				let location_ptr = player.get_location();

				(**item_ptr).write_out();
				match (*location_ptr).remove_item(*item_ptr) {
					None => {
						terminal::write_full("That item is not at this location.");
					}
					Some(item) => {
						player.insert_item(item);
						terminal::write_full("Taken.");
					}
				}
			}
		}
	}
}

fn do_drop(items: &ItemCollection, arg: &str, player: &mut Player) {
//TODO
}
