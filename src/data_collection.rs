use std::cell::RefCell;
use std::rc::Rc;

use constants;
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
	puzzles: StringCollection,
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
			puzzles: StringCollection::new(),
			events: StringCollection::new(),
			max_score: 0u32,
		}
	}

	pub fn init(&mut self, mut buffer: &mut FileBuffer) {
		let mut treasure_count: u32 = 0;
		self.commands.init(&mut buffer);
		self.locations.init(&mut buffer);
		self.items.init(&mut buffer, &mut self.locations, &mut treasure_count);
		self.hints.init(&mut buffer);
		self.explanations.init(&mut buffer);
		self.responses.init(&mut buffer);
		self.puzzles.init(&mut buffer);
		self.events.init(&mut buffer);

		let achievement_count: u32 = self.puzzles.count_strings();
		self.max_score = treasure_count * constants::SCORE_TREASURE + achievement_count * constants::SCORE_PUZZLE;
	}

	pub fn get_command(&self, key: String) -> Option<&Rc<Box<Command>>> {
		self.commands.get(key)
	}

	pub fn get_item_by_name(&self, key: String) -> Option<&ItemRef> {
		self.items.get_by_name(key)
	}

	pub fn get_item_by_id_certain(&self, key: u32) -> &ItemRef {
		match self.items.get_by_id(key.clone()) {
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

	// TODO: more than one parameter; make generic with get_response
	pub fn get_response_param(&self, key: &str, param: &str) -> String {
		let response = String::from(self.responses.get_certain(key));
		response.replace("$0", param)
	}

	pub fn get_puzzle(&self, key: &str) -> &str {
		self.puzzles.get_certain(key)
	}

	pub fn get_event(&self, key: &str) -> Option<&String> {
		self.events.get_uncertain(key)
	}

	pub fn get_commands_non_secret(&self) -> String {
		self.commands.mk_non_secret_string(self.get_response("commands"))
	}

	pub fn get_direction_enum(&self, dir_str: &str) -> &Direction {
		self.locations.get_direction_enum(dir_str)
	}

	pub fn get_stowed_treasure_count(&self) -> u32 {
		let stowed_location = self.get_location_certain(constants::LOCATION_ID_TREASURESTORE);
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
