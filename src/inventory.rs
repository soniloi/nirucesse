use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use item::Item;
use location::Location;

pub struct Inventory {
	capacity: u32,
	items: HashMap<u32, Rc<RefCell<Box<Item>>>>,
}

impl Inventory {

	pub fn new(capacity: u32) -> Inventory {
		Inventory {
			capacity: capacity,
			items: HashMap::new(),
		}
	}

	pub fn has_light(&self) -> bool {
		// Inventory has light if any item within it has light
		for item in self.items.values() {
			if item.borrow().has_light() {
				return true
			}
		}

		false
	}

	pub fn has_air(&self) -> bool {
		for item in self.items.values() {
			if item.borrow().has_air() {
				return true
			}
		}
		false
	}

	pub fn contains_item_by_id(&self, id: u32) -> bool {
		self.items.contains_key(&id)
	}

	pub fn contains_item(&self, item: &Rc<RefCell<Box<Item>>>) -> bool {
		self.items.contains_key(&item.borrow().get_id())
	}

	pub fn insert_item(&mut self, item: Rc<RefCell<Box<Item>>>) {
		self.items.insert(item.borrow().get_id(), item.clone());
	}

	pub fn remove_item(&mut self, item: &Rc<RefCell<Box<Item>>>) -> Option<Rc<RefCell<Box<Item>>>> {
		self.items.remove(&item.borrow().get_id())
	}

	pub fn remove_item_certain(&mut self, id: u32) -> Rc<RefCell<Box<Item>>> {
		 match self.items.remove(&id) {
			 None => panic!("Data corruption seeking item [{}], fail.", id),
			 Some(item) => item,
		 }
	}

	// Return combined size of all items currently in inventory
	fn get_size(&self) -> u32 {
		let mut result = 0;
		for (_, item) in &self.items {
			result += item.borrow().get_size();
		}
		result
	}

	// Return whether an item could fit in the inventory
	pub fn can_accept(&self, item: &Rc<RefCell<Box<Item>>>) -> bool {
		(item.borrow().get_size() + self.get_size()) <= self.capacity
	}

	pub fn drop_on_death(&mut self, safe_loc: &Rc<RefCell<Box<Location>>>, current_loc: &Rc<RefCell<Box<Location>>>) {
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
