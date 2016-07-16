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
mod terminal;

use std::cell::RefCell;
use std::env;
use std::process;
use std::rc::Rc;

use command_collection::CommandCollection;
use file_buffer::FileBuffer;
use item::Item;
use item_collection::ItemCollection;
use location_collection::LocationCollection;
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

    let mut buffer = FileBuffer::new(filename);

    let mut cmd_coll = CommandCollection::new();
    cmd_coll.init(&mut buffer);

    let mut loc_coll = LocationCollection::new();
    loc_coll.init(&mut buffer);

/*
    while !buffer.eof() {
		println!("{}", buffer.get_line());
    }
*/

	// Test item
	let bowl: Rc<Box<Item>> = Rc::new(Box::new(Item::new(17u64, 123u32, 2u32, String::from("bowl"), String::from("a bowl"), String::from("a small wooden bowl"), String::from("Made in Lanta"))));
	let medallion: Rc<Box<Item>> = Rc::new(Box::new(Item::new(75u64, 128u32, 2u32, String::from("medallion"), String::from("an asterium medallion"), String::from("a large asterium medallion, engraved with pirate symbolism"), String::from("arr!"))));
	let radishes: Rc<Box<Item>> = Rc::new(Box::new(Item::new(28u64, 132u32, 2u32, String::from("radishes"), String::from("a bunch of radishes"), String::from("some tasty-looking radishes"), String::from(""))));

	let mut item_coll = ItemCollection::new();
	item_coll.put("bowl", bowl.clone());
	item_coll.put("medal", medallion.clone());
	item_coll.put("medallion", medallion.clone());
	item_coll.put("radishes", radishes.clone());

/*
	// Test location
	let kitchen = Rc::new(RefCell::new(Box::new(Location::new(91u32, 765u32, String::from("Kitchen"), String::from("in the kitchen"), String::from(". A lovely aroma of lentil soup lingers in the air. There are doors to the north and southeast")))));
	let store = Rc::new(RefCell::new(Box::new(Location::new(92u32, 763u32, String::from("Store"), String::from("in the food store"), String::from(". The area is filled with sacks, tins, jars, barrels, and casks of the finest food and drink this side of the Etenar Nebula")))));
	let garden = Rc::new(RefCell::new(Box::new(Location::new(93u32, 760u32, String::from("Garden"), String::from("in the garden"), String::from(", a large, high-roofed dome filled with all manner of trees and plants. In the centre, where there is most room for it to grow, stands a particularly large tree")))));
	let ward = Rc::new(RefCell::new(Box::new(Location::new(9u32, 0x70Fu32, String::from("Ward"), String::from("in a medical ward"), String::from(". The faint electric light is flickering on and off, but it is enough to see by. The exit is to the south")))));

	kitchen.borrow_mut().set_direction(String::from("southeast"), store.clone());
	kitchen.borrow_mut().set_direction(String::from("up"), ward.clone());
	store.borrow_mut().set_direction(String::from("north"), kitchen.clone());
	store.borrow_mut().set_direction(String::from("west"), garden.clone());
	garden.borrow_mut().set_direction(String::from("southwest"), store.clone());
	ward.borrow_mut().set_direction(String::from("down"), kitchen.clone());
	ward.borrow_mut().insert_item(bowl);

    loc_coll.put(91u32, kitchen.clone());
    loc_coll.put(92u32, store.clone());
    loc_coll.put(93u32, garden.clone());
    loc_coll.put(9u32, ward.clone());
*/

	let start_loc = match loc_coll.get(9u32) {
		None => panic!("Unable to set starting location number: {}", 9u32),
		Some(loc) => loc,
	};

	// Test player
	let mut player = Box::new(Player::new(start_loc.clone()));
	player.insert_item(radishes);
	player.write_out();

	// Test terminal
	terminal::write_full("You awaken. You feel ill and dazed. Slowly you raise your head. You try to look around. You are intermittently blinded by flickering light. Groggily and warily you flail around.");

	while player.is_playing() {
		let inputs: Vec<String> = terminal::read_stub((*player).get_location().borrow().get_stubname());
		let cmd_name = inputs[0].clone();
		if !cmd_name.is_empty() {
			match cmd_coll.get(cmd_name.clone()) {
				Some(cmd) => {
					let arg: &str = if inputs.len() > 1 { &inputs[1] } else { "" };
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
