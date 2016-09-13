use std::collections::HashMap;

use data_collection::ItemRef;
use data_collection::LocationRef;

pub struct Inventory {
	capacity: u32,
	items: HashMap<u32, ItemRef>,
}

impl Inventory {

	pub fn new(capacity: u32) -> Inventory {
		Inventory {
			capacity: capacity,
			items: HashMap::new(),
		}
	}

	pub fn has_light(&self) -> bool {
		self.items.values().any(|x| x.borrow().has_light())
	}

	pub fn has_air(&self) -> bool {
		self.items.values().any(|x| x.borrow().has_air())
	}

	pub fn has_gravity(&self) -> bool {
		self.items.values().any(|x| x.borrow().has_gravity())
	}

	pub fn contains_item(&self, id: u32) -> bool {
		self.items.values().any(|x| x.borrow().is_or_contains_item(id))
	}

	pub fn insert_item(&mut self, item: ItemRef) {
		self.items.insert(item.borrow().get_id(), item.clone());
	}

	pub fn remove_item_certain(&mut self, id: u32) -> ItemRef {
		if self.items.contains_key(&id) {
			match self.items.remove(&id) {
				None => {},
				Some(item) => return item,
			}
		}
		for item in self.items.values() {
			if item.borrow().contains_item(id) {
				return item.borrow_mut().remove_item_certain(id);
			}
		}
		panic!("Data corruption seeking item [{}], fail.", id);
	}

	// Return combined size of all items currently in inventory
	fn get_size(&self) -> u32 {
		self.items.values().fold(0, |acc, x| acc + x.borrow().get_size())
	}

	// Return whether an item could fit in the inventory
	pub fn can_accept(&self, item: &ItemRef) -> bool {
		(item.borrow().get_size() + self.get_size()) <= self.capacity
	}

	pub fn drop_on_death(&mut self, safe_loc: &LocationRef, current_loc: &LocationRef) {
		let removed = self.items.drain();
		for (_, item) in removed {
			if item.borrow().is_essential() {
				safe_loc.borrow_mut().insert_item(item.clone());
			} else {
				current_loc.borrow_mut().insert_item(item.clone());
			}
		}
	}

	pub fn mk_string(&self) -> String {
		let mut result = String::new();
		if self.items.is_empty() {
			result = result + "You are not carrying anything.";
		} else {
			result = result + "You currently have the following:";
			for item in self.items.values() {
				result = result + "\n\t" + &item.borrow().get_inventoryname();
			}
		}
		result
	}
}
