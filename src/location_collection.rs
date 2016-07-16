use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use actions;
use location::Location;
use file_buffer::FileBuffer;

/*
const FILE_INDEX_LOCATION_INDEX: usize = 0;
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
*/

const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct LocationCollection {
	locations: HashMap<u32, Rc<RefCell<Box<Location>>>>,
}

impl LocationCollection {

	pub fn new() -> LocationCollection {
		LocationCollection {
			locations: HashMap::new(),
		}
	}

	pub fn put(&mut self, key: u32, val: Rc<RefCell<Box<Location>>>) {
		self.locations.insert(key, val);
	}

	pub fn get(&self, key: u32) -> Option<&Rc<RefCell<Box<Location>>>> {
		self.locations.get(&key)
	}
}
