use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use item::Item;

pub struct Location {
	id: u64,
	status: u32,
	shortname: String,	
	longname: String,
	description: String,

	directions: HashMap<String, Rc<RefCell<Box<Location>>>>,
	items: HashMap<u64, Rc<Box<Item>>>,
}

impl Location {

	pub fn new(id: u64, status: u32, shortname: String, longname: String, description: String) -> Location {
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

	pub fn set_direction(&mut self, dir: String, loc: Rc<RefCell<Box<Location>>>) {
		self.directions.insert(dir, loc);
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

	pub fn write_out(&self) {
		println!("Location [id={}] [status={}] [shortname={}] [longname={}] [description={}] ", 
			self.id, self.status, self.shortname, self.longname, self.description);

		for (key, val) in self.directions.iter() {
			println!("\tTo the {} there is {}", key, (*val).borrow().get_stubname());
		}

		for val in self.items.values() {
			println!("\tThere is {} here [id={}]", (**val).get_longname(), (**val).get_id());
		}
	}
}
