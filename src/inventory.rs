use std::collections::HashMap;
use std::rc::Rc;

use item::Item;

pub struct Inventory {
	capacity: u32,
	items: HashMap<u64, Rc<Box<Item>>>,
}

impl Inventory {

	pub fn new(capacity: u32) -> Inventory {
		Inventory {
			capacity: capacity,
			items: HashMap::new(),
		}
	}

	pub fn contains_item(&self, item: &Rc<Box<Item>>) -> bool {
		for val in self.items.values() {
			if (**item).get_id() == (*val).get_id() {
				return true;
			}
		}
		false
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

	pub fn write_out(&self) {
		if self.items.is_empty() {
			println!("There are currently no items in the inventory.");
		} else {
			println!("The inventory contains the following items:");
			for (key, val) in self.items.iter() {
				println!("\t{} [id={}]", (**val).get_longname(), key);
			}
		}
	}
}
