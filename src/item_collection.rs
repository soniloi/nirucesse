use std::collections::HashMap;

use item::Item;

pub struct ItemCollection<'a> {
	items: HashMap<&'a str, *const Item>,
}

impl<'a> ItemCollection<'a> {

	pub fn new() -> ItemCollection<'a> {
		ItemCollection {
			items: HashMap::new(),
		}
	}

	pub fn put(&mut self, key: &'a str, val: *const Item) {
		self.items.insert(key, val);
	}

	pub fn get(&self, key: &str) -> Option<&*const Item> {
		self.items.get(key)
	}

	pub fn write_out(&self) {
		for (key, val) in self.items.iter() {
			unsafe {
				print!("\t[{}]\t", key);
				(**val).write_out();
			}
		}
	}
}