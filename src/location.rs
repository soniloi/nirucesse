use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use item::Item;

const CTRL_LOC_HAS_LIGHT: u32 = 0x01; // Whether the location has ambient lighting
const CTRL_LOC_HAS_AIR: u32 = 0x2; // Whether there is air at the location
const CTRL_LOC_NEEDSNO_LIGHT: u32 = 0x10; // Whether the location requires no portable lighting in order for the player to be able to see clearly

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
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
	Back,
}

pub struct Location {
	id: u32,
	properties: u32,
	shortname: String,
	longname: String,
	description: String,

	directions: HashMap<Direction, Rc<RefCell<Box<Location>>>>,
	items: HashMap<u32, Rc<Box<Item>>>,
}

impl Location {

	pub fn new(id: u32, properties: u32, shortname: String, longname: String, description: String) -> Location {
		Location {
			id: id,
			properties: properties,
			shortname: shortname,
			longname: longname,
			description: description,
			directions: HashMap::with_capacity(10),
			items: HashMap::new(),
		}
	}

	pub fn get_id(&self) -> u32 {
		self.id
	}

	pub fn is(&self, id: u32) -> bool {
		id == self.id
	}

	fn has_property(&self, property: u32) -> bool {
		self.properties & property != 0
	}

	fn set_property(&mut self, property: u32) {
		self.properties |= property;
	}

	fn unset_property(&mut self, property: u32) {
		self.properties &= !property;
	}

	pub fn has_light(&self) -> bool {
		// First check whether the location has ambient light
		if self.has_property(CTRL_LOC_HAS_LIGHT) {
			return true
		}

		// Next check whether any items at location emit light
		for item in self.items.values() {
			if item.has_light() {
				return true
			}
		}

		false
	}

	pub fn has_air(&self) -> bool {
		self.has_property(CTRL_LOC_HAS_AIR)
	}

	pub fn set_air(&mut self, on: bool) {
		if on {
			self.set_property(CTRL_LOC_HAS_AIR);
		} else {
			self.unset_property(CTRL_LOC_HAS_AIR);
		}
	}

	pub fn needsno_light(&self) -> bool {
		self.has_property(CTRL_LOC_NEEDSNO_LIGHT)
	}

	pub fn get_obstruction(&self) -> Option<Rc<Box<Item>>> {
		for item in self.items.values() {
			if item.is_obstruction() {
				return Some(item.clone());
			}
		}

		None
	}

	pub fn get_direction(&self, dir: &Direction) -> Option<&Rc<RefCell<Box<Location>>>> {
		self.directions.get(dir)
	}

	pub fn set_direction(&mut self, dir: Direction, loc: Rc<RefCell<Box<Location>>>) {
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

	pub fn remove_item_certain(&mut self, item: &Rc<Box<Item>>) {
		match self.items.remove(&(*item).get_id()) {
			None => panic!("Error: Location or item [{}] corrupt.", item.get_shortname()),
			Some(_) => {},
		}
	}

	pub fn get_shortname(&self) -> String {
		self.shortname.clone()
	}

	// Return whether another location can be reached in one step from this one
	pub fn can_reach(&self, other: &Rc<RefCell<Box<Location>>>) -> bool {
		for dir in self.directions.values() {
			if dir.borrow().get_id() == other.borrow().get_id() {
				return true;
			}
		}

		false
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
