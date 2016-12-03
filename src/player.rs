use rand;
use rand::Rng;

use constants;
use data_collection::{DataCollection, InventoryRef, ItemId, ItemRef, LocationId, LocationRef, StringId, TpMap};
use item::{Item, ItemCheckFn};
use location::Direction;
use terminal;

pub type ItemManipFinalFn = fn(player: &mut Player, data: &DataCollection, item: &ItemRef);
pub type ItemManipFn = ItemManipFinalFn;

pub struct Player {
	inventory: InventoryRef,
	location: LocationRef,
	previous: Option<LocationRef>,
	achievement_count: u32, // number of puzzles player has solved
	playing: bool, // whether player is currently playing
	hints: u32, // number of hints player has requested
	instructions: u32, // number of instructions player has entered
	deaths: u32, // number of times player has died
	death_divisor: u32, // chance of death under specific circumstances
	alive: bool,
	strong: bool,
	location_id_safe: LocationId, // where player's important items get dropped on death
	location_id_wake: LocationId, // where player wakes after being reincarnated
}

impl Player {

	pub fn new(initial: LocationRef, inventory: InventoryRef) -> Player {
		Player {
			inventory: inventory,
			location: initial.clone(),
			previous: None,
			achievement_count: 0u32,
			playing: true,
			hints: 0u32,
			instructions: 0u32,
			deaths: 0u32,
			death_divisor: constants::DEATH_DIVISOR_NORMAL,
			alive: true,
			strong: false,
			location_id_safe: constants::LOCATION_ID_SAFE_INITIAL,
			location_id_wake: constants::LOCATION_ID_WAKE_INITIAL,
		}
	}

	pub fn has_light(&self) -> bool {
		self.location.borrow().has_or_contains_with_switchable_property(constants::CTRL_LOC_HAS_LIGHT, constants::CTRL_ITEM_GIVES_LIGHT) ||
			self.inventory.borrow().contains_with_switchable_property(constants::CTRL_ITEM_GIVES_LIGHT)
	}

	fn has_light_and_needsno_light(&self) -> bool {
		self.location.borrow().has_property(constants::CTRL_LOC_NEEDSNO_LIGHT) &&
			(self.inventory.borrow().contains_with_switchable_property(constants::CTRL_ITEM_GIVES_LIGHT) || self.location.borrow().contains_with_switchable_property(constants::CTRL_ITEM_GIVES_LIGHT))
	}

	pub fn has_air(&self) -> bool {
		self.location.borrow().has_or_contains_with_property(constants::CTRL_LOC_HAS_AIR, constants::CTRL_ITEM_GIVES_AIR) ||
			self.inventory.borrow().contains_with_property(constants::CTRL_ITEM_GIVES_AIR)
	}

	pub fn has_gravity(&self) -> bool {
		self.location.borrow().has_property(constants::CTRL_LOC_HAS_GRAVITY) || self.inventory.borrow().contains_with_property(constants::CTRL_ITEM_GIVES_GRAVITY)
	}

	pub fn has_nosnomp(&self) -> bool {
		self.location.borrow().has_or_contains_with_property(constants::CTRL_LOC_HAS_NOSNOMP, constants::CTRL_ITEM_GIVES_NOSNOMP) ||
			self.inventory.borrow().contains_with_property(constants::CTRL_ITEM_GIVES_NOSNOMP)
	}

	pub fn has_land(&self) -> bool {
	        self.location.borrow().has_property(constants::CTRL_LOC_HAS_LAND) || self.inventory.borrow().contains_with_property(constants::CTRL_ITEM_GIVES_LAND)
	}

	fn has_invisibility(&self) -> bool {
		self.inventory.borrow().contains_with_property(constants::CTRL_ITEM_GIVES_INVISIBILITY)
	}

	pub fn is_playing(&self) -> bool {
		self.playing
	}

	pub fn set_playing(&mut self, b: bool) {
		self.playing = b
	}

	pub fn is_alive(&self) -> bool {
		self.alive
	}

	pub fn set_alive(&mut self, b: bool) {
		self.alive = b;
	}

	// Return a description of what the player sees when they look
	pub fn get_look(&self, data: &DataCollection) -> String {
		self.location.borrow_mut().set_visited(true);
		self.get_effective_appearance(data, self.mk_location_string(data))
	}

	pub fn get_score_str(&self, data: &DataCollection, response_code_start: StringId) -> String {
		let total_score = self.calculate_score(data);
		String::from(data.get_response(response_code_start)) +
		&data.get_response_param(constants::STR_ID_SCORE_POINTS, &total_score.to_string()) +
		&data.get_response_param(constants::STR_ID_SCORE_DIED, &data.get_max_score().to_string()) +
		&data.get_response_param(constants::STR_ID_SCORE_DEATHS, &self.deaths.to_string()) +
		&data.get_response_param(constants::STR_ID_SCORE_INSTRUCTIONS, &self.instructions.to_string()) +
		&data.get_response_param(constants::STR_ID_SCORE_HINTS, &self.hints.to_string())
	}

	fn calculate_score(&self, data: &DataCollection) -> u32 {
		let treasure_score = data.get_stowed_treasure_count() * constants::SCORE_TREASURE;
		let achievement_score = self.achievement_count * constants::SCORE_PUZZLE;
		let death_penalty = (self.deaths * constants::PENALTY_DEATH) as i32 * -1;
		let hint_penalty = (self.hints * constants::PENALTY_HINT) as i32 * -1;
		let total_score = treasure_score as i32 + achievement_score as i32 + death_penalty + hint_penalty;
		if total_score < 0 {0} else {total_score as u32}
	}

	pub fn increment_hints(&mut self) {
		self.hints = self.hints + 1;
	}

	pub fn get_instructions(&self) -> u32 {
		self.instructions
	}

	pub fn increment_instructions(&mut self) {
		self.instructions = self.instructions + 1;
	}

	pub fn decrement_instructions(&mut self) {
		self.instructions = self.instructions - 1;
	}

	pub fn increment_deaths(&mut self) {
		self.deaths = self.deaths + 1;
	}

	pub fn mk_inventory_string(&self, data: &DataCollection) -> String {
		self.inventory.borrow().mk_string(data.get_response(constants::STR_ID_INVENTORY_EMPTY), data.get_response(constants::STR_ID_INVENTORY_INTRO))
	}

	fn mk_location_string(&self, data: &DataCollection) -> String {
		self.location.borrow().mk_full_string(data.get_response(constants::STR_ID_YOU_ARE))
	}

	// Return whether a location is the last place the player was
	fn is_previous_loc(&self, next: &LocationRef) -> bool {
		let previous = self.previous.clone();
		match previous {
			None => return false,
			Some(prev) => prev.borrow().get_id() == next.borrow().get_id(),
		}
	}

	pub fn die(&mut self, data: &DataCollection) {
		self.set_alive(false);
		self.increment_deaths();
		let location_safe = data.get_location_certain(self.location_id_safe);
		self.drop_on_death(location_safe);
		self.location = data.get_location_certain(self.location_id_wake).clone();
		self.previous = None;
	}

	pub fn drop_on_death(&mut self, location_safe: &LocationRef) {
		self.inventory.borrow_mut().drop_on_death(&self.location, location_safe);
	}

	fn get_effective_description(&self, haze_description: String, darkness_description: String, default_description: String) -> String {
		if self.has_light_and_needsno_light() {
			return haze_description;
		}
		if !self.has_light() {
			return darkness_description;
		}
		return default_description
	}

	fn get_effective_appearance(&self, data: &DataCollection, default_description: String) -> String {
		self.get_effective_description(String::from(data.get_response(constants::STR_ID_NO_SEE_HAZE)), String::from(data.get_response(constants::STR_ID_NO_SEE_DARKNESS)), default_description)
	}

	pub fn get_location_stubname(&self) -> String {
		let unknown_description = String::from(constants::STR_LOCATION_UNKNOWN);
		self.get_effective_description(unknown_description.clone(), unknown_description, self.location.borrow().get_shortname())
	}

	fn observe_item(&mut self, data: &DataCollection, item: &ItemRef, act: ItemManipFinalFn) {
		if !self.has_light() {
			terminal::write_full(data.get_response(constants::STR_ID_NO_SEE_DARKNESS));
			return;
		}
		act(self, data, item);
	}

