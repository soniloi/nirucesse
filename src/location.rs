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
}

pub struct Location {
	id: LocationId,
	properties: LocationProperties,
	shortname: String,
	longname: String,
	description_common: String,
	description_suffixes: Vec<String>,
	description_suffix_index: usize,
	visited: bool,
	directions: HashMap<Direction, LocationRef>,
	items: HashMap<ItemId, ItemRef>,
}

impl Location {

	pub fn new(id: LocationId, properties: LocationProperties, shortname: String, longname: String,
		description_common: String, description_suffixes: Vec<String>) -> Location {
		Location {
			id: id,
			properties: properties,
			shortname: shortname,
			longname: longname,
			description_common: description_common,
			description_suffixes: description_suffixes,
			description_suffix_index: constants::LOCATION_DESCRIPTION_SUFFIX_INDEX_DEFAULT,
			visited: false,
			directions: HashMap::new(),
			items: HashMap::new(),
		}
	}

	// Clear the entire direction map, removing all cross-references
	pub fn remove_all_directions(&mut self) {
		self.directions.clear();
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

	pub fn has_or_contains_with_property(&self, property_code_loc: LocationProperties, property_code_item: ItemProperties, on_optional: bool) -> bool {
		if self.has_property(property_code_loc) {
			return true;
		}
		self.contains_with_property(property_code_item, on_optional)
	}

	pub fn contains_with_property(&self, property_code: ItemProperties, on_optional: bool) -> bool {
		self.items.values().any(|x| x.borrow().has_or_contains_with_property(property_code, on_optional) && !x.borrow().has_property(constants::CTRL_ITEM_WEARABLE))
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
		self.directions.get(&dir).and_then(|next| Some(next.clone()))
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

	pub fn set_description_suffix_index(&mut self, index: usize) {
		self.description_suffix_index = index;
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

	fn mk_hot_string(&self, desc_hot: &str) -> String {
		if self.has_property(constants::CTRL_LOC_HOT) {
			return String::from("\n") + desc_hot;
		}
		String::new()
	}

	pub fn mk_arrival_string(&self, desc_start: &str, desc_hot: &str) -> String {
		match self.visited {
			true => self.mk_basic_string(desc_start) + "." + &self.mk_contents_string() + &self.mk_hot_string(desc_hot),
			_ => self.mk_full_string(desc_start, desc_hot),
		}
	}

	pub fn mk_full_string(&self, desc_start: &str, desc_hot: &str) -> String {
		self.mk_basic_string(desc_start) + &self.description_common +
			&self.description_suffixes[self.description_suffix_index] + &self.mk_contents_string() + &self.mk_hot_string(desc_hot)
	}
}
