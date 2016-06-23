use std::cmp::Eq;
use std::collections::HashMap;

use item::Item;

#[derive(Hash)]
enum Direction {
	North,
	Northeast,
	East,
	Southeast,
	South,
	Southwest,
	West,
	Northwest,
	Up,
	Down,
	Out,
}

impl Eq for Direction {}

impl PartialEq for Direction {
	fn eq(&self, other: &Direction) -> bool {
		self == other
	}
}

pub struct Location {
	id: u64,
	status: u32,
	shortname: String,	
	longname: String,
	description: String,

	directions: HashMap<Direction, Location>,
	items: HashMap<u32, Item>,
}

impl Location {

	pub fn new(id: u64, status: u32, shortname: String, longname: String, description: String) -> Location {
		Location {
			id: id,
			status: status,
			shortname: shortname,
			longname: longname,
			description: description,
			directions: HashMap::new(),
			items: HashMap::new(),
		}
	}

	pub fn write_out(&self) {
		println!("Location [id={}] [status={}] [shortname={}] [longname={}] [description={}] ", 
			self.id, self.status, self.shortname, self.longname, self.description);
	}
}