	pub fn has_item_inventory(&self, item_id: ItemId) -> bool {
		self.inventory.borrow().contains_item(item_id)
	}

	fn has_item_location(&self, item_id: ItemId) -> bool {
		self.location.borrow().contains_item(item_id)
	}

	pub fn has_item_present(&self, item_id: ItemId) -> bool {
	    self.has_item_inventory(item_id) || self.has_item_location(item_id)
	}

	fn complete_obstruction_achievement(&mut self, obstruction_id: ItemId, response: &str) {
		self.location.borrow_mut().remove_item_certain(obstruction_id);
		self.complete_achievement(response);
	}

	fn complete_achievement(&mut self, response: &str) {
		self.achievement_count = self.achievement_count + 1;
		terminal::write_full(response);
	}

	pub fn float(&mut self, data: &DataCollection) {
		let has_ceiling = self.location.borrow().has_property(constants::CTRL_LOC_HAS_CEILING);
		if has_ceiling { // There is a ceiling; player is safe
			terminal::write_full(data.get_response(constants::STR_ID_NO_GRAVITY));
		} else { // There is nothing above, so player floats away and dies
			terminal::write_full(data.get_response(constants::STR_ID_DEATH_NO_GRAVITY));
			self.die(data);
		}
	}

	fn operate_machine(&mut self, data: &DataCollection, cartridge: &ItemRef, request: &ItemRef) {
		if !request.borrow().has_property(constants::CTRL_ITEM_FACTORY) {
			terminal::write_full(data.get_response(constants::STR_ID_MACHINE_NO_KNOW_CREATE));
			return;
		}
		if !request.borrow().is_new() {
			terminal::write_full(data.get_response(constants::STR_ID_MACHINE_ALREADY_CREATE));
			return;
		}
		self.inventory.borrow_mut().remove_item_certain(constants::ITEM_ID_CARTRIDGE);
		match request.borrow().get_id() {
			constants::ITEM_ID_LENS => data.get_location_certain(constants::LOCATION_ID_OBSERVATORY).borrow_mut().insert_item(cartridge.clone()),
			constants::ITEM_ID_WIRE => data.get_location_certain(constants::LOCATION_ID_SENSOR).borrow_mut().insert_item(cartridge.clone()),
			_ => {},
		}

		self.location.borrow_mut().insert_item(request.clone());
		terminal::write_full(&data.get_response_param(constants::STR_ID_MACHINE_DISPENSE, &request.borrow().get_shortname()));
	}

	fn play_player(&self, data: &DataCollection, player: &ItemRef) {
		let mut response_code = constants::STR_ID_NO_MUSIC;
		if player.borrow().contains_item(constants::ITEM_ID_CD) {
			response_code = constants::STR_ID_PLAY_CD
		} else if player.borrow().contains_item(constants::ITEM_ID_CASSETTE) {
			response_code = constants::STR_ID_PLAY_CASSETTE;
		}
		terminal::write_full(data.get_response(response_code));
		player.borrow_mut().set_on(false);
	}

	// Determine whether there would be a problem executing a particular command on a particular item FIXME: clean this up
	fn has_problem_executing(data: &DataCollection, primary: &ItemRef, other: &ItemRef, check: ItemCheckFn) -> bool {
		match check(&**primary.borrow(), other) {
			None => return false,
			Some(reason) => {
				terminal::write_full(&data.get_response_param(reason, primary.borrow().get_shortname()));
				return true;
			},
		}
	}

