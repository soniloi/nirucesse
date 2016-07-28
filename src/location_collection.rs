use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use location::Location;
use data_collection;
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
const FILE_INDEX_LOCATION_DESCRIPTION: usize = 14;
const KEY_DIRECTION_NONE: u32 = 0;

const LOCATION_INDEX_WAKE: u32 = 9;

const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct LocationCollection {
	locations: HashMap<u32, Rc<RefCell<Box<Location>>>>,
	location_wake: u32, // Where the player wakes on game start, or after being reincarnated
}

impl LocationCollection {

	pub fn new() -> LocationCollection {
		LocationCollection {
			locations: HashMap::new(),
			location_wake: LOCATION_INDEX_WAKE,
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer) {

		let mut all_links: HashMap<u32, Box<HashMap<String, u32>>> = HashMap::new();
		let mut line = buffer.get_line();
		while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => break,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					let id = data_collection::str_to_u32(words[FILE_INDEX_LOCATION_ID], 10);
					let properties = data_collection::str_to_u32(words[FILE_INDEX_LOCATION_STATUS], 16);
					let shortname = String::from(words[FILE_INDEX_LOCATION_SHORTNAME]);
					let longname = String::from(words[FILE_INDEX_LOCATION_LONGNAME]);
					let description = String::from(words[FILE_INDEX_LOCATION_DESCRIPTION]);

					let loc = Rc::new(RefCell::new(Box::new(Location::new(id, properties, shortname, longname, description))));
					self.locations.insert(id, loc);

					let mut links: Box<HashMap<String, u32>> = Box::new(HashMap::new());
					links.insert(String::from("north"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_N], 10));
					links.insert(String::from("south"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_S], 10));
					links.insert(String::from("east"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_E], 10));
					links.insert(String::from("west"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_W], 10));
					links.insert(String::from("northeast"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_NE], 10));
					links.insert(String::from("southwest"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_SW], 10));
					links.insert(String::from("southeast"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_SE], 10));
					links.insert(String::from("northwest"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_NW], 10));
					links.insert(String::from("up"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_U], 10));
					links.insert(String::from("down"), data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_D], 10));
					all_links.insert(id, links.clone());
				},
			}
			line = buffer.get_line();
		}

		// Cross-reference all locations
		// FIXME: refactor
		for (loc_id, direction_map) in all_links.iter() {
			match self.get(*loc_id) {
				None => {
					println!("\x1b[31m[Warning: error cross-referencing location [{}]; giving up]\x1b[0m", *loc_id);
					return;
				},
				Some(loc) => {
					for (direction_key, direction_val) in (*direction_map).iter() {
						if *direction_val != KEY_DIRECTION_NONE {
							match self.get(*direction_val) {
								None => {
									println!("\x1b[31m[Warning: error cross-referencing location [{}]; giving up]\x1b[0m", *loc_id);
									return;
								},
								Some(direction) => {
									loc.borrow_mut().set_direction((*direction_key).clone(), (*direction).clone());
								},
							}
						}
					}
				},
			}
		}
	}

	pub fn get(&self, key: u32) -> Option<&Rc<RefCell<Box<Location>>>> {
		self.locations.get(&key)
	}

	pub fn get_location_wake(&self) -> Option<&Rc<RefCell<Box<Location>>>> {
		self.get(self.location_wake)
	}
}
