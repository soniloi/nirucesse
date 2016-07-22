use std::cell::RefCell;
use std::rc::Rc;

use file_buffer::FileBuffer;
use command::Command;
use command_collection::CommandCollection;
use item::Item;
use item_collection::ItemCollection;
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

	pub fn get_location(&self, key: u32) -> Option<&Rc<RefCell<Box<Location>>>> {
		self.locations.get(key)
	}

	pub fn get_hint(&self, key: String) -> Option<&String> {
		self.hints.get(key)
	}

	pub fn get_explanation(&self, key: String) -> Option<&String> {
		self.explanations.get(key)
	}

	pub fn get_response(&self, key: String) -> Option<&String> {
		self.responses.get(key)
	}

	pub fn get_event(&self, key: String) -> Option<&String> {
		self.events.get(key)
	}

	// TODO: remove when replaced
	pub fn get_items(&self) -> &ItemCollection {
		&self.items
	}
}