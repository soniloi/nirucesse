use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use inventory::Inventory;
use item::Item;
use location::Location;
use terminal;

pub struct Player {
	inventory: Inventory,
	location: Rc<RefCell<Box<Location>>>,
	score: u32,
	playing: bool,
}

impl Player {

	pub fn new(initial: Rc<RefCell<Box<Location>>>) -> Player {
		Player {
			inventory: Inventory::new(16),
			location: initial,
			score: 0u32,
			playing: true,
		}
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

	pub fn mk_inventory_string(&self) -> String {
		self.inventory.mk_string()
	}

	pub fn mk_location_string(&self) -> String {
		self.location.borrow().mk_full_string()
	}

	pub fn write_out(&self) {
		println!("Player [current score={}] [location={}]", self.score, self.location.borrow().get_stubname());
		self.inventory.write_out();
	}
}
