use std::collections::HashMap;

use inventory::Inventory;
use item::Item;
use location::Location;

pub struct Player {
	inventory: Inventory,
	location: *const Location,
	score: u32,
}

impl Player {

	pub fn new(initial: *const Location) -> Player {
		Player {
			inventory: Inventory::new(16),
			location: initial,
			score: 0u32,
		}
	}

	pub fn insert_item(&mut self, item: Item) {
		self.inventory.insert_item(item);
	}

	pub fn write_out(&self) {
		println!("Player [current score={}] [location={}]", self.score, unsafe{(*self.location).get_stubname()});
		self.inventory.write_out();
	}
}
