use std::collections::HashMap;
use std::rc::Rc;

use file_buffer::FileBuffer;
use item::Item;
use location_collection::LocationCollection;

const FILE_INDEX_ITEM_ID: usize = 0;
const FILE_INDEX_ITEM_STATUS: usize = 1;
const FILE_INDEX_ITEM_INITIAL_LOC: usize = 2;
const FILE_INDEX_ITEM_SIZE: usize = 3;
const FILE_INDEX_ITEM_SHORTNAME: usize = 4;
const FILE_INDEX_ITEM_LONGNAME: usize = 5;
const FILE_INDEX_ITEM_DESCRIPTION: usize = 6;
const FILE_INDEX_ITEM_WRITING: usize = 7;
//const ITEM_INDEX_START: usize = 1000; // ID numbers before this index are used for locations, everything from here on for items
const ITEM_WRITING_NONE: &'static str = "0"; // String indicating that there is no writing

const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct ItemCollection {
	items: HashMap<String, Rc<Box<Item>>>,
}

impl ItemCollection {

	pub fn new() -> ItemCollection {
		ItemCollection {
			items: HashMap::new(),
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer, locations: &mut LocationCollection) {

		let mut line = buffer.get_line();
		while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => break,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					// Create item
					let id = str_to_u32(words[FILE_INDEX_ITEM_ID], 10);
					let properties = str_to_u32(words[FILE_INDEX_ITEM_STATUS], 16);
					let initial = str_to_u32(words[FILE_INDEX_ITEM_INITIAL_LOC], 10);
					let size = str_to_u32(words[FILE_INDEX_ITEM_SIZE], 10);
					let shortname = String::from(words[FILE_INDEX_ITEM_SHORTNAME]);
					let longname = String::from(words[FILE_INDEX_ITEM_LONGNAME]);
					let description = String::from(words[FILE_INDEX_ITEM_DESCRIPTION]);
					let writing = match words[FILE_INDEX_ITEM_WRITING] {
						ITEM_WRITING_NONE => String::from(""),
						writ => String::from(writ),
					};

					let item = Rc::new(Box::new(Item::new(id, properties, size, shortname.clone(), longname, description, writing)));
					self.items.insert(shortname, item.clone());

					// Point item's starting location at it
					let initial_loc = match locations.get(initial) {
						None => panic!("Unable to find location with ID: {}", initial),
						Some(loc) => {
							loc
						},
					};
					initial_loc.borrow_mut().insert_item(item);
				},
			}
			line = buffer.get_line();
		}
	}

	pub fn get(&self, key: String) -> Option<&Rc<Box<Item>>> {
		self.items.get(&key)
	}
}

fn str_to_u32(st: &str, radix: u32) -> u32 {
	match u32::from_str_radix(st, radix) {
		Err(why) => panic!("Unable to parse integer field {}: {}", st, why),
		Ok(status) => status,
	}
}
