use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use data_collection;
use data_collection::ItemRef;
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
const FILE_INDEX_ITEM_ALIAS_START: usize = 8;
const ITEM_INDEX_START: u32 = 1000; // ID numbers before this index are used for locations, everything from here on for items
const ITEM_WRITING_NONE: &'static str = "0"; // String indicating that there is no writing

const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct ItemCollection {
	items_by_id: HashMap<u32, ItemRef>,
	items_by_name: HashMap<String, ItemRef>,
}

impl ItemCollection {

	pub fn new() -> ItemCollection {
		ItemCollection {
			items_by_id: HashMap::new(),
			items_by_name: HashMap::new(),
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer, locations: &mut LocationCollection, treasure_count: &mut u32) {

		let mut initial_locations: HashMap<u32, u32> = HashMap::new();
		let mut line = buffer.get_line();
		while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => break,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					// Create item and copy a reference into this collection
					let item_parsed = self.parse_and_insert_item(&words);
					let item = item_parsed.0;

					*treasure_count = *treasure_count + item.borrow().get_treasure_value();

					// Note item's starting location
					let initial = item_parsed.1;
					initial_locations.insert(item.borrow().get_id(), initial);
				},
			}
			line = buffer.get_line();
		}

		for (item_id, initial_id) in initial_locations {
			match self.get_by_id(item_id) {
				None => panic!("Unable to find item with ID: {}", item_id),
				Some(item) => self.set_initial(locations, item, initial_id),
			}
		}
	}

	fn parse_and_insert_item(&mut self, words: &Vec<&str>) -> (ItemRef, u32) {
		let id = data_collection::str_to_u32(words[FILE_INDEX_ITEM_ID], 10);
		let properties = data_collection::str_to_u32(words[FILE_INDEX_ITEM_STATUS], 16);
		let initial = data_collection::str_to_u32(words[FILE_INDEX_ITEM_INITIAL_LOC], 10);
		let size = data_collection::str_to_u32(words[FILE_INDEX_ITEM_SIZE], 10);
		let shortname = String::from(words[FILE_INDEX_ITEM_SHORTNAME]);
		let longname = String::from(words[FILE_INDEX_ITEM_LONGNAME]);
		let description = String::from(words[FILE_INDEX_ITEM_DESCRIPTION]);
		let writing: Option<String> = match words[FILE_INDEX_ITEM_WRITING] {
			ITEM_WRITING_NONE => None,
			writ => Some(String::from(writ)),
		};

		let item = Rc::new(RefCell::new(Box::new(Item::new(id, properties, size, shortname, longname, description, writing, initial))));
		self.items_by_id.insert(id, item.clone());
		self.items_by_name.insert(String::from(item.borrow().get_shortname()), item.clone());
		for i in FILE_INDEX_ITEM_ALIAS_START..words.len() {
			if !words[i].is_empty() {
				self.items_by_name.insert(String::from(words[i]), item.clone());
			}
		}

		(item, initial)
	}

	fn set_initial(&self, locations: &mut LocationCollection, item: &ItemRef, initial_id: u32) {
		// FIXME: tidy this up
		if initial_id <= ITEM_INDEX_START {
			let initial_loc = match locations.get(initial_id) {
				None => panic!("Unable to find location with ID: {}", initial_id),
				Some(loc) => loc,
			};
			initial_loc.borrow_mut().insert_item(item.clone(), true);
		} else {
			let initial_container = match self.get_by_id(initial_id) {
				None => panic!("Unable to find container with ID: {}", initial_id),
				Some(container) => container,
			};
			if !initial_container.borrow().is_container() {
				panic!("Item with ID: {} is not a container", initial_id);
			}
			if initial_container.borrow().is_container_liquid() && !item.borrow().is_liquid() ||
				!initial_container.borrow().is_container_liquid() && item.borrow().is_liquid() {
				panic!("Container with ID: {} is not the right kind of container for item: {}", initial_id, item.borrow().get_shortname());
			}
			initial_container.borrow_mut().set_within(Some(item.clone()));
		}
	}

	pub fn get_by_id(&self, key: u32) -> Option<&ItemRef> {
		self.items_by_id.get(&key)
	}

	pub fn get_by_name(&self, key: String) -> Option<&ItemRef> {
		self.items_by_name.get(&key)
	}
}
