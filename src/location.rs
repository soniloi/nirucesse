use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use item::Item;

pub struct Location {
	id: u32,
	status: u32,
	shortname: String,	
	longname: String,
	description: String,

	directions: HashMap<String, Rc<RefCell<Box<Location>>>>,
	items: HashMap<u64, Rc<Box<Item>>>,
}

impl Location {

	pub fn new(id: u32, status: u32, shortname: String, longname: String, description: String) -> Location {
		Location {
			id: id,
			status: status,
			shortname: shortname,
			longname: longname,
			description: description,
			directions: HashMap::with_capacity(11),
			items: HashMap::new(),
		}
	}

	pub fn get_id(&self) -> u32 {
		self.id
	}

	pub fn get_direction(&self, dir: String) -> Option<&Rc<RefCell<Box<Location>>>> {
		self.directions.get(&dir)
	}

	pub fn set_direction(&mut self, dir: String, loc: Rc<RefCell<Box<Location>>>) {
		self.directions.insert(dir, loc);
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

	pub fn get_stubname(&self) -> &str {
		&self.shortname
	}

	fn mk_basic_string(&self) -> String {
		String::from("You are ") + &self.longname
	}

	pub fn mk_full_string(&self) -> String {
		let mut result = self.mk_basic_string();
		result = result + &self.description + ".";
		if !self.items.is_empty() {
			for item in self.items.values() {
				result = result + "\nThere is " + item.get_longname() + " here.";
			}
		}

		result
	}
}
