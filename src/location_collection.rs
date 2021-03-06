use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use constants;
use data_collection::{self, LocationId, LocationRef};
use location::{Direction, Location};
use file_buffer::FileBuffer;

const FILE_INDEX_LOCATION_ID: usize = 0;
const FILE_INDEX_LOCATION_DIRECTION_N: usize = 1;
const FILE_INDEX_LOCATION_DIRECTION_S: usize = 2;
const FILE_INDEX_LOCATION_DIRECTION_E: usize = 3;
const FILE_INDEX_LOCATION_DIRECTION_W: usize = 4;
const FILE_INDEX_LOCATION_DIRECTION_NE: usize = 5;
const FILE_INDEX_LOCATION_DIRECTION_SW: usize = 6;
const FILE_INDEX_LOCATION_DIRECTION_SE: usize = 7;
const FILE_INDEX_LOCATION_DIRECTION_NW: usize = 8;
const FILE_INDEX_LOCATION_DIRECTION_U: usize = 9;
const FILE_INDEX_LOCATION_DIRECTION_D: usize = 10;
const FILE_INDEX_LOCATION_STATUS: usize = 11;
const FILE_INDEX_LOCATION_SHORTNAME: usize = 12;
const FILE_INDEX_LOCATION_LONGNAME: usize = 13;
const FILE_INDEX_LOCATION_DESCRIPTION_COMMON: usize = 14;
const FILE_INDEX_LOCATION_DESCRIPTION_SUFFIX_START: usize = 15;
const KEY_DIRECTION_NONE: u32 = 0;
const EXPECTED_DESCRIPTION_SUFFIXES: usize = 2;

pub struct LocationCollection {
	locations: HashMap<LocationId, LocationRef>,
}

impl Drop for LocationCollection {
	fn drop(&mut self) {
		for location in self.locations.values() {
			location.borrow_mut().remove_all_directions();
		}
	}
}

impl LocationCollection {

	pub fn new() -> LocationCollection {
		LocationCollection {
			locations: HashMap::new(),
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer, expected_count: u32) {
		let mut all_links: HashMap<LocationId, Box<HashMap<Direction, LocationId>>> = HashMap::new();
		let mut line = buffer.get_line();
		while !buffer.eof() {
			match line.as_ref() {
				constants::FILE_SECTION_SEPARATOR => break,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					// Create location and copy a reference into this collection
					let location_parsed = LocationCollection::parse_location(&words);
					let (location, id) = location_parsed;
					self.locations.insert(id, location);

					// Note links to adjacent locations
					let links = LocationCollection::parse_links(&words);
					all_links.insert(id, links);
				},
			}
			line = buffer.get_line();
		}

		// Use noted links to connect all adjacent locations to each other
		self.cross_reference(&all_links);

		self.validate(expected_count);
	}

	fn parse_location(words: &Vec<&str>) -> (LocationRef, LocationId) {
		let id = data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_ID], 10);
		let properties = data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_STATUS], 16);
		let shortname = String::from(words[FILE_INDEX_LOCATION_SHORTNAME]);
		let longname = String::from(words[FILE_INDEX_LOCATION_LONGNAME]);
		let description_common = String::from(words[FILE_INDEX_LOCATION_DESCRIPTION_COMMON]);

		let mut description_suffixes: Vec<String> = Vec::new();
		for i in FILE_INDEX_LOCATION_DESCRIPTION_SUFFIX_START..words.len() {
			if words[i].is_empty() {
				panic!("Error in location collection. Empty description suffix at index [{}] found for location with id [{}]", i, id);
			}
			description_suffixes.push(String::from(words[i]));
		}
		if description_suffixes.len() != 2 {
			panic!("Error in location collection. Expected [{}] description suffixes, but found [{}] for location with id [{}]", EXPECTED_DESCRIPTION_SUFFIXES, description_suffixes.len(), id);
		}

		let loc = Rc::new(RefCell::new(Box::new(Location::new(id, properties, shortname, longname,
			description_common, description_suffixes))));
		(loc, id)
	}

	fn parse_links(words: &Vec<&str>) -> Box<HashMap<Direction, LocationId>> {
		let mut links: Box<HashMap<Direction, LocationId>> = Box::new(HashMap::new());
		links.insert(Direction::North, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_N], 10));
		links.insert(Direction::South, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_S], 10));
		links.insert(Direction::East, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_E], 10));
		links.insert(Direction::West, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_W], 10));
		links.insert(Direction::Northeast, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_NE], 10));
		links.insert(Direction::Southwest, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_SW], 10));
		links.insert(Direction::Southeast, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_SE], 10));
		links.insert(Direction::Northwest, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_NW], 10));
		links.insert(Direction::Up, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_U], 10));
		links.insert(Direction::Down, data_collection::str_to_u32_certain(words[FILE_INDEX_LOCATION_DIRECTION_D], 10));
		links
	}

	fn cross_reference(&mut self, all_links: &HashMap<LocationId, Box<HashMap<Direction, LocationId>>>) {
		for (loc_id, direction_map) in all_links.iter() {
			let loc = self.get_certain(*loc_id);
			for (direction_key, direction_val) in (*direction_map).iter() {
				if *direction_val != KEY_DIRECTION_NONE {
					let adjacent_loc = self.get_certain(*direction_val);
					loc.borrow_mut().set_direction(*direction_key, Some((*adjacent_loc).clone()));
				}
			}
		}
	}

	// Ensure that all the necessary ids will be available
	fn validate(&self, expected_count: u32) {
		if self.locations.len() as u32 != expected_count {
			panic!("Error in location collection. Expected [{}] tags, found [{}]", expected_count, self.locations.len());
		}
		for id in 0..expected_count {
			if !self.locations.contains_key(&id) {
				panic!("Error in location collection. ID [{}] not found", id);
			}
		}
	}

	pub fn get(&self, key: LocationId) -> Option<&LocationRef> {
		self.locations.get(&key)
	}

	fn get_certain(&self, key: LocationId) -> &LocationRef {
		match self.locations.get(&key) {
			None => panic!("Location collection corruption for location id [{}], fail.", key),
			Some(location) => return location,
		}
	}
}
