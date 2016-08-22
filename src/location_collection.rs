use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use location::Direction;
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

const LOCATION_INDEX_SAFE: u32 = 34;
const LOCATION_INDEX_WAKE: u32 = 9;

const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct LocationCollection {
	locations: HashMap<u32, Rc<RefCell<Box<Location>>>>,
	location_wake: u32, // Where player wakes on game start, or after being reincarnated
	location_safe: u32, // Where player's items get dropped on death
	direction_map: HashMap<&'static str, Direction>, // Map of direction strings to direction enum
}

impl LocationCollection {

	pub fn new() -> LocationCollection {
		LocationCollection {
			locations: HashMap::new(),
			location_wake: LOCATION_INDEX_WAKE,
			location_safe: LOCATION_INDEX_SAFE,
			direction_map: HashMap::new(),
		}
	}

	// TODO: make static
	fn init_direction_map(&mut self) {
		self.direction_map.insert("north", Direction::North);
		self.direction_map.insert("south", Direction::South);
		self.direction_map.insert("east", Direction::East);
		self.direction_map.insert("west", Direction::West);
		self.direction_map.insert("northeast", Direction::Northeast);
		self.direction_map.insert("southwest", Direction::Southwest);
		self.direction_map.insert("southeast", Direction::Southeast);
		self.direction_map.insert("northwest", Direction::Northwest);
		self.direction_map.insert("up", Direction::Up);
		self.direction_map.insert("down", Direction::Down);
		self.direction_map.insert("back", Direction::Back);
	}

	pub fn init(&mut self, buffer: &mut FileBuffer) {

		self.init_direction_map();

		let mut all_links: HashMap<u32, Box<HashMap<Direction, u32>>> = HashMap::new();
		let mut line = buffer.get_line();
		while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => break,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					// Create location and copy a reference into this collection
					let location_parsed = LocationCollection::parse_location(&words);
					let location = location_parsed.0;
					let id = location_parsed.1;
					self.locations.insert(id, location);

					let mut links: Box<HashMap<Direction, u32>> = Box::new(HashMap::new());
					links.insert(Direction::North, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_N], 10));
					links.insert(Direction::South, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_S], 10));
					links.insert(Direction::East, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_E], 10));
					links.insert(Direction::West, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_W], 10));
					links.insert(Direction::Northeast, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_NE], 10));
					links.insert(Direction::Southwest, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_SW], 10));
					links.insert(Direction::Southeast, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_SE], 10));
					links.insert(Direction::Northwest, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_NW], 10));
					links.insert(Direction::Up, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_U], 10));
					links.insert(Direction::Down, data_collection::str_to_u32(words[FILE_INDEX_LOCATION_DIRECTION_D], 10));
					all_links.insert(id, links);
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
									loc.borrow_mut().set_direction(*direction_key, (*direction).clone());
								},
							}
						}
					}
				},
			}
		}
	}

	fn parse_location(words: &Vec<&str>) -> (Rc<RefCell<Box<Location>>>, u32) {
		let id = data_collection::str_to_u32(words[FILE_INDEX_LOCATION_ID], 10);
		let properties = data_collection::str_to_u32(words[FILE_INDEX_LOCATION_STATUS], 16);
		let shortname = String::from(words[FILE_INDEX_LOCATION_SHORTNAME]);
		let longname = String::from(words[FILE_INDEX_LOCATION_LONGNAME]);
		let description = String::from(words[FILE_INDEX_LOCATION_DESCRIPTION]);

		let loc = Rc::new(RefCell::new(Box::new(Location::new(id, properties, shortname, longname, description))));
		(loc, id)
	}

	pub fn get(&self, key: u32) -> Option<&Rc<RefCell<Box<Location>>>> {
		self.locations.get(&key)
	}

	fn get_certain(&self, key: u32) -> &Rc<RefCell<Box<Location>>> {
		match self.locations.get(&key) {
			None => panic!("Unable to determine essential location [{}], fail.", key),
			Some(location) => return location,
		}
	}

	pub fn get_location_wake(&self) -> &Rc<RefCell<Box<Location>>> {
		self.get_certain(self.location_wake)
	}

	pub fn get_location_safe(&self) -> &Rc<RefCell<Box<Location>>> {
		self.get_certain(self.location_safe)
	}

	// Get a Direction from a string
	pub fn get_direction_enum(&self, dir_str: &str) -> &Direction {
		match self.direction_map.get(dir_str) {
		    None => panic!("Location collection corruption, fail."),
			Some(dir) => dir,
		}
	}
}
