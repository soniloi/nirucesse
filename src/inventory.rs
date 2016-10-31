use std::collections::HashMap;

use constants;
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
		self.items.values().any(|x| x.borrow().has_property(constants::CTRL_ITEM_GIVES_AIR))
	}

	pub fn has_gravity(&self) -> bool {
		self.items.values().any(|x| x.borrow().has_property(constants::CTRL_ITEM_GIVES_GRAVITY))
	}

	pub fn has_nosnomp(&self) -> bool {
		self.items.values().any(|x| x.borrow().has_property(constants::CTRL_ITEM_GIVES_NOSNOMP))
	}

	pub fn has_invisibility(&self) -> bool {
		self.items.values().any(|x| x.borrow().has_property(constants::CTRL_ITEM_GIVES_INVISIBILITY))
	}

	pub fn has_land(&self) -> bool {
		self.items.values().any(|x| x.borrow().has_property(constants::CTRL_ITEM_GIVES_LAND))
	}

	pub fn contains_item(&self, id: u32) -> bool {
		self.items.values().any(|x| x.borrow().is_or_contains_item(id))
	}

	pub fn insert_item(&mut self, item: ItemRef) {
		item.borrow_mut().set_locations(constants::LOCATION_ID_INVENTORY);
		self.items.insert(item.borrow().get_id(), item.clone());
	}

	pub fn remove_item_certain(&mut self, id: u32) {
		if self.items.contains_key(&id) {
			let found_option = self.items.remove(&id);
			match found_option {
				None => {},
				Some(found) => {
					let is_liquid = found.borrow().has_property(constants::CTRL_ITEM_LIQUID);
					if !is_liquid {
						found.borrow_mut().retire();
					}
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

	pub fn drop_all(&mut self, current_loc: &LocationRef, safe_loc: &LocationRef, death: bool, permanent: bool) {
		let removed = self.items.drain();
		for (_, item) in removed {
			if death && item.borrow().has_property(constants::CTRL_ITEM_ESSENTIAL) {
				safe_loc.borrow_mut().insert_item(item.clone(), permanent);
			} else {
				current_loc.borrow_mut().insert_item(item.clone(), permanent);
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
