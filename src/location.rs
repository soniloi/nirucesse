use std::collections::HashMap;

use item::Item;

pub struct Location {
	id: u64,
	status: u32,
	shortname: String,	
	longname: String,
	description: String,

	directions: HashMap<String, *mut Location>,
	items: HashMap<u64, *const Item>,
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

	pub fn set_direction(&mut self, dir: String, loc: *mut Location) {
		self.directions.insert(dir, loc);
	}

	pub fn insert_item(&mut self, item: *const Item) {
		unsafe { self.items.insert((*item).get_id(), item); }
	}

	pub fn remove_item(&mut self, item_ptr: *const Item) -> Option<*const Item> {
		self.write_out();
		unsafe {println!("Trying to remove item [id={}] from this location", (*item_ptr).get_id()); }
		unsafe {println!("Trying to remove item [name=\"{}\"] from this location", (*item_ptr).get_longname()); }
		unsafe {println!("Trying to remove item [name=\"{}\"] [id={}] from this location", (*item_ptr).get_longname(), (*item_ptr).get_id()); }
		unsafe {self.items.remove(&(*item_ptr).get_id()) }
	}

	pub fn get_stubname(&self) -> &str {
		&self.shortname
	}

	pub fn write_out(&self) {
		println!("Location [id={}] [status={}] [shortname={}] [longname={}] [description={}] ", 
			self.id, self.status, self.shortname, self.longname, self.description);

		for (key, val) in self.directions.iter() {
			unsafe {
				println!("\tTo the {} there is {}", key, (**val).get_stubname());
			}
		}

		for val in self.items.values() {
			unsafe { println!("\tThere is {} here [id={}]", (**val).get_longname(), (**val).get_id()); }
		}
	}
}
