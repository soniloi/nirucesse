use std::collections::HashMap;

use inventory::Inventory;
use item::Item;

pub struct Player {
	score: u32,
	inventory: Inventory,
}

impl Player {

	pub fn new() -> Player {
		Player {
			score: 0u32,
			inventory: Inventory::new(16),
		}
	}

	pub fn insert_item(&mut self, item: Item) {
		self.inventory.insert_item(item);
	}

	pub fn write_out(&self) {
		println!("Player [current score={}]", self.score);
		self.inventory.write_out();
	}
}
