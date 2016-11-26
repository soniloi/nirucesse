use std::collections::HashMap;

use constants;
use data_collection::{ItemId, ItemProperties, ItemRef, LocationId, LocationProperties, LocationRef};

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
	Max,
}

pub struct Location {
	id: LocationId,
	properties: LocationProperties,
	shortname: String,
	longname: String,
	description: String,
	visited: bool,
	directions: HashMap<Direction, LocationRef>,
	items: HashMap<ItemId, ItemRef>,
}

pub type PropertyWithinFn = fn(location: &Location, property_code: ItemProperties) -> bool;

impl Location {

	pub fn new(id: LocationId, properties: LocationProperties, shortname: String, longname: String, description: String) -> Location {
		Location {
			id: id,
			properties: properties,
			shortname: shortname,
			longname: longname,
			description: description,
			visited: false,
			directions: HashMap::with_capacity(Direction::Max as usize),
			items: HashMap::new(),
		}
	}

	pub fn get_id(&self) -> LocationId {
		self.id
	}

	pub fn is(&self, id: LocationId) -> bool {
		id == self.id
	}

	pub fn set_visited(&mut self, vis: bool) {
		self.visited = vis;
	}

	pub fn has_property(&self, property_code: LocationProperties) -> bool {
		self.properties & property_code != 0
	}

	fn has_or_contains_with_property_generic(&self, property_code_loc: LocationProperties, property_code_item: ItemProperties, next: PropertyWithinFn) -> bool {
		if self.has_property(property_code_loc) {
			return true;
		}
		next(self, property_code_item)
	}

	pub fn has_or_contains_with_property(&self, property_code_loc: LocationProperties, property_code_item: ItemProperties) -> bool {
		self.has_or_contains_with_property_generic(property_code_loc, property_code_item, Location::contains_with_property)
	}

	pub fn has_or_contains_with_switchable_property(&self, property_code_loc: LocationProperties, property_code_item: ItemProperties) -> bool {
		self.has_or_contains_with_property_generic(property_code_loc, property_code_item, Location::contains_with_switchable_property)
	}

	pub fn contains_with_property(&self, property_code: ItemProperties) -> bool {
		self.items.values().any(|x| x.borrow().has_or_contains_with_property(property_code) && !x.borrow().has_property(constants::CTRL_ITEM_WEARABLE))
	}

	pub fn contains_with_switchable_property(&self, property_code: ItemProperties) -> bool {
		self.items.values().any(|x| x.borrow().has_or_contains_with_switchable_property(property_code) && !x.borrow().has_property(constants::CTRL_ITEM_WEARABLE))
	}

	pub fn set_property(&mut self, property_code: LocationProperties, next: bool) {
		if next {
			self.properties |= property_code;
		} else {
			self.properties &= !property_code;
		}
	}

	pub fn get_obstruction(&self) -> Option<ItemRef> {
		for item in self.items.values() {
			if item.borrow().has_property(constants::CTRL_ITEM_OBSTRUCTION) {
				return Some(item.clone());
			}
		}
		None
	}

	pub fn get_direction(&self, dir: Direction) -> Option<LocationRef> {
		match self.directions.get(&dir) {
			Some(next) => Some(next.clone()),
			_ => None,
		}
	}

	// Return the only direction one can go from here, if it exists; return None if there are multiple possible directions or none
	fn determine_out(&self) -> Option<LocationRef> {
		let mut direction_iter = self.directions.iter();
		if let Some(direction) = direction_iter.next() {
			if let None = direction_iter.next() {
				return Some(direction.1.clone());
			}
		}
		None
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

	pub fn contains_item(&self, id: ItemId) -> bool {
		self.items.values().any(|x| x.borrow().is_or_contains_item(id))
	}

	pub fn insert_item(&mut self, item: ItemRef) {
		item.borrow_mut().set_location(self.id);
		self.items.insert(item.borrow().get_id(), item.clone());
	}

	// FIXME: clean up the flow here
	pub fn remove_item_certain(&mut self, id: ItemId) {
		if let Some(item) = self.items.get(&id) {
			// Liquids don't get removed ONLY if they were at a location and not within a container
			if item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
				return;
			}
			item.borrow_mut().retire();
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

	fn mk_basic_string(&self, desc_start: &str) -> String {
		String::from(desc_start) + &self.longname
	}

	fn mk_contents_string(&self) -> String {
		self.items.values().fold(String::new(), |acc, x| acc + &x.borrow().get_locationname())
	}

	pub fn mk_arrival_string(&self, desc_start: &str) -> String {
		if self.visited {
			return self.mk_basic_string(desc_start) + "." + &self.mk_contents_string();
		}
		self.mk_full_string(desc_start)
	}

	pub fn mk_full_string(&self, desc_start: &str) -> String {
		self.mk_basic_string(desc_start) + &self.description + &self.mk_contents_string()
	}
}
