use std::collections::HashMap;
use std::rc::Rc;

use item::Item;

pub struct Inventory {
	capacity: u32,
	items: HashMap<u32, Rc<Box<Item>>>,
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
			if item.has_light() {
				return true
			}
		}

		false
	}

	pub fn contains_item(&self, item: &Rc<Box<Item>>) -> bool {
		self.items.contains_key(&(*item).get_id())
	}

	pub fn insert_item(&mut self, item: Rc<Box<Item>>) {
		self.items.insert((*item).get_id(), item);
	}

	pub fn remove_item(&mut self, item: &Rc<Box<Item>>) -> Option<Rc<Box<Item>>> {
		self.items.remove(&(*item).get_id())
	}

	pub fn mk_string(&self) -> String {
		let mut result = String::new();
		if self.items.is_empty() {
			result = result + "You are not carrying anything.";
		} else {
			result = result + "You currently have the following:";
			for item in self.items.values() {
				result = result + "\n\t" + item.get_longname();
			}
		}
		result
	}
}
