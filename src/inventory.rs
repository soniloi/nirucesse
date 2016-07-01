use std::collections::HashMap;

use item::Item;

pub struct Inventory {
	capacity: u32,
	items: HashMap<u64, Item>,
}

impl Inventory {

	pub fn new(capacity: u32) -> Inventory {
		Inventory {
			capacity: capacity,
			items: HashMap::new(),
		}
	}

	pub fn insert_item(&mut self, item: Item) {
		self.items.insert(item.get_id(), item);
	}

	pub fn write_out(&self) {
		if self.items.len() < 1 {
			println!("There are currently no items in the inventory.");
		} else {
			println!("The inventory contains the following items:");
			for (key, val) in self.items.iter() {
				println!("\tThere is {} here [id={}]", (*val).get_longname(), key);
			}
		}
	}
}
