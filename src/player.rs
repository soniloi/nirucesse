use std::collections::HashMap;
use std::rc::Rc;

use inventory::Inventory;
use item::Item;
use location::Location;

pub struct Player {
	inventory: Inventory,
	location: *mut Location,
	score: u32,
}

impl Player {

	pub fn new(initial: *mut Location) -> Player {
		Player {
			inventory: Inventory::new(16),
			location: initial,
			score: 0u32,
		}
	}

	pub fn contains_item(&self, item_ptr: &Rc<Box<Item>>) -> bool {
		self.inventory.contains_item(item_ptr)
	}

	pub fn insert_item(&mut self, item_ptr: Rc<Box<Item>>) {
		self.inventory.insert_item(item_ptr);
	}

	pub fn get_location(&self) -> *mut Location {
		self.location
	}

	pub fn write_out(&self) {
		println!("Player [current score={}] [location={}]", self.score, unsafe{(*self.location).get_stubname()});
		self.inventory.write_out();
	}
}
