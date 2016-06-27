mod file_util;
mod item;
mod location;
mod terminal;

use std::env;
use std::process;

use item::Item;
use location::Location;

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
	let item = Item::new(17u64, 123u32, 2u32, String::from("a bowl"), String::from("a small wooden bowl"), String::from("Made in Lanta"));
	item.write_out();

	// Test location
	let mut kitchen = Location::new(91u64, 765u32, String::from("Kitchen"), String::from("in the kitchen"), String::from(". A lovely aroma of lentil soup lingers in the air. There are doors to the north and southeast"));
	let mut store = Location::new(92u64, 763u32, String::from("Store"), String::from("in the food store"), String::from(". The area is filled with sacks, tins, jars, barrels, and casks of the finest food and drink this side of the Etenar Nebula"));
	let mut garden = Location::new(93u64, 760u32, String::from("Garden"), String::from("in the garden"), String::from(", a large, high-roofed dome filled with all manner of trees and plants. In the centre, where there is most room for it to grow, stands a particularly large tree"));

	kitchen.set_direction(String::from("southeast"), &mut store as *mut Location);
	store.set_direction(String::from("north"), &mut kitchen as *mut Location);
	store.set_direction(String::from("west"), &mut garden as *mut Location);
	garden.set_direction(String::from("northeast"), &mut store as *mut Location);

	kitchen.drop_item(item);

	kitchen.write_out();
	store.write_out();
	garden.write_out();

	// Test terminal
	terminal::write_full("You awaken. You feel ill and dazed. Slowly you raise your head. You try to look around. You are intermittently blinded by flickering light. Groggily and warily you flail around.");
	
	let inputs: Vec<String> = terminal::read_location(kitchen.get_stubname());
	let mut output: String = String::from("Your input was [ ");
	for input in inputs {
		output = output + &input + " ";
	}
	output = output + "]";
	
	terminal::write_full(&output);

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
