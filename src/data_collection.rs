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

pub type GenericRcBox<T> = Rc<Box<T>>;
pub type GenericRcRefCellBox<T> = Rc<RefCell<Box<T>>>;
pub type CommandRef = GenericRcBox<Command>;
pub type ItemRef = GenericRcRefCellBox<Item>;
pub type LocationRef = GenericRcRefCellBox<Location>;

pub struct DataCollection {
	commands: CommandCollection,
	items: ItemCollection,
	locations: LocationCollection,
	hints: StringCollection,
	explanations: StringCollection,
	responses: StringCollection,
	events: StringCollection,
	max_score: u32,
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
			max_score: 0u32,
		}
	}

	pub fn init(&mut self, mut buffer: &mut FileBuffer) {
		let mut treasure_count: u32 = 0;
		let mut achievement_count: u32 = 0;
		self.commands.init(&mut buffer);
		self.locations.init(&mut buffer);
		self.items.init(&mut buffer, &mut self.locations, &mut treasure_count);
		self.hints.init(&mut buffer, &mut achievement_count);
		self.explanations.init(&mut buffer, &mut achievement_count);
		self.responses.init(&mut buffer, &mut achievement_count);
		self.events.init(&mut buffer, &mut achievement_count);
		self.max_score = treasure_count * ::SCORE_TREASURE + achievement_count * ::SCORE_PUZZLE;
	}

	pub fn get_command(&self, key: String) -> Option<&Rc<Box<Command>>> {
		self.commands.get(key)
	}

	pub fn get_item(&self, key: String) -> Option<&ItemRef> {
		self.items.get(key)
	}

	pub fn get_item_certain(&self, key: String) -> &ItemRef {
		match self.items.get(key.clone()) {
			None => panic!("Error: Data collection corrupt when searching for item [{}].", key),
			Some(item) => return item,
		}
	}

	pub fn get_location(&self, key: u32) -> Option<&LocationRef> {
		self.locations.get(key)
	}

	pub fn get_location_certain(&self, key: u32) -> &LocationRef {
		match self.locations.get(key) {
			None => panic!("Error: Data collection corrupt when searching for location [{}].", key),
			Some(loc) => return loc,
		}
	}

	pub fn get_location_wake(&self) -> &LocationRef {
		self.locations.get_location_wake()
	}

	pub fn get_location_safe(&self) -> &LocationRef {
		self.locations.get_location_safe()
	}

	pub fn get_hint(&self, key: &str) -> Option<&String> {
		self.hints.get_uncertain(key)
	}

	pub fn get_hint_certain(&self, key: &str) -> &str {
		self.hints.get_certain(key)
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

	pub fn get_direction_enum(&self, dir_str: &str) -> &Direction {
		self.locations.get_direction_enum(dir_str)
	}

	pub fn get_stowed_treasure_count(&self) -> u32 {
		let stowed_location = self.get_location_certain(::LOCATION_ID_TREASURESTORE);
		stowed_location.borrow().get_treasure_count()
	}

	pub fn get_max_score(&self) -> u32 {
		self.max_score
	}
}

pub fn str_to_u32(st: &str, radix: u32) -> u32 {
	match u32::from_str_radix(st, radix) {
		Err(why) => panic!("Unable to parse integer field {}: {}", st, why),
		Ok(status) => status,
	}
}
