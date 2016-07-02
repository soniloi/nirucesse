use std::collections::HashMap;

use item::Item;

pub struct Inventory {
	capacity: u32,
	items: HashMap<u64, *const Item>,
}

impl Inventory {

	pub fn new(capacity: u32) -> Inventory {
		Inventory {
			capacity: capacity,
			items: HashMap::new(),
		}
	}

	pub fn contains_item(&self, item: *const Item) -> bool {
		for val in self.items.values() {
			if item == *val {
				return true;
			}
		}
		false
	}

	pub fn insert_item(&mut self, item: *const Item) {
		unsafe { self.items.insert((*item).get_id(), item); }
	}

	pub fn write_out(&self) {
		if self.items.is_empty() {
			println!("There are currently no items in the inventory.");
		} else {
			println!("The inventory contains the following items:");
			for (key, val) in self.items.iter() {
				unsafe { println!("\t{} [id={}]", (**val).get_longname(), key); }
			}
		}
	}
}
