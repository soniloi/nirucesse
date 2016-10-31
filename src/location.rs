use std::collections::HashMap;

use constants;
use data_collection::ItemRef;
use data_collection::LocationRef;
use inventory::Inventory;

const CTRL_LOC_HAS_LIGHT: u32 = 0x01; // Whether the location has ambient lighting
const CTRL_LOC_HAS_AIR: u32 = 0x2; // Whether there is air at the location
const CTRL_LOC_HAS_GRAVITY: u32 = 0x4; // Whether there is gravity at the location
const CTRL_LOC_HAS_NOSNOMP: u32 = 0x8; // Whether there is absence of snomps at the location
const CTRL_LOC_NEEDSNO_LIGHT: u32 = 0x10; // Whether the location requires no portable lighting in order for the player to be able to see clearly
const CTRL_LOC_NEEDSNO_GRAVITY: u32 = 0x40; // Whether the location requires that there be no gravity
const CTRL_LOC_HAS_CEILING: u32 = 0x100; // Whether there is a ceiling to this location, or something above it
const CTRL_LOC_HAS_FLOOR: u32 = 0x200; // Whether there is a floor at this location
const CTRL_LOC_HAS_LAND: u32 = 0x400; // Whether the location has land, as opposed to open water

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
	Out,
}

pub struct Location {
	id: u32,
	properties: u32,
	shortname: String,
	longname: String,
	description: String,
	visited: bool,
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
			visited: false,
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

	pub fn set_visited(&mut self, vis: bool) {
		self.visited = vis;
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

	pub fn set_gravity(&mut self, on: bool) {
		if on {
			self.set_property(CTRL_LOC_HAS_GRAVITY);
		} else {
			self.unset_property(CTRL_LOC_HAS_GRAVITY);
		}
	}

	pub fn has_nosnomp(&self) -> bool {
		self.has_property(CTRL_LOC_HAS_NOSNOMP)
	}

	pub fn needsno_light(&self) -> bool {
		self.has_property(CTRL_LOC_NEEDSNO_LIGHT)
	}

	pub fn needsno_gravity(&self) -> bool {
		self.has_property(CTRL_LOC_NEEDSNO_GRAVITY)
	}

	pub fn has_ceiling(&self) -> bool {
		self.has_property(CTRL_LOC_HAS_CEILING)
	}

	pub fn has_floor(&self) -> bool {
		self.has_property(CTRL_LOC_HAS_FLOOR)
	}

	pub fn has_land(&self) -> bool {
		self.has_property(CTRL_LOC_HAS_LAND)
	}

	pub fn get_obstruction(&self) -> Option<ItemRef> {
		for item in self.items.values() {
			if item.borrow().has_property(constants::CTRL_ITEM_OBSTRUCTION) {
				return Some(item.clone());
			}
		}
		None
	}

	pub fn get_direction(&self, dir: &Direction) -> Option<&LocationRef> {
		self.directions.get(dir)
	}

	// FIXME: tidy the flow here
	fn determine_out(&self) -> Option<LocationRef> {
		let mut direction_iter = self.directions.iter();
		let direction_opt = direction_iter.next();
		match direction_opt {
			None => return None,
			Some (direction) => {
				match direction_iter.next() {
					None => return Some(direction.1.clone()),
					Some(_) => return None,
				}
			}
		}
	}

	pub fn set_direction(&mut self, dir: Direction, next: Option<LocationRef>) {
		match next {
			None => {self.directions.remove(&dir);},
			Some(loc) => {self.directions.insert(dir, loc);},
		}
		let next_out = self.determine_out();
		match next_out {
			None => {self.directions.remove(&Direction::Out);},
			Some(out) => {self.directions.insert(Direction::Out, out);},
		}
	}

	pub fn contains_item(&self, id: u32) -> bool {
		self.items.values().any(|x| x.borrow().is_or_contains_item(id))
	}

	pub fn insert_item(&mut self, item: ItemRef, permanent: bool) {
		item.borrow_mut().set_location_true(self.id);
		if permanent {
			item.borrow_mut().set_location_stated(self.id);
		}
		self.items.insert(item.borrow().get_id(), item.clone());
	}

	// Release items that are marked as still in inventory, i.e. only at this location temporarily
	pub fn release_temporary(&mut self, inventory: &mut Inventory) {
		let mut to_remove: Vec<ItemRef> = Vec::new();
		for item in self.items.values() {
			if item.borrow().get_location_stated() == constants::LOCATION_ID_INVENTORY {
				to_remove.push(item.clone());
			}
		}
		for item in to_remove {
			self.items.remove(&item.borrow().get_id());
			inventory.insert_item(item);
		}
	}

	// FIXME: clean up the flow here
	pub fn remove_item_certain(&mut self, id: u32) {
		match self.items.get(&id) {
			None => {},
			Some(item) => {
				// Liquids don't get removed ONLY if they were at a location and not within a container
				if item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
					return;
				}
				item.borrow_mut().retire();
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

	fn mk_contents_string(&self) -> String {
		self.items.values().fold(String::new(), |acc, x| acc + &x.borrow().get_locationname())
	}

	pub fn mk_arrival_string(&self) -> String {
		if self.visited {
			return self.mk_basic_string() + "." + &self.mk_contents_string();
		}
		self.mk_full_string()
	}

	pub fn mk_full_string(&self) -> String {
		self.mk_basic_string() + &self.description + &self.mk_contents_string()
	}
}
