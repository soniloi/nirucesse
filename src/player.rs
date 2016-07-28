use std::cell::RefCell;
use rand;
use rand::Rng;
use std::rc::Rc;

use data_collection::DataCollection;
use inventory::Inventory;
use item::Item;
use location::Location;
use terminal;

pub struct Player {
	inventory: Inventory,
	location: Rc<RefCell<Box<Location>>>,
	score: u32, // player's current score
	playing: bool, // whether player is currently playing
	hints: u32, // number of hints player has requested
	instructions: u32, // number of instructions player has entered
	deaths: u32, // number of times player has died
	alive: bool,
}

impl Player {

	pub fn new(initial: Rc<RefCell<Box<Location>>>) -> Player {
		Player {
			inventory: Inventory::new(16),
			location: initial,
			score: 0u32,
			playing: true,
			hints: 0u32,
			instructions: 0u32,
			deaths: 0u32,
			alive: true,
		}
	}

	fn has_light(&self) -> bool {
		self.inventory.has_light() || self.location.borrow().has_light()
	}

	pub fn contains_item(&self, item_ptr: &Rc<Box<Item>>) -> bool {
		self.inventory.contains_item(item_ptr)
	}

	pub fn insert_item(&mut self, item_ptr: Rc<Box<Item>>) {
		self.inventory.insert_item(item_ptr);
	}

	pub fn get_location(&self) -> &Rc<RefCell<Box<Location>>> {
		&self.location
	}

	pub fn is_playing(&self) -> bool {
		self.playing
	}

	pub fn set_playing(&mut self, b: bool) {
		self.playing = b
	}

	pub fn is_alive(&self) -> bool {
		self.alive
	}

	pub fn set_alive(&mut self, b: bool) {
		self.alive = b
	}

	fn die(&mut self, data: &DataCollection) {
		self.set_alive(false);
		self.increment_deaths();
		match data.get_location_wake() {
			None => panic!("Unable to get wake location, fail"),
			Some(loc_wake) => {
				self.location = loc_wake.clone();
			},
		}
	}

	// Have player attempt to pick up item from current location
	pub fn pick_up(&mut self, item: &Rc<Box<Item>>) {
		if self.contains_item(item) {
			terminal::write_full("You are already carrying that.");
			return;	
		}

		let it = self.location.borrow_mut().remove_item(item);
		match it {
			None => {
				terminal::write_full("That item is not at this location.");
			}
			Some(i) => {
				self.insert_item(i);
				terminal::write_full("Taken.");
			}
		}
	}

	// Have player attempt to drop item from inventory to current location
	pub fn drop(&mut self, item: &Rc<Box<Item>>) {
		let it = self.inventory.remove_item(item);
		match it {
			None => {
				terminal::write_full("You are not carrying it.");
			}
			Some(i) => {
				self.location.borrow_mut().insert_item(i);
				terminal::write_full("Dropped.");
			}
		}	
	}

	// Describe an item in the player's inventory or at the player's location
	pub fn describe(&self, item: &Rc<Box<Item>>) {
		if self.inventory.contains_item(item) || self.location.borrow().contains_item(item) {
			terminal::write_full(&item.mk_full_string());
		} else {
			let response = String::from("I see no ") + &item.get_shortname() + " here.";
			terminal::write_full(&response);
		}
	}

	// Have player travel to an adjacent location
	// TODO: I don't really like this very much; there's probably a better way
	pub fn go(&mut self, data: &DataCollection, dir: String) {

		let loc_clone = self.location.clone();
		let self_loc = loc_clone.borrow();
		match self_loc.get_direction(dir) {
			None => {
				terminal::write_full("You cannot go that way.");
				return;
			},
			Some(next) => {
				let mut rng = rand::thread_rng();
				let death_rand: u32 = rng.gen();
				let death = death_rand % 4 == 0;
				if !self.has_light() && !next.borrow().has_light() && death {
					terminal::write_full("... ouch! You seem to have tripped on something. You fall and break your neck in a multitude of places.");
					self.die(data);
				} else {
					self.location = next.clone();
					terminal::write_full(&self.location.borrow().mk_full_string());
				}
			},
		}
	}

	pub fn get_score(&self) -> u32 {
		self.score
	}

	pub fn get_hints(&self) -> u32 {
		self.hints
	}

	pub fn increment_hints(&mut self) {
		self.hints = self.hints + 1;
	}

	pub fn get_instructions(&self) -> u32 {
		self.instructions
	}

	pub fn increment_instructions(&mut self) {
		self.instructions = self.instructions + 1;
	}

	pub fn decrement_instructions(&mut self) {
		self.instructions = self.instructions - 1;
	}

	pub fn get_deaths(&self) -> u32 {
		self.deaths
	}

	pub fn increment_deaths(&mut self) {
		self.deaths = self.deaths + 1;
	}

	pub fn mk_inventory_string(&self) -> String {
		self.inventory.mk_string()
	}

	pub fn mk_location_string(&self) -> String {
		self.location.borrow().mk_full_string()
	}

	pub fn read(&self, item: &Rc<Box<Item>>) {
		if self.inventory.contains_item(item) || self.location.borrow().contains_item(item) {
			terminal::write_full(&item.mk_writing_string());
		} else {
			let response = String::from("I see no ") + &item.get_shortname() + " here.";
			terminal::write_full(&response);
		}
	}
}
