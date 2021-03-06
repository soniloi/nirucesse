use std::collections::HashMap;

use constants;
use data_collection::{InventoryId, ItemId, ItemProperties, ItemRef, LocationRef};

pub struct Inventory {
	id: InventoryId,
	capacity: u32,
	items: HashMap<ItemId, ItemRef>,
}

impl Inventory {

	pub fn new(id: InventoryId, capacity: u32) -> Inventory {
		Inventory {
			id: id,
			capacity: capacity,
			items: HashMap::new(),
		}
	}

	pub fn contains_with_property(&self, property_code: ItemProperties, on_optional: bool) -> bool {
		self.items.values().any(|x| x.borrow().has_or_contains_with_property(property_code, on_optional))
	}

	pub fn contains_item(&self, id: ItemId) -> bool {
		self.items.values().any(|x| x.borrow().is_or_contains_item(id))
	}

	pub fn insert_item(&mut self, item: ItemRef) {
		item.borrow_mut().set_location(self.id);
		self.items.insert(item.borrow().get_id(), item.clone());
	}

	pub fn remove_item_certain(&mut self, id: ItemId) {
		if self.items.contains_key(&id) {
			let found_option = self.items.remove(&id);
			if let Some(found) = found_option {
				let is_liquid = found.borrow().has_property(constants::CTRL_ITEM_LIQUID);
				if !is_liquid {
					found.borrow_mut().retire();
				}
			}
			return;
		}
		for item in self.items.values() {
			if item.borrow().contains_item(id) {
				item.borrow_mut().remove_item_certain(id);
				return;
			}
		}
		panic!("Data corruption seeking item [{}], fail.", id);
	}

	// Return combined size of all items currently in inventory
	fn get_size(&self) -> u32 {
		self.items.values().fold(0, |acc, x| acc + x.borrow().get_size())
	}

	// Return whether an item could fit in the inventory
	pub fn can_fit(&self, item: &ItemRef) -> bool {
		(item.borrow().get_size() + self.get_size()) <= self.capacity
	}

	pub fn drop_on_death(&mut self, current_loc: &LocationRef, safe_loc: &LocationRef) {
		let removed = self.items.drain();
		for (_, item) in removed {
			let essential = item.borrow().has_property(constants::CTRL_ITEM_ESSENTIAL);
			if essential {
				safe_loc.borrow_mut().insert_item(item.clone());
			} else {
				current_loc.borrow_mut().insert_item(item.clone());
			}
		}
	}

	pub fn mk_string(&self, inventory_empty: &str, inventory_intro: &str) -> String {
		let mut result = String::new();
		if self.items.is_empty() {
			result = result + inventory_empty;
		} else {
			result = result + inventory_intro;
			for item in self.items.values() {
				result = result + "\n\t" + &item.borrow().get_inventoryname();
			}
		}
		result
	}
}