	fn release_item(&mut self, data: &DataCollection, item: &ItemRef, thrown: bool) {
		let item_id = item.borrow().get_id();
		self.inventory.borrow_mut().remove_item_certain(item_id);

		let liquid = item.borrow().has_property(constants::CTRL_ITEM_LIQUID);
		let is_fragile = item.borrow().has_property(constants::CTRL_ITEM_FRAGILE);
		let has_floor = self.location.borrow().has_property(constants::CTRL_LOC_HAS_FLOOR);
		let has_land = self.location.borrow().has_property(constants::CTRL_LOC_HAS_LAND);
		let mut shattered = false;
		let mut response_code = constants::STR_ID_DROP_GOOD;
		if liquid { // When dropped, liquids drain away
			response_code = constants::STR_ID_EMPTY_LIQUID
		} else if !has_floor && self.has_gravity() { // When there is no floor, gravity pulls item down to location below current location
			if let Some(below) = self.location.borrow().get_direction(Direction::Down) {
				terminal::write_full(data.get_response(constants::STR_ID_DROP_NO_FLOOR));
				if is_fragile {
					shattered = true;
					response_code = constants::STR_ID_BREAK_FAR;
				} else {
					below.borrow_mut().insert_item(item.clone());
					response_code = constants::STR_ID_DROP_FAR;
				}
			}
		} else if is_fragile && thrown { // When thrown, fragile items shatter
			shattered = true;
			response_code = constants::STR_ID_BREAK_NEAR;
		} else if !has_land { // When dropped into open water, item is lost forever
			item.borrow_mut().retire();
			response_code = constants::STR_ID_DROP_WATER;
		} else {
			self.location.borrow_mut().insert_item(item.clone());
		}

		// Specific item drops
		if item_id == constants::ITEM_ID_LION {
			response_code = constants::STR_ID_LION_SITS;
			let wolf_present = self.has_item_location(constants::ITEM_ID_WOLF);
			if wolf_present {
				self.complete_obstruction_achievement(constants::ITEM_ID_WOLF, data.get_puzzle(constants::PUZZLE_ID_WOLF));
			}
		}

		if item_id == constants::ITEM_ID_PUPPY {
			let dogs_present = self.has_item_location(constants::ITEM_ID_DOGS);
			if dogs_present {
				self.location.borrow_mut().remove_item_certain(constants::ITEM_ID_PUPPY);
				self.complete_obstruction_achievement(constants::ITEM_ID_DOGS, data.get_puzzle(constants::PUZZLE_ID_DOGS));
				self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_BELL).clone());
				response_code = constants::STR_ID_BELL_FEET;
			}
		}

		terminal::write_full(data.get_response(response_code));

		if shattered && item_id == constants::ITEM_ID_MIRROR {
			terminal::write_full(data.get_response(constants::STR_ID_BAD_LUCK));
			self.death_divisor = constants::DEATH_DIVISOR_SMASHED;
		}
	}

	// Remove one item from either location or inventory
	fn remove_item_from_current(&mut self, id_to_remove: ItemId) {
		let in_inventory = self.has_item_inventory(id_to_remove);
		if in_inventory {
			self.inventory.borrow_mut().remove_item_certain(id_to_remove);
		} else {
			self.location.borrow_mut().remove_item_certain(id_to_remove);
		}
	}

	fn rob_pirate(&mut self, data: &DataCollection, pirate: &ItemRef, reward_code: ItemId, kill: bool,
			response_code_kill: StringId, response_code_success: StringId) {
		let reward = data.get_item_by_id_certain(reward_code);
		let reward_is_new = reward.borrow().is_new();
		let inventory_fits = self.inventory.borrow().can_fit(reward);
		if !reward_is_new {
			terminal::write_full(data.get_response(constants::STR_ID_PIRATE_EMPTY)); // Player has already robbed the pirate
		} else if kill {
			terminal::write_full(data.get_response(response_code_kill));
			self.die(data);
		} else if !inventory_fits {
			terminal::write_full(&data.get_response_param(constants::STR_ID_PIRATE_HEAVY, pirate.borrow().get_shortname()));
		} else {
			self.inventory.borrow_mut().insert_item(reward.clone());
			self.complete_achievement(data.get_puzzle(response_code_success));
		}
	}

	fn switch_item(&mut self, data: &DataCollection, item: &ItemRef, on_next: bool) {
		if !item.borrow().has_property(constants::CTRL_ITEM_SWITCHABLE) {
			terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW));
			return;
		}
		if (item.borrow().is_on() && on_next) || (!item.borrow().is_on() && !on_next) {
			terminal::write_full(data.get_response(constants::STR_ID_ALREADY_DONE));
			return;
		}

		item.borrow_mut().set_on(on_next);
		terminal::write_full(data.get_response(constants::STR_ID_DONE));
		let item_id = item.borrow().get_id();
		if item_id == constants::ITEM_ID_BUTTON { // When the button is off, ambient gravity in the anteroom is on, and vice-versa
			let anteroom = data.get_location_certain(constants::LOCATION_ID_ANTEROOM);
			anteroom.borrow_mut().set_property(constants::CTRL_LOC_HAS_GRAVITY, !on_next);
			terminal::write_full(data.get_response(constants::STR_ID_NOTHING_HAPPENS));
		} else if item_id == constants::ITEM_ID_LEVER {
			let docking_ctrl = data.get_location_certain(constants::LOCATION_ID_DOCKINGCONTROL);
			docking_ctrl.borrow_mut().set_property(constants::CTRL_LOC_HAS_LIGHT, on_next); // Opposite, as we have just changed it
			let response_code = if docking_ctrl.borrow().has_property(constants::CTRL_LOC_HAS_LIGHT) {constants::STR_ID_DOCKING_LIGHT_ON} else {constants::STR_ID_DOCKING_LIGHT_OFF};
			terminal::write_full(data.get_response(response_code));
		} else if item_id == constants::ITEM_ID_PLAYER && on_next {
			self.play_player(data, item);
		}
	}

	// Unlink an item from wherever currently contains it
	// FIXME: find a better solution to this
	fn unlink_item(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		let previous_id = item.borrow().get_location();
		match previous_id {
			constants::INDEX_START_INVENTORY ... constants::INDEX_STOP_INVENTORY => data.get_inventory(previous_id).borrow_mut().remove_item_certain(item_id),
			constants::INDEX_START_LOCATION ... constants::INDEX_STOP_LOCATION => data.get_location_certain(previous_id).borrow_mut().remove_item_certain(item_id),
			_ => data.get_item_by_id_certain(previous_id).borrow_mut().remove_item_certain(item_id),
		}
	}

	fn teleport(&mut self, data: &DataCollection, tp_map: &TpMap, response_code_no_teleport: StringId, response_code_teleport: StringId) {
		let loc_id = self.location.borrow().get_id();
		match tp_map.get(&loc_id) {
			None => terminal::write_full(data.get_response(response_code_no_teleport)),
			Some(nexts) => {
				let (location_id_next, inventory_id_next) = *nexts;
				self.inventory = data.get_inventory(inventory_id_next).clone();
				self.location = data.get_location_certain(location_id_next).clone();
				self.previous = None;
				terminal::write_full(data.get_response(response_code_teleport));
			},
		}
	}

	// Attempt to transfer an item from the player to a recipient
	fn transfer_item(&mut self, data: &DataCollection, gift: &ItemRef, recipient: &ItemRef) {

		let recipient_id = recipient.borrow().get_id();
		let gift_id = gift.borrow().get_id();
		let gift_edible = gift.borrow().has_property(constants::CTRL_ITEM_EDIBLE);
		let gift_liquid = gift.borrow().has_property(constants::CTRL_ITEM_LIQUID);
		let location_id = self.location.borrow().get_id();

		if recipient_id == constants::ITEM_ID_ALIEN {
			let chart_used = data.get_item_by_id_certain(constants::ITEM_ID_CHART).borrow().is_retired();
			let transmitter_used = data.get_item_by_id_certain(constants::ITEM_ID_TRANSMITTER).borrow().is_retired();
			let transmitter_on = data.get_item_by_id_certain(constants::ITEM_ID_TRANSMITTER).borrow().is_on();
			if gift_id == constants::ITEM_ID_CHART {
				self.inventory.borrow_mut().remove_item_certain(gift_id);
				gift.borrow_mut().set_location(constants::LOCATION_ID_GRAVEYARD);
				self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_CHART));
			} else if gift_id == constants::ITEM_ID_TRANSMITTER && chart_used && transmitter_on { // Alien cannot operate our machinery, so needs the transmitter to be on
				self.inventory.borrow_mut().remove_item_certain(gift_id);
				gift.borrow_mut().set_location(constants::LOCATION_ID_GRAVEYARD);
				self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_TRANSMITTER));
			} else if gift_id == constants::ITEM_ID_LENS && transmitter_used {
				let pendant = data.get_item_by_id_certain(constants::ITEM_ID_PENDANT);
				self.location.borrow_mut().insert_item(pendant.clone());
				self.inventory.borrow_mut().remove_item_certain(gift_id);
				gift.borrow_mut().set_location(constants::LOCATION_ID_GRAVEYARD);
				self.complete_obstruction_achievement(constants::ITEM_ID_ALIEN, data.get_puzzle(constants::PUZZLE_ID_LENS));
			} else {
				terminal::write_full(data.get_response(constants::STR_ID_ALIEN_NO_USE));
			}

		} else if recipient_id == constants::ITEM_ID_GUNSLINGER && gift_id == constants::ITEM_ID_MAGAZINE {
			let cartridge = data.get_item_by_id_certain(constants::ITEM_ID_CARTRIDGE);
			self.location.borrow_mut().insert_item(cartridge.clone());
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			self.complete_obstruction_achievement(constants::ITEM_ID_GUNSLINGER, data.get_puzzle(constants::PUZZLE_ID_GUNSLINGER));

		} else if recipient_id == constants::ITEM_ID_LION && gift_edible {
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			if gift_id == constants::ITEM_ID_KOHLRABI {
				terminal::write_full(data.get_response(constants::STR_ID_LION_CABBAGE));
				self.die(data);
			} else {
				terminal::write_full(data.get_response(constants::STR_ID_LION_WHET));
			}

		} else if recipient_id == constants::ITEM_ID_SKELETON && gift_id == constants::ITEM_ID_MILK {
			let brooch = data.get_item_by_id_certain(constants::ITEM_ID_BROOCH);
			self.location.borrow_mut().insert_item(brooch.clone());
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			self.complete_obstruction_achievement(constants::ITEM_ID_SKELETON, data.get_puzzle(constants::PUZZLE_ID_SKELETON));

		} else if recipient_id == constants::ITEM_ID_TROLL && gift_edible {
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			terminal::write_full(data.get_response(constants::STR_ID_TROLL_FED));
			self.die(data);

		} else if recipient_id == constants::ITEM_ID_BEAN && gift_id == constants::ITEM_ID_POTION {
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			self.remove_item_from_current(recipient_id);
			self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_PLANT).clone());
			terminal::write_full(data.get_response(constants::STR_ID_POUR_POTION_BEAN));

		} else if recipient_id == constants::ITEM_ID_BEAN && gift_id == constants::ITEM_ID_WATER && location_id == constants::LOCATION_ID_HOT {
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			self.remove_item_from_current(recipient_id);
			self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_BEANSTALK).clone());
			self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_BLOSSOM).clone());
			self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_BEANSTALK));

		} else if recipient_id == constants::ITEM_ID_PLANT && gift_id == constants::ITEM_ID_POTION {
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			self.remove_item_from_current(recipient_id);
			self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_BEAN).clone());
			terminal::write_full(data.get_response(constants::STR_ID_POUR_POTION_PLANT));

		} else if recipient_id == constants::ITEM_ID_MUSHROOM && gift_id == constants::ITEM_ID_WATER && location_id == constants::LOCATION_ID_SMALL {
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			self.remove_item_from_current(recipient_id);
			self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_TOADSTOOL).clone());
			self.location.borrow_mut().set_direction(Direction::North, Some(data.get_location_certain(constants::LOCATION_ID_TOADSTOOL).clone()));
			self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_MUSHROOM));

		} else if gift_liquid { // Default response for liquids
			self.inventory.borrow_mut().remove_item_certain(gift_id);
			terminal::write_full(&data.get_response_param(constants::STR_ID_POUR_LIQUID_DEFAULT, recipient.borrow().get_shortname()));

		} else { // Default response for non-liquids
			let response = String::from(data.get_response(constants::STR_ID_THE_START)) + recipient.borrow().get_shortname() + data.get_response(constants::STR_ID_NOT_INTERESTED) +
				gift.borrow().get_shortname() + data.get_response(constants::STR_ID_DOT);
			terminal::write_full(&response);
		}
	}

	pub fn acorn(&mut self, data: &DataCollection) {
		let at_treetop = self.location.borrow().is(constants::LOCATION_ID_TREETOP);
		if at_treetop {
			let acorn = data.get_item_by_id_certain(constants::ITEM_ID_ACORN);
			let acorn_is_new = acorn.borrow().is_new();
			if acorn_is_new {
				let garden = data.get_location_certain(constants::LOCATION_ID_GARDEN);
				garden.borrow_mut().insert_item(acorn.clone());
				self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_ACORN));
				return;
			}
		}
		terminal::write_full(data.get_response(constants::STR_ID_NOTHING_HAPPENS));
	}

	pub fn attack(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_BUCCANEER | constants::ITEM_ID_CORSAIR | constants::ITEM_ID_DOGS | constants::ITEM_ID_DRAGON |
				constants::ITEM_ID_GUNSLINGER | constants::ITEM_ID_LION | constants::ITEM_ID_WOLF => {
				terminal::write_full(data.get_response(constants::STR_ID_UNWISE))
			},
			constants::ITEM_ID_BOULDER => {
				if self.strong {
					self.complete_obstruction_achievement(constants::ITEM_ID_BOULDER, data.get_puzzle(constants::PUZZLE_ID_BOULDER));
					self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_DUST).clone());
					let cellar = data.get_location_certain(constants::LOCATION_ID_CELLAR);
					self.location.borrow_mut().set_direction(Direction::Down, Some(cellar.clone()));
					cellar.borrow_mut().set_direction(Direction::Up, Some(self.location.clone()));
					self.strong = false;
				} else {
					terminal::write_full(data.get_response(constants::STR_ID_BOULDER_HIT_WEAK));
				}
			}
			_ => {
				terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW));
			},
		}
	}

	pub fn burn(&mut self, data: &DataCollection, item: &ItemRef) {
		if !self.has_item_inventory(constants::ITEM_ID_MATCHES) {
			terminal::write_full(data.get_response(constants::STR_ID_NO_CARRY_BURN));
			return;
		}
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_BOOK => terminal::write_full(data.get_response(constants::STR_ID_PHILISTINE)),
			constants::ITEM_ID_BREAD => {
				self.remove_item_from_current(item_id);
				let toast = data.get_item_by_id_certain(constants::ITEM_ID_TOAST);
				self.location.borrow_mut().insert_item(toast.clone());
				terminal::write_full(data.get_response(constants::STR_ID_BURN_BREAD));
			},
			constants::ITEM_ID_LAMP => terminal::write_full(data.get_response(constants::STR_ID_NO_BURN_LAMP)),
			constants::ITEM_ID_MATCHES => terminal::write_full(data.get_response(constants::STR_ID_NO_BURN_MATCHES)),
			constants::ITEM_ID_TOAST => {
				self.remove_item_from_current(item_id);
				terminal::write_full(data.get_response(constants::STR_ID_BURN_TOAST));
				let at_airlocke = self.location.borrow().is(constants::LOCATION_ID_AIRLOCKE);
				if at_airlocke {
					let out_loc = data.get_location_certain(constants::LOCATION_ID_AIRLOCKEOUT);
					self.location.borrow_mut().set_direction(Direction::Southwest, Some(out_loc.clone()));
					self.location.borrow_mut().set_property(constants::CTRL_LOC_HAS_AIR, false);
					self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_AIRLOCK));
				} else {
					terminal::write_full(data.get_response(constants::STR_ID_ROBOT_MOUSE));
				}
			},
			_ => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW)),
		}
	}

	pub fn call(&mut self, data: &DataCollection, item: &ItemRef) {
		let callee_id = item.borrow().get_id();
		match callee_id {
			constants::ITEM_ID_BUCCANEER | constants::ITEM_ID_CORSAIR => {
				if item.borrow().is_new() {
					terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_APPLY));
				} else {
					terminal::write_full(data.get_response(constants::STR_ID_UNWISE));
				}
			},
			constants::ITEM_ID_SHIP => {
				let panel_present = self.has_item_location(constants::ITEM_ID_CONSOLE_FIXED);
				if !panel_present {
					terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_APPLY));
					return;
				}

				let console = data.get_item_by_id_certain(constants::ITEM_ID_CONSOLE_BROKEN);
				self.location.borrow_mut().insert_item(console.clone());

				// Player's safe and wake locations must now be west of the checkpoint, rather than east
				self.location_id_safe = constants::LOCATION_ID_SAFE_PIRATES;
				self.location_id_wake = constants::LOCATION_ID_WAKE_PIRATES;

				// Pirates arrive on the Asterbase
				let checkpoint = data.get_location_certain(constants::LOCATION_ID_CHECKPOINT);
				let corsair = data.get_item_by_id_certain(constants::ITEM_ID_CORSAIR);
				checkpoint.borrow_mut().insert_item(corsair.clone());
				let docking_ctrl = data.get_location_certain(constants::LOCATION_ID_DOCKINGCONTROL);
				let buccaneer = data.get_item_by_id_certain(constants::ITEM_ID_BUCCANEER);
				docking_ctrl.borrow_mut().insert_item(buccaneer.clone());
				docking_ctrl.borrow_mut().set_property(constants::CTRL_LOC_HAS_LIGHT, true);
				let lever = data.get_item_by_id_certain(constants::ITEM_ID_LEVER);
				lever.borrow_mut().set_on(true);

				// Link pirate ship (both item and location) to the docking bay
				let docking = data.get_location_certain(constants::LOCATION_ID_DOCKING);
				let ship_loc = data.get_location_certain(constants::LOCATION_ID_SHIP);
				docking.borrow_mut().insert_item(item.clone());
				docking.borrow_mut().set_direction(Direction::East, Some(ship_loc.clone()));
				docking.borrow_mut().set_direction(Direction::Southeast, Some(ship_loc.clone()));

				// Unlink the existing shuttle from the southeast of the docking bay
				let shuttle = data.get_location_certain(constants::LOCATION_ID_SHUTTLE);
				shuttle.borrow_mut().set_direction(Direction::South, None);

				self.complete_obstruction_achievement(constants::ITEM_ID_CONSOLE_FIXED, data.get_puzzle(constants::PUZZLE_ID_DISTRESS));
			},
			_ => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW)),
		}
	}

	pub fn cook(&mut self, data: &DataCollection, item: &ItemRef) {
		if !self.has_item_location(constants::ITEM_ID_CAULDRON) {
			terminal::write_full(data.get_response(constants::STR_ID_NO_HERE_COOK));
			return;
		}

		let cauldron = data.get_item_by_id_certain(constants::ITEM_ID_CAULDRON);
		if !cauldron.borrow().is_empty() {
		        terminal::write_full(data.get_response(constants::STR_ID_CAULDRON_FULL));
		        return;
		}

		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_MUSHROOM => terminal::write_full(data.get_response(constants::STR_ID_POISONOUS)),
			constants::ITEM_ID_KOHLRABI => {
			    self.inventory.borrow_mut().remove_item_certain(constants::ITEM_ID_KOHLRABI);
			    let stew = data.get_item_by_id_certain(constants::ITEM_ID_STEW);
			    cauldron.borrow_mut().set_within(Some(stew.clone()));
			    terminal::write_full(data.get_response(constants::STR_ID_COOK_CABBAGE));
			},
			constants::ITEM_ID_RADISHES => {
				self.inventory.borrow_mut().remove_item_certain(constants::ITEM_ID_RADISHES);
				let elixir = data.get_item_by_id_certain(constants::ITEM_ID_ELIXIR);
				cauldron.borrow_mut().set_within(Some(elixir.clone()));
				self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_ELIXIR));
			},
			_ => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW)),
		}
	}

	// Describe an item in the player's inventory or at the player's location
	pub fn describe(&mut self, data: &DataCollection, item: &ItemRef) {
		self.observe_item(data, item, Player::describe_final);
	}

	fn describe_final(&mut self, data: &DataCollection, item: &ItemRef) {
		terminal::write_full(&item.borrow().mk_full_string(data.get_response(constants::STR_ID_IT_IS)));
	}

	pub fn drink(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
			terminal::write_full(data.get_response(constants::STR_ID_DRINK_NON_LIQUID));
			return;
		}

		let item_id = item.borrow().get_id();
		self.inventory.borrow_mut().remove_item_certain(item_id);
		terminal::write_full(data.get_response(constants::STR_ID_DRINK_LIQUID));

		let mut response_code = constants::STR_ID_NOTHING_HAPPENS;
		match item_id {
			constants::ITEM_ID_AQUA => response_code = constants::STR_ID_DRINK_AQUA,
			constants::ITEM_ID_WATER => response_code = constants::STR_ID_DRINK_WATER,
			constants::ITEM_ID_STEW => response_code = constants::STR_ID_DRINK_STEW,
			constants::ITEM_ID_ELIXIR => {
				self.strong = true;
				response_code = constants::STR_ID_DRINK_ELIXIR;
			}
			constants::ITEM_ID_POTION => {
				response_code = constants::STR_ID_DRINK_POTION;
				self.die(data);
			},
			_ => {},
		}
		terminal::write_full(data.get_response(response_code));
	}

	pub fn drop(&mut self, data: &DataCollection, item: &ItemRef) {
		self.release_item(data, item, false);
	}

	pub fn eat(&mut self, data: &DataCollection, item: &ItemRef) {
		if item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
			terminal::write_full(&data.get_response_param(constants::STR_ID_EAT_LIQUID, item.borrow().get_shortname()));
			return;
		}

		let item_id = item.borrow().get_id();
		let mut response_code = constants::STR_ID_NO_KNOW_HOW;
		match item_id {
			constants::ITEM_ID_ALIEN => response_code = constants::STR_ID_EAT_ALIEN,
			constants::ITEM_ID_CAULDRON => response_code = constants::STR_ID_EAT_CAULDRON,
			constants::ITEM_ID_DOGS => response_code = constants::STR_ID_NO,
			constants::ITEM_ID_PUPPY => response_code = constants::STR_ID_EAT_PUPPY,
			constants::ITEM_ID_LION => response_code = constants::STR_ID_EAT_LION,
			constants::ITEM_ID_KOHLRABI => response_code = constants::STR_ID_EAT_CABBAGE,
			constants::ITEM_ID_MUSHROOM => response_code = constants::STR_ID_POISONOUS,
			constants::ITEM_ID_RADISHES => {
				self.remove_item_from_current(constants::ITEM_ID_RADISHES);
				response_code = constants::STR_ID_EAT_RADISHES;
			},
			_ => {},
		}
		terminal::write_full(data.get_response(response_code));
	}

	pub fn empty(&mut self, data: &DataCollection, item: &ItemRef) {
		if Player::has_problem_executing(data, item, item, Item::has_problem_emptying) {
			return;
		}

		let within_ref = item.borrow_mut().remove_within();
		match within_ref {
			None => terminal::write_full(data.get_response(constants::STR_ID_ALREADY_EMPTY)),
			Some(within) => {
				let is_liquid = within.borrow().has_property(constants::CTRL_ITEM_LIQUID);
				if is_liquid {
					terminal::write_full(data.get_response(constants::STR_ID_EMPTY_LIQUID));
				} else {
					let item_id = item.borrow().get_id();
					let in_inventory = self.has_item_inventory(item_id);
					if in_inventory {
						self.inventory.borrow_mut().insert_item(within.clone());
						terminal::write_full(&data.get_response_param(constants::STR_ID_EMPTY_CARRY, &within.borrow().get_shortname()));
					} else {
						self.location.borrow_mut().insert_item(within.clone());
						terminal::write_full(&data.get_response_param(constants::STR_ID_EMPTY_SET, &within.borrow().get_shortname()));
					}
				}
			},
		}
	}

	pub fn exchange(&mut self, data: &DataCollection, item: &ItemRef) {
		let building_present = self.has_item_location(constants::ITEM_ID_BUILDING);
		let machine_present = self.has_item_location(constants::ITEM_ID_MACHINE);
		if building_present {
			let is_treasure = item.borrow().has_property(constants::CTRL_ITEM_TREASURE);
			if is_treasure {
				terminal::write_full(&data.get_response_param(constants::STR_ID_EXCHANGE_GOOD, item.borrow().get_shortname()));
				terminal::write_full(data.get_response(constants::STR_ID_BUY_FARM));
				self.playing = false;
			} else {
				terminal::write_full(data.get_response(constants::STR_ID_NOT_VALUABLE));
			}
		} else if machine_present {
			if !item.borrow().is(constants::ITEM_ID_CARTRIDGE) {
				terminal::write_full(data.get_response(constants::STR_ID_MACHINE_REJECT));
				return;
			}
			let request_str = terminal::read_question(data.get_response(constants::STR_ID_MACHINE_ASK));
			match data.get_item_by_name(request_str[0].clone()) {
				None => terminal::write_full(data.get_response(constants::STR_ID_MACHINE_NO_KNOW_WHAT)),
				Some(request) => {
					self.operate_machine(data, item, request);
				},
			}
		} else {
			terminal::write_full(data.get_response(constants::STR_ID_NOWHERE_EXCHANGE));
		}
	}

	pub fn fairy(&mut self, data: &DataCollection) {
		let fairy_present = self.has_item_location(constants::ITEM_ID_FAIRY);
		let envelope = data.get_item_by_id_certain(constants::ITEM_ID_ENVELOPE);
		let tooth_within = envelope.borrow().contains_item(constants::ITEM_ID_TOOTH);
		if fairy_present && tooth_within {
			let coin = data.get_item_by_id_certain(constants::ITEM_ID_COIN);
			envelope.borrow_mut().remove_item_certain(constants::ITEM_ID_TOOTH);
			envelope.borrow_mut().set_within(Some(coin.clone()));
			self.complete_obstruction_achievement(constants::ITEM_ID_FAIRY, data.get_puzzle(constants::PUZZLE_ID_FAIRY));
		} else {
			terminal::write_full(data.get_response(constants::STR_ID_NOTHING_HAPPENS));
		}
	}

	pub fn feed(&mut self, data: &DataCollection, item: &ItemRef) {
		let is_recipient = item.borrow().has_property(constants::CTRL_ITEM_RECIPIENT);
		if is_recipient {
			self.feed_dative(data, item);
		} else {
			if !self.has_item_inventory(item.borrow().get_id()) {
				terminal::write_full(&data.get_response_param(constants::STR_ID_NO_HAVE_INVENTORY, &item.borrow().get_shortname()));
				return;
			}
			self.feed_accusative(data, item);
		}
	}

	// Feed, where the direct object is known and the indirect is not
	fn feed_accusative(&mut self, data: &DataCollection, direct: &ItemRef) {

		// Find out what player wants to feed it to
		let indirect_str = terminal::read_question(&data.get_response_param(constants::STR_ID_WHAT_FEED_ACC, direct.borrow().get_shortname()));

		// Feed food to recipient, if it exists and player is carrying it
		match data.get_item_by_name(indirect_str[0].clone()) {
			None => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_WHO_WHAT)),
			Some(indirect) => {
				let present = self.has_item_present(indirect.borrow().get_id());
				if present {
					self.feed_item_unknown(data, direct, indirect);
				} else {
					terminal::write_full(&data.get_response_param(constants::STR_ID_NO_SEE_HERE, &indirect.borrow().get_shortname()));
				}
			},
		}
	}

	// Feed, where the indirect object is known and the direct is not
	fn feed_dative(&mut self, data: &DataCollection, indirect: &ItemRef) {

		// Find out what player wants to feed to it
		let direct_str = terminal::read_question(&data.get_response_param(constants::STR_ID_WHAT_FEED_DAT, indirect.borrow().get_shortname()));

		// Feed food to recipient, if it exists and player is carrying it
		match data.get_item_by_name(direct_str[0].clone()) {
			None => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_WHO_WHAT)),
			Some(direct) => {
				let direct_id = direct.borrow().get_id();
				let in_inventory = self.has_item_inventory(direct_id);
				if in_inventory {
					self.feed_item_unknown(data, direct, indirect);
				} else {
					terminal::write_full(&data.get_response_param(constants::STR_ID_NO_HAVE_INVENTORY, &direct.borrow().get_shortname()));
				}
			},
		}
	}

	// Attempt to feed item, when we are not sure if the recipient can accept or not
	fn feed_item_unknown(&mut self, data: &DataCollection, direct: &ItemRef, indirect: &ItemRef) {
		if !indirect.borrow().has_property(constants::CTRL_ITEM_RECIPIENT) {
			terminal::write_full(&data.get_response_param(constants::STR_ID_NOT_FEEDABLE, indirect.borrow().get_shortname()));
			return;
		}
		self.transfer_item(data, direct, indirect);
	}

	pub fn fish(&mut self, data: &DataCollection) {
		if !self.has_item_inventory(constants::ITEM_ID_NET) {
			terminal::write_full(data.get_response(constants::STR_ID_NO_EQUIPMENT));
			return;
		}
		let glint_present = self.has_item_location(constants::ITEM_ID_GLINT);
		if !glint_present {
			terminal::write_full(data.get_response(constants::STR_ID_NO_FISH));
			return;
		}
		let nugget = data.get_item_by_id_certain(constants::ITEM_ID_NUGGET);
		if !self.inventory.borrow().can_fit(nugget) {
			terminal::write_full(data.get_response(constants::STR_ID_GLINT_HEAVY));
			return;
		}
		self.inventory.borrow_mut().insert_item(nugget.clone());
		self.complete_obstruction_achievement(constants::ITEM_ID_GLINT, data.get_puzzle(constants::PUZZLE_ID_GLINT));
	}

	#[cfg(debug_assertions)]
	pub fn flash(&mut self, data: &DataCollection, next: LocationRef) {
		self.location = next;
		self.previous = None;
		terminal::write_full(&self.location.borrow().mk_arrival_string(data.get_response(constants::STR_ID_YOU_ARE)));
	}

	pub fn fly(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		let loc_id = self.location.borrow().get_id();
		match item_id {
			constants::ITEM_ID_SHIP => {
				let ship_present = self.has_item_location(constants::ITEM_ID_SHIP);
				let key_present = self.has_item_inventory(constants::ITEM_ID_KEY);
				if ship_present {
					terminal::write_full(data.get_response(constants::STR_ID_NOT_IN_SHIP));
				} else if loc_id != constants::LOCATION_ID_SHIP {
					terminal::write_full(&data.get_response_param(constants::STR_ID_NO_SEE_HERE, item.borrow().get_shortname()));
				} else if !key_present {
					terminal::write_full(data.get_response(constants::STR_ID_NO_KEY));
				} else {
					self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_ESCAPE));
					self.playing = false;
				}
			}
			_ => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW)),
		}
	}

	pub fn give(&mut self, data: &DataCollection, item: &ItemRef) {
		// Find out what player wants to give item to
		let recipient_str = terminal::read_question(&data.get_response_param(constants::STR_ID_WHAT_GIVE, item.borrow().get_shortname()));

		// Give item to recipient, if it exists and player is carrying it
		match data.get_item_by_name(recipient_str[0].clone()) {
			None => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_WHO_WHAT)),
			Some(recipient) => {
				let present = self.has_item_present(recipient.borrow().get_id());
				if present {
					self.transfer_item(data, item, recipient);
				} else {
					terminal::write_full(&data.get_response_param(constants::STR_ID_NO_SEE_HERE, &recipient.borrow().get_shortname()));
				}
			},
		}
	}

	// Have player travel to an adjacent location
	pub fn go(&mut self, data: &DataCollection, dir: Direction) {

		let location_before = self.location.clone();

		let move_result = match dir {
			Direction::Back => self.try_move_back(dir),
			_ => self.try_move_other(dir),
		};
		let (next_location_option, death, response_code_option, obstruction_code_option) = move_result;

		// Print any returned responses
		if let Some(response_code) = response_code_option {
			match obstruction_code_option {
				None => terminal::write_full(data.get_response(response_code)),
				Some (obstruction_code) => {
					let obstruction_longname = String::from(data.get_item_by_id_certain(obstruction_code).borrow().get_longname());
					let obstruction_unknown = String::from(data.get_response(constants::STR_ID_OBSTRUCTION_UNKNOWN));
					terminal::write_full(&data.get_response_param(response_code, &self.get_effective_description(obstruction_unknown.clone(), obstruction_unknown, obstruction_longname)));
				}
			}
		}

		// Update location if returned
		if let Some(next_location) = next_location_option {
			self.location = next_location;
			if self.location.borrow().can_reach(&location_before) {
				self.previous = Some(location_before);
			} else {
				self.previous = None;
			}
			let arrival_description = self.location.borrow().mk_arrival_string(data.get_response(constants::STR_ID_YOU_ARE));
			terminal::write_full(&self.get_effective_appearance(data, arrival_description));
			self.location.borrow_mut().set_visited(true);
		}

		// Process death
		if death {
			self.die(data);
		}
	}

	// Attempt to move to previous location
	// Return a tuple representing the next location (if move is successful), whether the player died, and any response message to be printed
	fn try_move_back(&mut self, dir: Direction) -> (Option<LocationRef>, bool, Option<StringId>, Option<ItemId>) {
		match self.previous.clone() {
			None => return (None, false, Some(constants::STR_ID_NO_REMEMBER), None),
			Some(prev) => {
				if let Some(movement_problem_id) = self.has_environmental_movement_problem(dir, &prev) {
					return (None, false, Some(movement_problem_id), None);
				}
				return self.try_move_to(&prev);
			},
		};
	}

	fn try_move_obstruction(&self, obstruction: &ItemRef, next: &LocationRef) -> (Option<LocationRef>, bool, Option<StringId>, Option<ItemId>) {
		let mut next_loc_option: Option<LocationRef> = None;
		let mut death = false;
		let mut response_code = constants::STR_ID_BLOCKED;
		let mut obstruction_code_option = Some(obstruction.borrow().get_id());
		if obstruction.borrow().is(constants::ITEM_ID_BUCCANEER) {
			if !self.has_invisibility() {
				response_code = constants::STR_ID_BUCCANEER_WATCHING;
			} else {
				next_loc_option = Some(next.clone());
				obstruction_code_option = None;
				response_code = constants::STR_ID_BUCCANEER_SNEAK_PAST;
			}
		} else if obstruction.borrow().is(constants::ITEM_ID_CORSAIR) {
			if self.has_item_inventory(constants::ITEM_ID_BOOTS) {
				death = true;
				response_code = constants::STR_ID_CORSAIR_SNEAK_PAST;
			} else {
				response_code = constants::STR_ID_CORSAIR_LISTENING;
			}
		}
		(next_loc_option, death, Some(response_code), obstruction_code_option)
	}

	fn has_environmental_movement_problem(&self, dir: Direction, next: &LocationRef) -> Option<StringId> {
		if !next.borrow().has_or_contains_with_property(constants::CTRL_LOC_HAS_AIR, constants::CTRL_ITEM_GIVES_AIR) && !self.inventory.borrow().contains_with_property(constants::CTRL_ITEM_GIVES_AIR) { // Refuse to proceed if there is no air at the next location
			return Some(constants::STR_ID_NO_AIR);
		}
		if dir == Direction::Up && self.has_gravity() && self.location.borrow().has_property(constants::CTRL_LOC_NEEDSNO_GRAVITY) { // Gravity is preventing the player from going up
			return Some(constants::STR_ID_NO_REACH_CEILING);
		}
		if dir == Direction::Down && self.has_gravity() && !self.location.borrow().has_property(constants::CTRL_LOC_HAS_FLOOR) {
			return Some(constants::STR_ID_DOWN_KILL);
		}
		if !next.borrow().has_property(constants::CTRL_LOC_HAS_LAND) && !self.inventory.borrow().contains_with_property(constants::CTRL_ITEM_GIVES_LAND) {
			return Some(constants::STR_ID_OPEN_WATER);
		}
		None
	}

	// Attempt to move to some location, which may not be reachable from the current location
	// Return a tuple representing the next location (if move is successful), whether the player died, and any response message to be printed
	fn try_move_other(&mut self, dir: Direction) -> (Option<LocationRef>, bool, Option<StringId>, Option<ItemId>) {
		let next_option = self.location.borrow().get_direction(dir);
		match next_option {
			None => {
				if dir == Direction::Out {
					return (None, false, Some(constants::STR_ID_NO_IN_OUT), None);
				}
				return (None, false, Some(constants::STR_ID_CANNOT_GO), None);
			},
			Some(next) => {
				if !self.is_previous_loc(&next) {
					if let Some(obstruction) = (**self.location.borrow()).get_obstruction() {
						return self.try_move_obstruction(&obstruction, &next);
					}
				}
				if let Some(movement_problem_id) = self.has_environmental_movement_problem(dir, &next) {
					return (None, false, Some(movement_problem_id), None);
				}
				return self.try_move_to(&next);
			},
		}
	}

	// Attempt to go to a location known to be adjacent
	// Return a tuple representing the next location (if move is successful), whether the player died, and any response message to be printed
	fn try_move_to(&mut self, next: &LocationRef) -> (Option<LocationRef>, bool, Option<StringId>, Option<ItemId>) {
		let mut rng = rand::thread_rng();
		let death_rand: u32 = rng.gen();
		let death = death_rand % self.death_divisor == 0;
		if !self.has_light() && !next.borrow().has_or_contains_with_switchable_property(constants::CTRL_LOC_HAS_LIGHT, constants::CTRL_ITEM_GIVES_LIGHT) && death {
			return (None, true, Some(constants::STR_ID_BREAK_NECK), None);
		} else if !self.has_nosnomp() && !next.borrow().has_or_contains_with_property(constants::CTRL_LOC_HAS_NOSNOMP, constants::CTRL_ITEM_GIVES_NOSNOMP) && death {
			return (None, true, Some(constants::STR_ID_SNOMP_KILL), None);
		} else {
			return (Some(next.clone()), false, None, None);
		}
	}

	#[cfg(debug_assertions)]
	pub fn grab(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().is_portable() {
			terminal::write_full(data.get_response(constants::STR_ID_NO_WANT_TAKE));
			return;
		}
		if !item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
			self.unlink_item(data, item);
		}
		self.inventory.borrow_mut().insert_item(item.clone());
		terminal::write_full(&data.get_response_param(constants::STR_ID_GRABBED, item.borrow().get_shortname()));
	}

	pub fn ignore(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_TROLL => self.complete_obstruction_achievement(constants::ITEM_ID_TROLL, data.get_puzzle(constants::PUZZLE_ID_TROLL)),
			_ => terminal::write_full(data.get_response(constants::STR_ID_IGNORED)),
		}
	}

	pub fn insert(&mut self, data: &DataCollection, item: &ItemRef) {
		if Player::has_problem_executing(data, item, item, Item::has_problem_inserting) {
			return;
		}

		// Find out what player wants to insert it into
		let container_str = terminal::read_question(&data.get_response_param(constants::STR_ID_WHAT_INSERT, item.borrow().get_shortname()));

		// Insert item into container, if container exists and is present
		match data.get_item_by_name(container_str[0].clone()) {
			None => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_WHO_WHAT)),
			Some(container) => {
				let present = self.has_item_present(container.borrow().get_id());
				if present {
					self.insert_final(data, item, container)
				} else {
					terminal::write_full(&data.get_response_param(constants::STR_ID_NO_SEE_HERE, container.borrow().get_shortname()));
				}
			},
		}
	}

	fn insert_final(&mut self, data: &DataCollection, item: &ItemRef, container: &ItemRef) {
		if Player::has_problem_executing(data, container, item, Item::has_problem_accepting) {
			return;
		}

		let item_id = item.borrow().get_id();
		let in_location = self.has_item_location(item_id);
		let in_inventory = self.has_item_inventory(item_id);
		if in_location {
			if !self.inventory.borrow().can_fit(&item) {
				terminal::write_full(data.get_response(constants::STR_ID_ITEM_HEAVY));
				return;
			}
			self.location.borrow_mut().remove_item_certain(item_id);
		} else if in_inventory {
			self.inventory.borrow_mut().remove_item_certain(item_id);
		}
		container.borrow_mut().set_within(Some(item.clone()));
		terminal::write_full(data.get_response(constants::STR_ID_INSERTED));
	}

	pub fn knit(&mut self, data: &DataCollection) {
		if !self.has_item_inventory(constants::ITEM_ID_NEEDLES) || !self.has_item_inventory(constants::ITEM_ID_YARN) {
			terminal::write_full(data.get_response(constants::STR_ID_NO_EQUIPMENT));
			return;
		}
		self.inventory.borrow_mut().remove_item_certain(constants::ITEM_ID_YARN);
		self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_JUMPER).clone());
		self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_JUMPER));
	}

	pub fn light(&mut self, data: &DataCollection, item: &ItemRef) {
		self.switch_item(data, item, true);
	}

	#[cfg(debug_assertions)]
	pub fn get_node(&self, data: &DataCollection) -> String {
		data.get_response_param(constants::STR_ID_NODE, &self.location.borrow().get_id().to_string())
	}

	pub fn play(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_WHISTLE => {
				let tune_words = terminal::read_question(data.get_response(constants::STR_ID_WHAT_PLAY));
				let tune = &tune_words[0];
				terminal::write_full(&data.get_response_param(constants::STR_ID_PLAY_WHISTLE, tune));

				if tune == data.get_response(constants::STR_ID_CABBAGE) {
					let lion_present = self.has_item_location(constants::ITEM_ID_LION);
					if lion_present {
						let lion = data.get_item_by_id_certain(constants::ITEM_ID_LION);
						let lion_obstruction = lion.borrow().has_property(constants::CTRL_ITEM_OBSTRUCTION);
						if lion_obstruction {
							lion.borrow_mut().set_property(constants::CTRL_ITEM_OBSTRUCTION, false);
							self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_LION));
						}
					}
				}
			},
			constants::ITEM_ID_PLAYER => {
				self.play_player(data, item);
			},
			_ => {
				terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW));
			},
		}
	}

	pub fn pour(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
			terminal::write_full(data.get_response(constants::STR_ID_POUR_NONLIQUID));
			return;
		}

		// Find out what player wants to pour liquid onto
		let recipient_str = terminal::read_question(&data.get_response_param(constants::STR_ID_WHAT_POUR, item.borrow().get_shortname()));

		// Pour liquid onto recipient
		match data.get_item_by_name(recipient_str[0].clone()) {
			None => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_WHO_WHAT)),
			Some(recipient) => {
				let present = self.has_item_present(recipient.borrow().get_id());
				if present {
					self.transfer_item(data, item, recipient);
				} else {
					terminal::write_full(&data.get_response_param(constants::STR_ID_NO_SEE_HERE, &recipient.borrow().get_shortname()));
				}
			},
		}
	}

	pub fn push(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_BUTTON | constants::ITEM_ID_LEVER => {
				let is_on = item.borrow().is_on();
				self.switch_item(data, item, !is_on);
			},
			_ => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW)),
		}
	}

	pub fn quench(&mut self, data: &DataCollection, item: &ItemRef) {
		self.switch_item(data, item, false);
	}

	pub fn read(&mut self, data: &DataCollection, item: &ItemRef) {
		self.observe_item(data, item, Player::read_final);
	}

	fn read_final(&mut self, data: &DataCollection, item: &ItemRef) {
		terminal::write_full(&item.borrow().mk_writing_string(data.get_response(constants::STR_ID_NO_WRITING), data.get_response(constants::STR_ID_READS)));
	}

	pub fn repair(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_CONSOLE_FIXED => terminal::write_full(data.get_response(constants::STR_ID_ALREADY_REPAIRED)),
			constants::ITEM_ID_CONSOLE_BROKEN => {
				let wire_present = self.has_item_inventory(constants::ITEM_ID_WIRE);
				if !wire_present {
					terminal::write_full(data.get_response(constants::STR_ID_NO_COMPONENT));
				} else {
					let panel = data.get_item_by_id_certain(constants::ITEM_ID_CONSOLE_FIXED);
					self.location.borrow_mut().insert_item(panel.clone());
					self.inventory.borrow_mut().remove_item_certain(constants::ITEM_ID_WIRE);
					self.complete_obstruction_achievement(constants::ITEM_ID_CONSOLE_BROKEN, data.get_puzzle(constants::PUZZLE_ID_CONSOLE));
				}
			},
			_ => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW)),
		}
	}

	pub fn rob(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_BODIES => terminal::write_full(data.get_response(constants::STR_ID_NO)),
			constants::ITEM_ID_BUCCANEER => {
				let kill_condition = !self.has_invisibility();
				self.rob_pirate(data, item, constants::ITEM_ID_MEDALLION, kill_condition, constants::STR_ID_BUCCANEER_SNEAK_ROB, constants::PUZZLE_ID_BUCCANEER);
			},
			constants::ITEM_ID_CORSAIR => {
				let kill_condition = self.has_item_inventory(constants::ITEM_ID_BOOTS);
				self.rob_pirate(data, item, constants::ITEM_ID_KEY, kill_condition, constants::STR_ID_CORSAIR_SNEAK_ROB, constants::PUZZLE_ID_CORSAIR);
			},
			_ => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW)),
		}
	}

	pub fn robot(&mut self, data: &DataCollection) {
		let robot_present = self.has_item_location(constants::ITEM_ID_ROBOT);
		if robot_present {
			self.complete_obstruction_achievement(constants::ITEM_ID_ROBOT, data.get_puzzle(constants::PUZZLE_ID_ROBOT));
		} else {
			terminal::write_full(data.get_response(constants::STR_ID_NOTHING_HAPPENS));
		}
	}

	pub fn roll(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_MARBLE => {
				let loc_id = self.location.borrow().get_id();
				let corsair_present = self.has_item_location(constants::ITEM_ID_CORSAIR);
				self.inventory.borrow_mut().remove_item_certain(item_id);
				terminal::write_full(data.get_response(constants::STR_ID_ROLL_MARBLE));
				if loc_id == constants::LOCATION_ID_CHECKPOINT && corsair_present {
					let under = data.get_location_certain(constants::LOCATION_ID_UNDER);
					self.complete_obstruction_achievement(constants::ITEM_ID_CORSAIR, data.get_puzzle(constants::PUZZLE_ID_MARBLE));
					under.borrow_mut().insert_item(item.clone());
					under.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_CORSAIR).clone());
				} else {
					self.location.borrow_mut().insert_item(item.clone());
					terminal::write_full(data.get_response(constants::STR_ID_NOTHING_HAPPENS));
				}
			}
			_ => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW)),
		}
	}

	pub fn rub(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_LAMP => terminal::write_full(data.get_response(constants::STR_ID_RUB_LAMP)),
			constants::ITEM_ID_DRAGON => {
				let tooth = data.get_item_by_id_certain(constants::ITEM_ID_TOOTH);
				self.location.borrow_mut().insert_item(tooth.clone());
				self.complete_obstruction_achievement(constants::ITEM_ID_DRAGON, data.get_puzzle(constants::PUZZLE_ID_DRAGON));
			},
			constants::ITEM_ID_PENDANT => {
				let thor = data.get_location_certain(constants::LOCATION_ID_THOR);
				let rod = data.get_item_by_id_certain(constants::ITEM_ID_ROD);
				self.unlink_item(data, rod);
				thor.borrow_mut().insert_item(rod.clone());
				terminal::write_full(data.get_response(constants::STR_ID_RUB_PENDANT));
			},
			_ => terminal::write_full(data.get_response(constants::STR_ID_NOTHING_INTERESTING)),
		}
	}

	pub fn say(&mut self, data: &DataCollection, statement: &str) {
		terminal::write_full(&data.get_response_param(constants::STR_ID_SAY, statement));
		if self.has_item_location(constants::ITEM_ID_CORSAIR) { // Pirate hears player
			terminal::write_full(data.get_response(constants::STR_ID_CORSAIR_SPEAK));
			self.die(data);
			return;
		}
		if statement == data.get_response(constants::STR_ID_HELLO) {
			let alien_present = self.has_item_location(constants::ITEM_ID_ALIEN);
			if alien_present {
				let chart_used = data.get_item_by_id_certain(constants::ITEM_ID_CHART).borrow().is_retired();
				let transmitter_used = data.get_item_by_id_certain(constants::ITEM_ID_TRANSMITTER).borrow().is_retired();
				if transmitter_used {
					terminal::write_full(data.get_response(constants::STR_ID_HELLO_LENS));
				} else if chart_used {
					terminal::write_full(data.get_response(constants::STR_ID_HELLO_BEACON));
				} else {
					terminal::write_full(data.get_response(constants::STR_ID_HELLO_CHART));
				}
			}
		}
	}

	pub fn sleep(&mut self, data: &DataCollection) {
		self.teleport(data, data.get_tp_map_sleep(), constants::STR_ID_NO_SLEEP, constants::STR_ID_SLEEP);
	}

	pub fn stare(&mut self, data: &DataCollection) {
		if !self.has_light() {
			terminal::write_full(data.get_response(constants::STR_ID_NO_SEE_DARKNESS));
			return;
		}
		if self.location.borrow().is(constants::LOCATION_ID_REFLECTION) || self.has_item_inventory(constants::ITEM_ID_MIRROR) {
			if self.has_invisibility() {
				terminal::write_full(data.get_response(constants::STR_ID_SEE_INVISIBLE));
			} else if self.strong {
				terminal::write_full(data.get_response(constants::STR_ID_SEE_STRONG));
			} else {
				terminal::write_full(data.get_response(constants::STR_ID_SEE_NORMAL));
			}
		} else {
			terminal::write_full(data.get_response(constants::STR_ID_SEE_NOTHING));
		}
	}

	pub fn take(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		if self.has_item_inventory(item_id) && !item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
			terminal::write_full(data.get_response(constants::STR_ID_ALREADY_HAVE));
			return;
		}

		if !item.borrow().is_portable() { // Cannot take fixtures, furniture, very heavy things, etc.
			terminal::write_full(data.get_response(constants::STR_ID_CANNOT_TAKE));
			return;
		}

		if !self.inventory.borrow().can_fit(&item) { // Can only carry so much at a time
			terminal::write_full(data.get_response(constants::STR_ID_ITEM_HEAVY));
			return;
		}

		if item.borrow().has_property(constants::CTRL_ITEM_LIQUID) { // Liquids require a container
			self.insert(data, item);
			return;
		}

		self.location.borrow_mut().remove_item_certain(item_id);
		self.inventory.borrow_mut().insert_item(item.clone());

		if !self.has_light() {
			terminal::write_full(&data.get_response_param(constants::STR_ID_TAKE_NO_LIGHT, item.borrow().get_shortname()));
		}
		if item.borrow().has_property(constants::CTRL_ITEM_WEARABLE) {
			terminal::write_full(data.get_response(constants::STR_ID_WORN));
		} else {
			terminal::write_full(data.get_response(constants::STR_ID_TAKEN));
		}
	}

	pub fn tether(&mut self, data: &DataCollection, item: &ItemRef) {
		if !self.has_item_inventory(constants::ITEM_ID_CABLE) {
			terminal::write_full(&data.get_response_param(constants::STR_ID_NO_TETHER, item.borrow().get_shortname()));
			return;
		}

		// Find out what player wants to tether it to
		let anchor_str = terminal::read_question(&data.get_response_param(constants::STR_ID_WHAT_TETHER, item.borrow().get_shortname()));

		match data.get_item_by_name(anchor_str[0].clone()) {
			None => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_WHO_WHAT)),
			Some(anchor) => {
				let anchor_id = anchor.borrow().get_id();
				if !self.has_item_inventory(anchor_id) && !self.has_item_location(anchor_id) {
					terminal::write_full(&data.get_response_param(constants::STR_ID_NO_SEE_HERE, anchor.borrow().get_shortname()));
					return;
				}
				let item_id = item.borrow().get_id();
				if item_id == constants::ITEM_ID_SHUTTLE && anchor_id == constants::ITEM_ID_SHIP {
					self.inventory.borrow_mut().remove_item_certain(constants::ITEM_ID_CABLE);
					self.complete_achievement(data.get_puzzle(constants::PUZZLE_ID_TETHER));
				} else {
					terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW));
				}
			},
		}
	}

	pub fn tezazzle(&mut self, data: &DataCollection) {
		self.teleport(data, data.get_tp_map_witch(), constants::STR_ID_NOTHING_HAPPENS, constants::STR_ID_WITCHED);
	}

	pub fn throw(&mut self, data: &DataCollection, item: &ItemRef) {
		terminal::write_full(data.get_response(constants::STR_ID_THROW));
		self.release_item(data, item, true);
	}

	pub fn wizard(&mut self, data: &DataCollection) {
		let wizard_present = self.has_item_location(constants::ITEM_ID_WIZARD);
		let mirror_present = self.has_item_inventory(constants::ITEM_ID_MIRROR);
		if wizard_present {
			if self.has_invisibility() {
				terminal::write_full(data.get_response(constants::STR_ID_WIZARDED));
			} else if mirror_present {
				self.complete_obstruction_achievement(constants::ITEM_ID_WIZARD, data.get_puzzle(constants::PUZZLE_ID_WIZARD));
			} else {
				terminal::write_full(data.get_response(constants::STR_ID_WIZARD_INVISIBLE));
				self.die(data);
			}
		} else {
			terminal::write_full(data.get_response(constants::STR_ID_SH_MAGIC));
		}
	}
}
