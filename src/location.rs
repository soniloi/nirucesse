use std::collections::HashMap;

use data_collection::ItemRef;
use data_collection::LocationRef;

const CTRL_LOC_HAS_LIGHT: u32 = 0x01; // Whether the location has ambient lighting
const CTRL_LOC_HAS_AIR: u32 = 0x2; // Whether there is air at the location
const CTRL_LOC_HAS_GRAVITY: u32 = 0x4; // Whether there is gravity at the location
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

	directions: HashMap<Direction, LocationRef>,
	items: HashMap<u32, ItemRef>,
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
		self.has_light_item()
	}

	// Return whether any item resting at this location emits light
	pub fn has_light_item(&self) -> bool {
		self.items.values().any(|x| x.borrow().has_light())
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

	pub fn has_gravity(&self) -> bool {
		self.has_property(CTRL_LOC_HAS_GRAVITY)
	}

	pub fn needsno_light(&self) -> bool {
		self.has_property(CTRL_LOC_NEEDSNO_LIGHT)
	}

	pub fn get_obstruction(&self) -> Option<ItemRef> {
		for item in self.items.values() {
			if item.borrow().is_obstruction() {
				return Some(item.clone());
			}
		}
		None
	}

	pub fn get_direction(&self, dir: &Direction) -> Option<&LocationRef> {
		self.directions.get(dir)
	}

	pub fn set_direction(&mut self, dir: Direction, loc: LocationRef) {
		self.directions.insert(dir, loc);
	}

	pub fn contains_item(&self, id: u32) -> bool {
		self.items.values().any(|x| x.borrow().is_or_contains_item(id))
	}

	pub fn insert_item(&mut self, item: ItemRef) {
		self.items.insert(item.borrow().get_id(), item.clone());
	}

	// FIXME: clean up the flow here
	pub fn remove_item_certain(&mut self, id: u32) {
		match self.items.get(&id) {
			None => {},
			Some(item) => {
				// Liquids don't get removed ONLY if they were at a location and not within a container
				if item.borrow().is_liquid() {
					return;
				}
			}
		}
		for item in self.items.values() {
			if item.borrow().contains_item(id) {
				item.borrow_mut().remove_item_certain(id);
				return;
			}
		}
		self.items.remove(&id);
	}

	pub fn get_shortname(&self) -> String {
		self.shortname.clone()
	}

	// Return whether another location can be reached in one step from this one
	pub fn can_reach(&self, other: &LocationRef) -> bool {
		let other_id = other.borrow().get_id();
		self.directions.values().any(|x| x.borrow().get_id() == other_id)
	}

	// Return the number of treasures at this location
	pub fn get_treasure_count(&self) -> u32 {
		self.items.values().fold(0, |acc, x| acc + x.borrow().get_treasure_value())
	}

	fn mk_basic_string(&self) -> String {
		String::from("You are ") + &self.longname
	}

	pub fn mk_full_string(&self) -> String {
		let mut result = self.mk_basic_string() + &self.description;
		for item in self.items.values() {
			result = result + &item.borrow().get_locationname();
		}
		result
	}
}
