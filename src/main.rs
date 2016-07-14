mod actions;
mod command;
mod command_collection;
mod file_buffer;
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
use file_buffer::FileBuffer;
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

    let mut buffer = FileBuffer::new(filename);

    let mut cmd_coll = CommandCollection::new();
    cmd_coll.init(&mut buffer);
    while !buffer.eof() {
		println!("{}", buffer.get_line());
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

	kitchen.borrow_mut().set_direction(String::from("southeast"), store.clone());
	kitchen.borrow_mut().set_direction(String::from("up"), ward.clone());
	store.borrow_mut().set_direction(String::from("north"), kitchen.clone());
	store.borrow_mut().set_direction(String::from("west"), garden.clone());
	garden.borrow_mut().set_direction(String::from("southwest"), store.clone());
	ward.borrow_mut().set_direction(String::from("down"), kitchen.clone());
	ward.borrow_mut().insert_item(bowl);

	// Test command
	let take: Rc<Box<Command>> = Rc::new(Box::new(Command::new("take", 0x0c, actions::do_take)));
	let drop: Rc<Box<Command>> = Rc::new(Box::new(Command::new("drop", 0x0e, actions::do_drop)));
	let quit: Rc<Box<Command>> = Rc::new(Box::new(Command::new("quit", 0x00, actions::do_quit)));
	let inventory: Rc<Box<Command>> = Rc::new(Box::new(Command::new("inventory", 0x00, actions::do_inventory)));
	let look: Rc<Box<Command>> = Rc::new(Box::new(Command::new("look", 0x00, actions::do_look)));
	let go: Rc<Box<Command>> = Rc::new(Box::new(Command::new("go", 0xC0, actions::do_go)));
	let north: Rc<Box<Command>> = Rc::new(Box::new(Command::new("north", 0x40, actions::do_go)));
	let northeast: Rc<Box<Command>> = Rc::new(Box::new(Command::new("northeast", 0x40, actions::do_go)));
	let east: Rc<Box<Command>> = Rc::new(Box::new(Command::new("east", 0x40, actions::do_go)));
	let southeast: Rc<Box<Command>> = Rc::new(Box::new(Command::new("southeast", 0x40, actions::do_go)));
	let south: Rc<Box<Command>> = Rc::new(Box::new(Command::new("south", 0x40, actions::do_go)));
	let southwest: Rc<Box<Command>> = Rc::new(Box::new(Command::new("southwest", 0x40, actions::do_go)));
	let west: Rc<Box<Command>> = Rc::new(Box::new(Command::new("west", 0x40, actions::do_go)));
	let northwest: Rc<Box<Command>> = Rc::new(Box::new(Command::new("northwest", 0x40, actions::do_go)));
	let up: Rc<Box<Command>> = Rc::new(Box::new(Command::new("up", 0x40, actions::do_go)));
	let down: Rc<Box<Command>> = Rc::new(Box::new(Command::new("down", 0x40, actions::do_go)));
	let describe: Rc<Box<Command>> = Rc::new(Box::new(Command::new("describe", 0x0c, actions::do_describe)));

	cmd_coll.put("take", take.clone());
	cmd_coll.put("t", take.clone());
	cmd_coll.put("drop", drop.clone());
	cmd_coll.put("dr", drop.clone());
	cmd_coll.put("quit", quit.clone());
	cmd_coll.put("q", quit.clone());
	cmd_coll.put("end", quit.clone());
	cmd_coll.put("i", inventory.clone());
	cmd_coll.put("invent", inventory.clone());
	cmd_coll.put("inventory", inventory.clone());
	cmd_coll.put("l", look.clone());
	cmd_coll.put("look", look.clone());
	cmd_coll.put("go", go.clone());
	cmd_coll.put("walk", go.clone());
	cmd_coll.put("travel", go.clone());
	cmd_coll.put("n", north.clone());
	cmd_coll.put("ne", northeast.clone());
	cmd_coll.put("e", east.clone());
	cmd_coll.put("se", southeast.clone());
	cmd_coll.put("s", south.clone());
	cmd_coll.put("sw", southwest.clone());
	cmd_coll.put("w", west.clone());
	cmd_coll.put("nw", northwest.clone());
	cmd_coll.put("u", up.clone());
	cmd_coll.put("d", down.clone());
	cmd_coll.put("north", north.clone());
	cmd_coll.put("northeast", northeast.clone());
	cmd_coll.put("east", east.clone());
	cmd_coll.put("southeast", southeast.clone());
	cmd_coll.put("south", southeast.clone());
	cmd_coll.put("southwest", southwest.clone());
	cmd_coll.put("west", west.clone());
	cmd_coll.put("northwest", northwest.clone());
	cmd_coll.put("up", up.clone());
	cmd_coll.put("down", down.clone());
	cmd_coll.put("describe", describe.clone());
	cmd_coll.put("de", describe.clone());
	cmd_coll.put("examine", describe.clone());

	// Test player
	let mut player = Box::new(Player::new(ward.clone()));
	player.insert_item(radishes);
	player.write_out();

	// Test terminal
	terminal::write_full("You awaken. You feel ill and dazed. Slowly you raise your head. You try to look around. You are intermittently blinded by flickering light. Groggily and warily you flail around.");

	while player.is_playing() {
		let inputs: Vec<String> = terminal::read_stub((*player).get_location().borrow().get_stubname());
		let cmd_name = &inputs[0];
		if !cmd_name.is_empty() {
			match cmd_coll.get(cmd_name) {
				Some(cmd) => {
					let arg: &str = if inputs.len() > 1 { &inputs[1] } else { "" };
					(**cmd).execute(&item_coll, arg, &mut player)
				},
				None => {
					println!("No such command [{}]", inputs[0]);
					terminal::write_full("I do not understand that instruction");
				},
			}
		}
	}

	// Clean
	terminal::reset();
}
