use std::cell::RefCell;
use std::collections::HashMap;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::rc::Rc;

use constants;
use file_buffer::FileBuffer;
use command::Command;
use command_collection::CommandCollection;
use help_string_collection::HelpStringCollection;
use info_string_collection::InfoStringCollection;
use item::Item;
use item_collection::ItemCollection;
use location::Direction;
use location::Location;
use location_collection::LocationCollection;

pub type GenericRcBox<T> = Rc<Box<T>>;
pub type GenericRcRefCellBox<T> = Rc<RefCell<Box<T>>>;
pub type CommandRef = GenericRcBox<Command>;
pub type ItemRef = GenericRcRefCellBox<Item>;
pub type LocationRef = GenericRcRefCellBox<Location>;

pub struct DataCollection {
	commands: CommandCollection,
	items: ItemCollection,
	locations: LocationCollection,
	hints: HelpStringCollection,
	explanations: HelpStringCollection,
	responses: InfoStringCollection,
	puzzles: InfoStringCollection,
	events: InfoStringCollection,
	event_turns: HashMap<u32, u32>,
	tp_map_sleep: HashMap<u32, u32>,
	tp_map_witch: HashMap<u32, u32>,
	max_score: u32,
}

impl DataCollection {

	pub fn new() -> DataCollection {
		DataCollection {
			commands: CommandCollection::new(),
			items: ItemCollection::new(),
			locations: LocationCollection::new(),
			hints: HelpStringCollection::new(),
			explanations: HelpStringCollection::new(),
			responses: InfoStringCollection::new(),
			puzzles: InfoStringCollection::new(),
			events: InfoStringCollection::new(),
			event_turns: HashMap::new(),
			tp_map_sleep: HashMap::new(),
			tp_map_witch: HashMap::new(),
			max_score: 0u32,
		}
	}

	pub fn init(&mut self, mut buffer: &mut FileBuffer) {
		let mut treasure_count: u32 = 0;
		self.commands.init(&mut buffer);
		self.locations.init(&mut buffer, constants::EXPECTED_LOCATIONS);
		self.items.init(&mut buffer, constants::EXPECTED_ITEMS, &mut self.locations, &mut treasure_count);
		self.hints.init(&mut buffer);
		self.explanations.init(&mut buffer);
		self.responses.init(&mut buffer, constants::EXPECTED_STRINGS_RESPONSES, true);
		self.puzzles.init(&mut buffer, constants::EXPECTED_STRINGS_PUZZLES, true);
		self.events.init(&mut buffer, 0, false);

		self.init_event_turns();
		self.init_tp_maps();
		let achievement_count: u32 = self.puzzles.count_strings();
		self.max_score = treasure_count * constants::SCORE_TREASURE + achievement_count * constants::SCORE_PUZZLE;
	}

	// Assign a turn for an event to be printed on
	fn init_event_turns(&mut self) {
		let turn_bounds = Range::new(constants::MIN_MOVES_EVENT, constants::MAX_MOVES_EVENT);
		let mut rng = rand::thread_rng();
		let event_keys = self.events.get_keys();
		for event_key in event_keys {
			loop {
				let event_turn = turn_bounds.ind_sample(&mut rng);
				if !self.event_turns.contains_key(&event_turn) {
					self.event_turns.insert(event_turn, event_key);
					break;
				}
			}
		}
	}

	// Initialize teleport maps for sleep and witch rooms
	fn init_tp_maps(&mut self) {
		self.tp_map_sleep.insert(constants::LOCATION_ID_SLEEP_0, constants::LOCATION_ID_SLEEP_1);
		self.tp_map_sleep.insert(constants::LOCATION_ID_SLEEP_2, constants::LOCATION_ID_SLEEP_0);
		self.tp_map_witch.insert(constants::LOCATION_ID_WITCH_0, constants::LOCATION_ID_WITCH_1);
		self.tp_map_witch.insert(constants::LOCATION_ID_WITCH_1, constants::LOCATION_ID_WITCH_0);
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

	fn get_value_or_default<'a>(collection: &'a HelpStringCollection, key: &str) -> &'a str {
		match collection.get_uncertain(key) {
			None => collection.get_certain("default"),
			Some(value) => value
		}
	}

	pub fn get_response(&self, key: u32) -> &str {
		self.responses.get_certain(key)
	}

	// TODO: more than one parameter; make generic with get_response
	pub fn get_response_param(&self, key: u32, param: &str) -> String {
		let response = String::from(self.responses.get_certain(key));
		response.replace("$0", param)
	}

	pub fn get_puzzle(&self, key: u32) -> &str {
		self.puzzles.get_certain(key)
	}

	pub fn get_event(&self, turn: u32) -> Option<&str> {
		match self.event_turns.get(&turn) {
			None => None,
			Some(event_turn) => Some(self.events.get_certain(*event_turn)),
		}
	}

	pub fn get_commands_non_secret(&self) -> String {
		self.commands.mk_non_secret_string(self.get_response(19))
	}

	pub fn get_direction_enum(&self, dir_str: &str) -> &Direction {
		self.locations.get_direction_enum(dir_str)
	}

	pub fn get_tp_map_sleep(&self) -> &HashMap<u32, u32> {
		&self.tp_map_sleep
	}

	pub fn get_tp_map_witch(&self) -> &HashMap<u32, u32> {
		&self.tp_map_witch
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
