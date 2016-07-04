use std::collections::HashMap;
use std::rc::Rc;

use item::Item;

pub struct ItemCollection<'a> {
	items: HashMap<&'a str, Rc<Box<Item>>>,
}

impl<'a> ItemCollection<'a> {

	pub fn new() -> ItemCollection<'a> {
		ItemCollection {
			items: HashMap::new(),
		}
	}

	pub fn put(&mut self, key: &'a str, val: Rc<Box<Item>>) {
		self.items.insert(key, val);
	}

	pub fn get(&self, key: &str) -> Option<&Rc<Box<Item>>> {
		self.items.get(key)
	}

	pub fn write_out(&self) {
		for (key, val) in self.items.iter() {
			print!("\t[{}]\t", key);
			(**val).write_out();
		}
	}
}