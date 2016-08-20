use std::cell::RefCell;
use std::rc::Rc;

use file_buffer::FileBuffer;
use command::Command;
use command_collection::CommandCollection;
use item::Item;
use item_collection::ItemCollection;
use location::Direction;
use location::Location;
use location_collection::LocationCollection;
use string_collection::StringCollection;

pub struct DataCollection {
	commands: CommandCollection,
	items: ItemCollection,
	locations: LocationCollection,
	hints: StringCollection,
	explanations: StringCollection,
	responses: StringCollection,
	events: StringCollection,
}

impl DataCollection {

	pub fn new() -> DataCollection {
		DataCollection {
			commands: CommandCollection::new(),
			items: ItemCollection::new(),
			locations: LocationCollection::new(),
			hints: StringCollection::new(),
			explanations: StringCollection::new(),
			responses: StringCollection::new(),
			events: StringCollection::new(),
		}
	}

	pub fn init(&mut self, mut buffer: &mut FileBuffer) {
		self.commands.init(&mut buffer);
		self.locations.init(&mut buffer);
		self.items.init(&mut buffer, &mut self.locations);
		self.hints.init(&mut buffer);
		self.explanations.init(&mut buffer);
		self.responses.init(&mut buffer);
		self.events.init(&mut buffer);
	}

	pub fn get_command(&self, key: String) -> Option<&Rc<Box<Command>>> {
		self.commands.get(key)
	}

	pub fn get_item(&self, key: String) -> Option<&Rc<Box<Item>>> {
		self.items.get(key)
	}

	pub fn get_item_certain(&self, key: String) -> &Rc<Box<Item>> {
		match self.items.get(key.clone()) {
			None => panic!("Error: Data collection corrupt when searching for item [{}].", key),
			Some(item) => return item,
		}
	}

	pub fn get_location(&self, key: u32) -> Option<&Rc<RefCell<Box<Location>>>> {
		self.locations.get(key)
	}

	pub fn get_location_certain(&self, key: u32) -> &Rc<RefCell<Box<Location>>> {
		match self.locations.get(key) {
			None => panic!("Error: Data collection corrupt when searching for location [{}].", key),
			Some(loc) => return loc,
		}
	}

	pub fn get_location_wake(&self) -> &Rc<RefCell<Box<Location>>> {
		self.locations.get_location_wake()
	}

	pub fn get_location_safe(&self) -> &Rc<RefCell<Box<Location>>> {
		self.locations.get_location_safe()
	}

	pub fn get_hint(&self, key: &str) -> &str {
		DataCollection::get_value_or_default(&self.hints, key)
	}

	pub fn get_explanation(&self, key: &str) -> &str {
		DataCollection::get_value_or_default(&self.explanations, key)
	}

	fn get_value_or_default<'a>(collection: &'a StringCollection, key: &str) -> &'a str {
		match collection.get_uncertain(key) {
			None => collection.get_certain("default"),
			Some(value) => value
		}
	}

	pub fn get_response(&self, key: &str) -> &str {
		self.responses.get_certain(key)
	}

	pub fn get_event(&self, key: &str) -> Option<&String> {
		self.events.get_uncertain(key)
	}

	pub fn get_commands_non_secret(&self) -> String {
		self.commands.mk_non_secret_string()
	}

	pub fn get_direction_enum(&self, dir_str: String) -> &Direction {
		self.locations.get_direction_enum(dir_str)
	}
}

pub fn str_to_u32(st: &str, radix: u32) -> u32 {
	match u32::from_str_radix(st, radix) {
		Err(why) => panic!("Unable to parse integer field {}: {}", st, why),
		Ok(status) => status,
	}
}
