use std::cell::RefCell;
use std::collections::HashMap;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::num::ParseIntError;
use std::rc::Rc;

use command::Command;
use command_collection::CommandCollection;
use constants;
use file_buffer::FileBuffer;
use help_string_collection::HelpStringCollection;
use info_string_collection::InfoStringCollection;
use inventory::Inventory;
use item::Item;
use item_collection::ItemCollection;
use location::{Direction, Location};
use location_collection::LocationCollection;

pub type GenericRcBox<T> = Rc<Box<T>>;
pub type GenericRcRefCellBox<T> = Rc<RefCell<Box<T>>>;
pub type CommandRef = GenericRcBox<Command>;
pub type InventoryRef = GenericRcRefCellBox<Inventory>;
pub type ItemRef = GenericRcRefCellBox<Item>;
pub type LocationRef = GenericRcRefCellBox<Location>;
pub type Id = u32;
pub type CommandId = Id;
pub type InventoryId = Id;
pub type ItemId = Id;
pub type LocationId = Id;
pub type StringId = Id;
pub type TpMap = HashMap<LocationId, (LocationId, InventoryId)>;
pub type Properties = u32;
pub type CommandProperties = Properties;
pub type ItemProperties = Properties;
pub type LocationProperties = Properties;

pub struct DataCollection {
	commands: CommandCollection,
	items: ItemCollection,
	locations: LocationCollection,
	hints: HelpStringCollection,
	explanations: HelpStringCollection,
	responses: InfoStringCollection,
	puzzles: InfoStringCollection,
	events: InfoStringCollection,
	inventories: HashMap<InventoryId, InventoryRef>,
	event_turns: HashMap<u32, StringId>,
	tp_map_sleep: TpMap,
	tp_map_witch: TpMap,
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
			inventories: HashMap::new(),
			event_turns: HashMap::new(),
			tp_map_sleep: HashMap::new(),
			tp_map_witch: HashMap::new(),
			max_score: 0u32,
		}
	}

	pub fn init(&mut self, mut buffer: &mut FileBuffer) {
		let mut treasure_count: u32 = 0;
		self.commands.init(&mut buffer, constants::EXPECTED_COMMANDS);
		self.locations.init(&mut buffer, constants::EXPECTED_LOCATIONS);
		self.items.init(&mut buffer, constants::EXPECTED_ITEMS, &mut self.locations, &mut treasure_count);
		self.hints.init(&mut buffer);
		self.explanations.init(&mut buffer);
		self.responses.init(&mut buffer, constants::EXPECTED_STRINGS_RESPONSES, true);
		self.puzzles.init(&mut buffer, constants::EXPECTED_STRINGS_PUZZLES, true);
		self.events.init(&mut buffer, 0, false);

		self.init_inventories();
		self.init_event_turns();
		self.init_tp_maps();
		let achievement_count: u32 = self.puzzles.count_strings();
		self.max_score = treasure_count * constants::SCORE_TREASURE + achievement_count * constants::SCORE_PUZZLE;
	}

	fn init_inventory(&mut self, inventory_id: InventoryId, inventory_capacity: u32) {
		let inventory = Rc::new(RefCell::new(Box::new(Inventory::new(inventory_id, inventory_capacity))));
		self.inventories.insert(inventory_id, inventory);
	}

	fn init_inventories(&mut self) {
		self.init_inventory(constants::INVENTORY_ID_MAIN, constants::INVENTORY_CAPACITY_NORMAL);
		self.init_inventory(constants::INVENTORY_ID_CHASM, constants::INVENTORY_CAPACITY_NORMAL);
		self.init_inventory(constants::INVENTORY_ID_DREAM, constants::INVENTORY_CAPACITY_DREAM);
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
		self.tp_map_sleep.insert(constants::LOCATION_ID_SLEEP_0, (constants::LOCATION_ID_SLEEP_1, constants::INVENTORY_ID_DREAM));
		self.tp_map_sleep.insert(constants::LOCATION_ID_SLEEP_2, (constants::LOCATION_ID_SLEEP_0, constants::INVENTORY_ID_MAIN));
		self.tp_map_witch.insert(constants::LOCATION_ID_WITCH_0, (constants::LOCATION_ID_WITCH_1, constants::INVENTORY_ID_CHASM));
		self.tp_map_witch.insert(constants::LOCATION_ID_WITCH_1, (constants::LOCATION_ID_WITCH_0, constants::INVENTORY_ID_MAIN));
	}

	pub fn get_command(&self, key: String) -> Option<&Rc<Box<Command>>> {
		self.commands.get(key)
	}

	pub fn get_inventory(&self, key: InventoryId) -> &InventoryRef {
		match self.inventories.get(&key) {
			None => panic!("Error: Data collection corrupt when searching for inventory [{}].", key),
			Some(inventory) => return inventory,
		}
	}

	pub fn get_item_by_name(&self, key: String) -> Option<&ItemRef> {
		self.items.get_by_name(key)
	}

	pub fn get_item_by_id_certain(&self, key: ItemId) -> &ItemRef {
		match self.items.get_by_id(key.clone()) {
			None => panic!("Error: Data collection corrupt when searching for item [{}].", key),
			Some(item) => return item,
		}
	}

	pub fn get_location(&self, key: LocationId) -> Option<&LocationRef> {
		self.locations.get(key)
	}

	pub fn get_location_certain(&self, key: LocationId) -> &LocationRef {
		match self.get_location(key) {
			None => panic!("Error: Data collection corrupt when searching for location [{}].", key),
			Some(loc) => return loc,
		}
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
			None => collection.get_certain(constants::STR_DEFAULT),
			Some(value) => value
		}
	}

	pub fn get_response(&self, key: StringId) -> &str {
		self.responses.get_certain(key)
	}

	// TODO: more than one parameter; make generic with get_response
	pub fn get_response_param(&self, key: StringId, param: &str) -> String {
		let response = String::from(self.responses.get_certain(key));
		response.replace("$0", param)
	}

	pub fn get_puzzle(&self, key: StringId) -> &str {
		self.puzzles.get_certain(key)
	}

	// Retrieve an event for a given turn index; clear any event found and return it
	pub fn get_and_clear_event(&mut self, turn: u32) -> Option<&str> {
		if let Some(event_turn) = self.event_turns.remove(&turn) {
			return Some(self.events.get_certain(event_turn));
		}
		None
	}

	pub fn get_commands_non_secret(&self) -> String {
		self.commands.mk_non_secret_string(self.get_response(constants::STR_ID_COMMANDS_INTRO))
	}

	pub fn get_direction_enum(&self, dir_str: &str) -> Direction {
		*self.commands.get_direction_enum(dir_str)
	}

	pub fn get_tp_map_sleep(&self) -> &TpMap {
		&self.tp_map_sleep
	}

	pub fn get_tp_map_witch(&self) -> &TpMap {
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

pub fn str_to_u32(st: &str, radix: u32) -> Result<u32, ParseIntError> {
	u32::from_str_radix(st, radix)
}

pub fn str_to_u32_certain(st: &str, radix: u32) -> u32 {
	match str_to_u32(st, radix) {
		Err(why) => panic!(why),
		Ok(result) => result,
	}
}
