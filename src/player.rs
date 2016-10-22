use rand;
use rand::Rng;
use std::collections::HashMap;

use constants;
use data_collection::DataCollection;
use data_collection::ItemRef;
use data_collection::LocationRef;
use inventory::Inventory;
use location::Direction;
use terminal;

pub type ItemManipFinalFn = fn(player: &mut Player, data: &DataCollection, item: &ItemRef);
pub type ItemManipFn = ItemManipFinalFn;

pub struct Player {
	inventory: Inventory,
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
	location_id_safe: u32, // where player's important items get dropped on death
	location_id_wake: u32, // where player wakes after being reincarnated
}

impl Player {

	pub fn new(initial: LocationRef) -> Player {
		Player {
			inventory: Inventory::new(16),
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
		self.inventory.has_light() || self.location.borrow().has_light()
	}

	fn has_light_and_needsno_light(&self) -> bool {
		(self.inventory.has_light() || self.location.borrow().has_light_item()) && self.location.borrow().needsno_light()
	}

	pub fn has_air(&self) -> bool {
		self.inventory.has_air() || self.location.borrow().has_air()
	}

	pub fn has_gravity(&self) -> bool {
		self.inventory.has_gravity() || self.location.borrow().has_gravity()
	}

	pub fn has_nosnomp(&self) -> bool {
		self.inventory.has_nosnomp() || self.location.borrow().has_nosnomp()
	}

	fn has_invisibility(&self) -> bool {
		self.inventory.has_invisibility()
	}

	pub fn insert_item(&mut self, item_ptr: ItemRef) {
		self.inventory.insert_item(item_ptr);
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

	pub fn die(&mut self, data: &DataCollection) {
		self.set_alive(false);
		self.increment_deaths();
		let location_safe = data.get_location_certain(self.location_id_safe);
		self.drop_on_death(location_safe);
		self.location = data.get_location_certain(self.location_id_wake).clone();
		self.previous = None;
	}

	pub fn drop_on_death(&mut self, location_safe: &LocationRef) {
		self.inventory.drop_all(&self.location, location_safe, true, true);
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
		self.get_effective_description(String::from(data.get_response(16)), String::from(data.get_response(15)), default_description)
	}

	pub fn get_location_stubname(&self) -> String {
		self.get_effective_description(String::from("???"), String::from("???"), self.location.borrow().get_shortname())
	}

	fn observe_item(&mut self, data: &DataCollection, item: &ItemRef, act: ItemManipFinalFn) {
		if !self.has_light() {
			terminal::write_full(data.get_response(15));
			return;
		}
		act(self, data, item);
	}

	pub fn has_item_inventory(&self, item_id: u32) -> bool {
		self.inventory.contains_item(item_id)
	}

	pub fn has_item_present(&self, item_id: u32) -> bool {
	        self.inventory.contains_item(item_id) || self.location.borrow().contains_item(item_id)
	}

	fn complete_obstruction_achievement(&mut self, obstruction_id: u32, response: &str) {
		self.location.borrow_mut().remove_item_certain(obstruction_id);
		self.complete_achievement(response);
	}

	fn complete_achievement(&mut self, response: &str) {
		self.achievement_count = self.achievement_count + 1;
		terminal::write_full(response);
	}

	pub fn float(&mut self, data: &DataCollection) {
		let has_ceiling = self.location.borrow().has_ceiling();
		if has_ceiling { // There is a ceiling; player is safe
			terminal::write_full(data.get_response(153));
		} else { // There is nothing above, so player floats away and dies
			terminal::write_full(data.get_response(85));
			self.die(data);
		}
	}

	fn play_player(&self, data: &DataCollection, player: &ItemRef) {
		if player.borrow().contains_item(constants::ITEM_ID_CD) {
			terminal::write_full(data.get_response(96));
		} else if player.borrow().contains_item(constants::ITEM_ID_CASSETTE) {
			terminal::write_full(data.get_response(97));
		} else{
			terminal::write_full(data.get_response(95));
		}
		player.borrow_mut().set_on(false);
	}

	fn release_item(&mut self, data: &DataCollection, item: &ItemRef, thrown: bool) {
		self.inventory.remove_item_certain(item.borrow().get_id());

		let liquid = item.borrow().is_liquid();
		let is_fragile = item.borrow().is_fragile();
		let has_floor = self.location.borrow().has_floor();
		if liquid { // When dropped, liquids drain away
			terminal::write_full(data.get_response(42));
		} else if is_fragile && thrown { // When thrown, fragile items shatter
			terminal::write_full(data.get_response(136));
			if item.borrow().is(constants::ITEM_ID_MIRROR) {
				terminal::write_full(data.get_response(137));
				self.death_divisor = constants::DEATH_DIVISOR_SMASHED;
			}
		} else if !has_floor && self.has_gravity() { // Gravity pulls item down to location beneath current
			let self_loc = self.location.borrow();
			let below_option = self_loc.get_direction(&Direction::Down);
			match below_option {
				None => {}, // Probably an error state (error in datafile) TODO: do something with this case
				Some(below) => {
					terminal::write_full(data.get_response(36));
					if is_fragile {
						terminal::write_full(data.get_response(135));
					} else {
						below.borrow_mut().insert_item(item.clone(), true);
					}
				},
			}
		} else {
			self.location.borrow_mut().insert_item(item.clone(), true);
			terminal::write_full(data.get_response(37));
		}

		// Specific item drops
		if item.borrow().is(constants::ITEM_ID_LION) {
			let wolf_present = self.location.borrow().contains_item(constants::ITEM_ID_WOLF);
			if wolf_present {
				self.complete_obstruction_achievement(constants::ITEM_ID_WOLF, data.get_puzzle(13));
			}
		}
	}

	// Remove one item from either location or inventory, and insert another at location in its place
	fn remove_and_insert_item(&mut self, data: &DataCollection, id_to_remove: u32, id_to_insert: u32, response_code: u32) {
		if self.inventory.contains_item(id_to_remove) {
			self.inventory.remove_item_certain(id_to_remove);
		} else {
			self.location.borrow_mut().remove_item_certain(id_to_remove);
		}
		self.location.borrow_mut().insert_item(data.get_item_by_id_certain(id_to_insert).clone(), true);
		terminal::write_full(data.get_response(response_code));
	}

	fn rob_pirate(&mut self, data: &DataCollection, pirate: &ItemRef, reward_code: u32, kill: bool,
			response_code_kill: u32, response_code_success: u32) {
		let reward = data.get_item_by_id_certain(reward_code);
		let reward_is_new = reward.borrow().is_new();
		if !reward_is_new {
			terminal::write_full(data.get_response(115)); // Player has already robbed the pirate
		} else if kill {
			terminal::write_full(data.get_response(response_code_kill));
			self.die(data);
		} else if !self.inventory.can_fit(reward) {
			terminal::write_full(&data.get_response_param(173, pirate.borrow().get_shortname()));
		} else {
			self.inventory.insert_item(reward.clone());
			self.complete_achievement(data.get_puzzle(response_code_success));
		}
	}

	fn switch_item(&mut self, data: &DataCollection, item: &ItemRef, on_next: bool) {
		if !item.borrow().is_switchable() {
			terminal::write_full(data.get_response(94));
			return;
		}
		if (item.borrow().is_on() && on_next) || (!item.borrow().is_on() && !on_next) {
			terminal::write_full(data.get_response(2));
			return;
		}

		item.borrow_mut().set_on(on_next);
		terminal::write_full(data.get_response(62));
		let item_id = item.borrow().get_id();
		if item_id == constants::ITEM_ID_BUTTON { // When the button is off, ambient gravity in the anteroom is on, and vice-versa
			let anteroom = data.get_location_certain(constants::LOCATION_ID_ANTEROOM);
			anteroom.borrow_mut().set_gravity(!on_next);
			terminal::write_full(data.get_response(86));
		} else if item_id == constants::ITEM_ID_PLAYER && on_next {
			self.play_player(data, item);
		}
	}

	// Unlink an item from wherever currently contains it
	// FIXME: find a better solution to this
	fn unlink_item(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		let previous_id = item.borrow().get_location_true();
		if previous_id == constants::LOCATION_ID_INVENTORY { // Item is inventory
			self.inventory.remove_item_certain(item_id);
		} else if previous_id < constants::ITEM_INDEX_START { // Item is at some location
			data.get_location_certain(previous_id).borrow_mut().remove_item_certain(item_id);
		} else { // Item is in another item
			data.get_item_by_id_certain(previous_id).borrow_mut().remove_item_certain(item_id);
		}
	}

	fn teleport(&mut self, data: &DataCollection, tp_map: &HashMap<u32, u32>, permanent: bool,
		response_tag_no_teleport: u32, response_tag_teleport: u32) {
		let loc_id = self.location.borrow().get_id();
		match tp_map.get(&loc_id) {
			None => terminal::write_full(data.get_response(response_tag_no_teleport)),
			Some(next_id) => {
				self.inventory.drop_all(&self.location, data.get_location_certain(self.location_id_safe), false, permanent);
				self.location = data.get_location_certain(*next_id).clone();
				self.previous = None;
				self.location.borrow_mut().release_temporary(&mut self.inventory);
				terminal::write_full(data.get_response(response_tag_teleport));
			},
		}
	}

	// Attempt to transfer an item from the player to a recipient
	fn transfer_item(&mut self, data: &DataCollection, gift: &ItemRef, recipient: &ItemRef) {

		let recipient_id = recipient.borrow().get_id();
		let gift_id = gift.borrow().get_id();
		let gift_edible = gift.borrow().is_edible();
		let gift_liquid = gift.borrow().is_liquid();
		let location_id = self.location.borrow().get_id();

		if recipient_id == constants::ITEM_ID_ALIEN {
			let chart_used = data.get_item_by_id_certain(constants::ITEM_ID_CHART).borrow().is_retired();
			let transmitter_used = data.get_item_by_id_certain(constants::ITEM_ID_TRANSMITTER).borrow().is_retired();
			let transmitter_on = data.get_item_by_id_certain(constants::ITEM_ID_TRANSMITTER).borrow().is_on();
			if gift_id == constants::ITEM_ID_CHART {
				self.inventory.remove_item_certain(gift_id);
				gift.borrow_mut().set_locations(constants::LOCATION_ID_GRAVEYARD);
				self.complete_achievement(data.get_puzzle(2));
			} else if gift_id == constants::ITEM_ID_TRANSMITTER && chart_used && transmitter_on { // Alien cannot operate our machinery, so needs the transmitter to be on
				self.inventory.remove_item_certain(gift_id);
				gift.borrow_mut().set_locations(constants::LOCATION_ID_GRAVEYARD);
				self.complete_achievement(data.get_puzzle(1));
			} else if gift_id == constants::ITEM_ID_LENS && transmitter_used {
				let pendant = data.get_item_by_id_certain(constants::ITEM_ID_PENDANT);
				self.location.borrow_mut().insert_item(pendant.clone(), true);
				self.inventory.remove_item_certain(gift_id);
				gift.borrow_mut().set_locations(constants::LOCATION_ID_GRAVEYARD);
				self.complete_obstruction_achievement(constants::ITEM_ID_ALIEN, data.get_puzzle(3));
			} else {
				terminal::write_full(data.get_response(1));
			}

		} else if recipient_id == constants::ITEM_ID_GUNSLINGER && gift_id == constants::ITEM_ID_MAGAZINE {
			let cartridge = data.get_item_by_id_certain(constants::ITEM_ID_CARTRIDGE);
			self.location.borrow_mut().insert_item(cartridge.clone(), true);
			self.inventory.remove_item_certain(gift_id);
			self.complete_obstruction_achievement(constants::ITEM_ID_GUNSLINGER, data.get_puzzle(10));

		} else if recipient_id == constants::ITEM_ID_LION && gift_edible {
			self.inventory.remove_item_certain(gift_id);
			if gift_id == constants::ITEM_ID_KOHLRABI {
				terminal::write_full(data.get_response(60));
				self.die(data);
			} else {
				terminal::write_full(data.get_response(61));
			}

		} else if recipient_id == constants::ITEM_ID_SKELETON && gift_id == constants::ITEM_ID_MILK {
			let brooch = data.get_item_by_id_certain(constants::ITEM_ID_BROOCH);
			self.location.borrow_mut().insert_item(brooch.clone(), true);
			self.inventory.remove_item_certain(gift_id);
			self.complete_obstruction_achievement(constants::ITEM_ID_SKELETON, data.get_puzzle(19));

		} else if recipient_id == constants::ITEM_ID_TROLL && gift_edible {
			self.inventory.remove_item_certain(gift_id);
			terminal::write_full(data.get_response(154));
			self.die(data);

		} else if recipient_id == constants::ITEM_ID_BEAN && gift_id == constants::ITEM_ID_POTION {
			self.inventory.remove_item_certain(gift_id);
			self.remove_and_insert_item(data, constants::ITEM_ID_BEAN, constants::ITEM_ID_PLANT, 176);

		} else if recipient_id == constants::ITEM_ID_BEAN && gift_id == constants::ITEM_ID_WATER && location_id == constants::LOCATION_ID_HOT {
			self.inventory.remove_item_certain(gift_id);
			if self.inventory.contains_item(recipient_id) {
				self.inventory.remove_item_certain(recipient_id);
			} else {
				self.location.borrow_mut().remove_item_certain(recipient_id);
			}
			self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_BEANSTALK).clone(), true);
			self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_BLOSSOM).clone(), true);
			self.complete_achievement(data.get_puzzle(4));

		} else if recipient_id == constants::ITEM_ID_PLANT && gift_id == constants::ITEM_ID_POTION {
			self.inventory.remove_item_certain(gift_id);
			self.remove_and_insert_item(data, constants::ITEM_ID_PLANT, constants::ITEM_ID_BEAN, 177);

		} else if gift_liquid { // Default response for liquids
			self.inventory.remove_item_certain(gift_id);
			terminal::write_full(&data.get_response_param(175, recipient.borrow().get_shortname()));

		} else { // Default response for non-liquids
			let response = String::from(data.get_response(149)) + recipient.borrow().get_shortname() + data.get_response(88) +
				gift.borrow().get_shortname() + data.get_response(29);
			terminal::write_full(&response);
		}
	}

	pub fn attack(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_BUCCANEER | constants::ITEM_ID_CORSAIR | constants::ITEM_ID_DOGS | constants::ITEM_ID_DRAGON |
				constants::ITEM_ID_GUNSLINGER | constants::ITEM_ID_LION | constants::ITEM_ID_WOLF => {
				terminal::write_full(data.get_response(106))
			},
			constants::ITEM_ID_BOULDER => {
				if self.strong {
					self.complete_obstruction_achievement(constants::ITEM_ID_BOULDER, data.get_puzzle(5));
					self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_DUST).clone(), true);
					let cellar = data.get_location_certain(constants::LOCATION_ID_CELLAR);
					self.location.borrow_mut().set_direction(Direction::Down, Some(cellar.clone()));
					cellar.borrow_mut().set_direction(Direction::Up, Some(self.location.clone()));
					self.strong = false;
				} else {
					terminal::write_full(data.get_response(11));
				}
			}
			_ => {
				terminal::write_full(data.get_response(94));
			},
		}
	}

	pub fn avnarand(&mut self, data: &DataCollection) {
		let robot_present = self.location.borrow().contains_item(constants::ITEM_ID_ROBOT);
		if robot_present {
			self.complete_obstruction_achievement(constants::ITEM_ID_ROBOT, data.get_puzzle(18));
		} else {
			terminal::write_full(data.get_response(86));
		}
	}

	pub fn burn(&mut self, data: &DataCollection, item: &ItemRef) {
		if !self.inventory.contains_item(constants::ITEM_ID_MATCHES) {
			terminal::write_full(data.get_response(92));
			return;
		}
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_BREAD => {
				if self.inventory.contains_item(item_id) {
					self.inventory.remove_item_certain(item_id);
				} else {
					self.location.borrow_mut().remove_item_certain(item_id);
				}
				let toast = data.get_item_by_id_certain(constants::ITEM_ID_TOAST);
				self.location.borrow_mut().insert_item(toast.clone(), true);
				terminal::write_full(data.get_response(12));
			},
			constants::ITEM_ID_TOAST => {
				if self.inventory.contains_item(item_id) {
					self.inventory.remove_item_certain(item_id);
				} else {
					self.location.borrow_mut().remove_item_certain(item_id);
				}
				terminal::write_full(data.get_response(152));
				let at_airlocke = self.location.borrow().is(constants::LOCATION_ID_AIRLOCKE);
				if at_airlocke {
					let out_loc = data.get_location_certain(constants::LOCATION_ID_AIRLOCKEOUT);
					self.location.borrow_mut().set_direction(Direction::Southwest, Some(out_loc.clone()));
					self.location.borrow_mut().set_air(false);
					self.complete_achievement(data.get_puzzle(21));
				} else {
					terminal::write_full(data.get_response(6));
				}
			},
			_ => terminal::write_full(data.get_response(94)),
		}
	}

	pub fn call(&mut self, data: &DataCollection, item: &ItemRef) {
		let panel_present = self.location.borrow().contains_item(constants::ITEM_ID_CONSOLE_FIXED);
		if !panel_present {
			terminal::write_full(data.get_response(101));
			return;
		}

		let callee_id = item.borrow().get_id();
		if callee_id == constants::ITEM_ID_SHIP {
			let console = data.get_item_by_id_certain(constants::ITEM_ID_CONSOLE_BROKEN);
			self.location.borrow_mut().insert_item(console.clone(), true);

			// Player's safe and wake locations must now be west of the checkpoint, rather than east
			self.location_id_safe = constants::LOCATION_ID_SAFE_PIRATES;
			self.location_id_wake = constants::LOCATION_ID_WAKE_PIRATES;

			// Pirates arrive on the Asterbase
			let checkpoint = data.get_location_certain(constants::LOCATION_ID_CHECKPOINT);
			let corsair = data.get_item_by_id_certain(constants::ITEM_ID_CORSAIR);
			checkpoint.borrow_mut().insert_item(corsair.clone(), true);
			let docking_ctrl = data.get_location_certain(constants::LOCATION_ID_DOCKINGCONTROL);
			let buccaneer = data.get_item_by_id_certain(constants::ITEM_ID_BUCCANEER);
			docking_ctrl.borrow_mut().insert_item(buccaneer.clone(), true);

			// Link pirate ship (both item and location) to the docking bay
			let docking = data.get_location_certain(constants::LOCATION_ID_DOCKING);
			let ship_loc = data.get_location_certain(constants::LOCATION_ID_SHIP);
			docking.borrow_mut().insert_item(item.clone(), true);
			docking.borrow_mut().set_direction(Direction::East, Some(ship_loc.clone()));
			docking.borrow_mut().set_direction(Direction::Southeast, Some(ship_loc.clone()));

			// Unlink the existing shuttle from the southeast of the docking bay
			let shuttle = data.get_location_certain(constants::LOCATION_ID_SHUTTLE);
			shuttle.borrow_mut().set_direction(Direction::South, None);

			self.complete_obstruction_achievement(constants::ITEM_ID_CONSOLE_FIXED, data.get_puzzle(7));
		} else {
			terminal::write_full(data.get_response(94));
		}
	}

	pub fn chimbu(&mut self, data: &DataCollection) {
		let fairy_present = self.location.borrow().contains_item(constants::ITEM_ID_FAIRY);
		let envelope = data.get_item_by_id_certain(constants::ITEM_ID_ENVELOPE);
		let tooth_within = envelope.borrow().contains_item(constants::ITEM_ID_TOOTH);
		if fairy_present && tooth_within {
			let coin = data.get_item_by_id_certain(constants::ITEM_ID_COIN);
			envelope.borrow_mut().remove_item_certain(constants::ITEM_ID_TOOTH);
			envelope.borrow_mut().set_within(Some(coin.clone()));
			self.complete_obstruction_achievement(constants::ITEM_ID_FAIRY, data.get_puzzle(9));
		} else {
			terminal::write_full(data.get_response(86));
		}
	}

	pub fn take(&mut self, data: &DataCollection, item: &ItemRef) {
		if self.inventory.contains_item(item.borrow().get_id()) && !item.borrow().is_liquid() {
			terminal::write_full(data.get_response(145));
			return;
		}

		if !item.borrow().is_portable() {
			terminal::write_full(data.get_response(146));
			return;
		}

		if !self.inventory.can_fit(&item) {
			terminal::write_full(data.get_response(147));
			return;
		}

		// Liquids require a container
		if item.borrow().is_liquid() {
			self.insert(data, item);
			return;
		}

		self.location.borrow_mut().remove_item_certain(item.borrow().get_id());
		self.insert_item(item.clone());

		if item.borrow().is_wearable() {
			terminal::write_full(data.get_response(156));
		} else {
			terminal::write_full(data.get_response(148));
		}
	}

	pub fn cook(&mut self, data: &DataCollection, item: &ItemRef) {
		if !self.location.borrow().contains_item(constants::ITEM_ID_CAULDRON) {
			terminal::write_full(data.get_response(76));
			return;
		}

		let cauldron = data.get_item_by_id_certain(constants::ITEM_ID_CAULDRON);
		if !cauldron.borrow().is_empty() {
		        terminal::write_full(data.get_response(17));
		        return;
		}

		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_KOHLRABI => {
			    self.inventory.remove_item_certain(constants::ITEM_ID_KOHLRABI);
			    let stew = data.get_item_by_id_certain(constants::ITEM_ID_STEW);
			    cauldron.borrow_mut().set_within(Some(stew.clone()));
			    terminal::write_full(data.get_response(14));
			},
			constants::ITEM_ID_RADISHES => {
				self.inventory.remove_item_certain(constants::ITEM_ID_RADISHES);
				let elixir = data.get_item_by_id_certain(constants::ITEM_ID_ELIXIR);
				cauldron.borrow_mut().set_within(Some(elixir.clone()));
				self.complete_achievement(data.get_puzzle(17));
			},
			_ => terminal::write_full(data.get_response(94)),
		}
	}

	// Describe an item in the player's inventory or at the player's location
	pub fn describe(&mut self, data: &DataCollection, item: &ItemRef) {
		self.observe_item(data, item, Player::describe_final);
	}

	fn describe_final(&mut self, data: &DataCollection, item: &ItemRef) {
		terminal::write_full(&item.borrow().mk_full_string(data.get_response(26)));
	}

	pub fn drink(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().is_liquid() {
			terminal::write_full(data.get_response(93));
			return;
		}

		let item_id = item.borrow().get_id();
		self.inventory.remove_item_certain(item_id);
		terminal::write_full(data.get_response(30));

		match item_id {
			constants::ITEM_ID_AQUA => terminal::write_full(data.get_response(31)),
			constants::ITEM_ID_WATER => terminal::write_full(data.get_response(35)),
			constants::ITEM_ID_STEW => terminal::write_full(data.get_response(34)),
			constants::ITEM_ID_ELIXIR => {
				self.strong = true;
				terminal::write_full(data.get_response(32));
			}
			constants::ITEM_ID_POTION => {
				terminal::write_full(data.get_response(33));
				self.die(data);
			},
			_ => terminal::write_full(data.get_response(86)),
		}
	}

	pub fn drop(&mut self, data: &DataCollection, item: &ItemRef) {
		self.release_item(data, item, false);
	}

	pub fn empty(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().is_container() {
			terminal::write_full(&data.get_response_param(24, &item.borrow().get_shortname()));
			return;
		}

		let within_ref = item.borrow_mut().remove_within();
		match within_ref {
			None => terminal::write_full(data.get_response(40)),
			Some(within) => {
				let is_liquid = within.borrow().is_liquid();
				if is_liquid {
					terminal::write_full(data.get_response(42));
				} else {
					let item_id = item.borrow().get_id();
					if self.inventory.contains_item(item_id) {
						self.inventory.insert_item(within.clone());
						terminal::write_full(&data.get_response_param(41, &within.borrow().get_shortname()));
					} else {
						self.location.borrow_mut().insert_item(within.clone(), true);
						terminal::write_full(&data.get_response_param(43, &within.borrow().get_shortname()));
					}
				}
			},
		}
	}

	pub fn feed(&mut self, data: &DataCollection, item: &ItemRef) {
		if item.borrow().is_recipient() {
			self.feed_dative(data, item);
		} else {
			if !self.inventory.contains_item(item.borrow().get_id()) {
				terminal::write_full(&data.get_response_param(74, &item.borrow().get_shortname()));
				return;
			}
			self.feed_accusative(data, item);
		}
	}

	// Feed, where the direct object is known and the indirect is not
	fn feed_accusative(&mut self, data: &DataCollection, direct: &ItemRef) {

		// Find out what player wants to feed it to
		let indirect_str = terminal::read_question(&data.get_response_param(159, direct.borrow().get_shortname()));

		// Feed food to recipient, if it exists and player is carrying it
		match data.get_item_by_name(indirect_str[0].clone()) {
			None => terminal::write_full(data.get_response(98)),
			Some(indirect) => {
				let indirect_id = indirect.borrow().get_id();
				if self.inventory.contains_item(indirect_id) || self.location.borrow().contains_item(indirect_id) {
					self.feed_item_unknown(data, direct, indirect);
				} else {
					terminal::write_full(&data.get_response_param(100, &indirect.borrow().get_shortname()));
				}
			},
		}
	}

	// Feed, where the indirect object is known and the direct is not
	fn feed_dative(&mut self, data: &DataCollection, indirect: &ItemRef) {

		// Find out what player wants to feed to it
		let direct_str = terminal::read_question(&data.get_response_param(160, indirect.borrow().get_shortname()));

		// Feed food to recipient, if it exists and player is carrying it
		match data.get_item_by_name(direct_str[0].clone()) {
			None => terminal::write_full(data.get_response(98)),
			Some(direct) => {
				let direct_id = direct.borrow().get_id();
				if self.inventory.contains_item(direct_id) {
					self.feed_item_unknown(data, direct, indirect);
				} else {
					terminal::write_full(&data.get_response_param(74, &direct.borrow().get_shortname()));
				}
			},
		}
	}

	// Attempt to feed item, when we are not sure if the recipient can accept or not
	fn feed_item_unknown(&mut self, data: &DataCollection, direct: &ItemRef, indirect: &ItemRef) {
		if !indirect.borrow().is_recipient() {
			terminal::write_full(&data.get_response_param(79, indirect.borrow().get_shortname()));
			return;
		}
		self.transfer_item(data, direct, indirect);
	}

	pub fn fish(&mut self, data: &DataCollection) {
		if !self.inventory.contains_item(constants::ITEM_ID_NET) {
			terminal::write_full(data.get_response(77));
			return;
		}
		let glint_present = self.location.borrow().contains_item(constants::ITEM_ID_GLINT);
		if !glint_present {
			terminal::write_full(data.get_response(80));
			return;
		}
		let nugget = data.get_item_by_id_certain(constants::ITEM_ID_NUGGET);
		if !self.inventory.can_fit(nugget) {
			terminal::write_full(data.get_response(172));
			return;
		}
		self.inventory.insert_item(nugget.clone());
		self.complete_obstruction_achievement(constants::ITEM_ID_GLINT, data.get_puzzle(14));
	}

	pub fn give(&mut self, data: &DataCollection, item: &ItemRef) {
		// Find out what player wants to give item to
		let recipient_str = terminal::read_question(&data.get_response_param(168, item.borrow().get_shortname()));

		// Give item to recipient, if it exists and player is carrying it
		match data.get_item_by_name(recipient_str[0].clone()) {
			None => terminal::write_full(data.get_response(98)),
			Some(recipient) => {
				let recipient_id = recipient.borrow().get_id();
				if self.inventory.contains_item(recipient_id) || self.location.borrow().contains_item(recipient_id) {
					self.transfer_item(data, item, recipient);
				} else {
					terminal::write_full(&data.get_response_param(100, &recipient.borrow().get_shortname()));
				}
			},
		}
	}

	// Have player travel to an adjacent location
	pub fn go(&mut self, data: &DataCollection, dir: Direction) {

		let location_before = self.location.clone();

		let move_result = match dir {
			Direction::Back => self.try_move_back(),
			_ => self.try_move_other(dir),
		};

		let next_location_option = move_result.0;
		let death = move_result.1;
		let response_code_option = move_result.2;

		// Update location if returned
		match next_location_option {
			None => {},
			Some (next_location) => {
				self.location = next_location;
				if self.location.borrow().can_reach(&location_before) {
					self.previous = Some(location_before);
				} else {
					self.previous = None;
				}
				terminal::write_full(&self.get_effective_appearance(data, self.location.borrow().mk_arrival_string()));
				self.location.borrow_mut().set_visited(true);
			}
		}

		// Process death
		if death {
			self.die(data);
		}

		// Print any returned responses
		match response_code_option {
			None => {},
			Some(response_code) => terminal::write_full(data.get_response(response_code)),
		}
	}

	// Attempt to move to previous location
	// Return a tuple representing the next location (if move is successful), whether the player died, and any response message to be printed
	fn try_move_back(&mut self) -> (Option<LocationRef>, bool, Option<u32>) {
		match self.previous.clone() {
			None => (None, false, Some(71)),
			Some(prev) => (Some(prev.clone()), false, None),
		}
	}

	// Attempt to move to some location, which may not be reachable from the current location
	// Return a tuple representing the next location (if move is successful), whether the player died, and any response message to be printed
	fn try_move_other(&mut self, dir: Direction) -> (Option<LocationRef>, bool, Option<u32>) {
		let loc_clone = self.location.clone();
		let self_loc = loc_clone.borrow();

		match self_loc.get_direction(&dir) {
			None => {
				return (None, false, Some(72));
			},
			Some(next) => {
				if !self.is_previous_loc(&next) {
					match (**self_loc).get_obstruction() {
						None => {},
						Some(obstruction) => {
							// FIXME: tidy this whole area
							let mut next_loc_option: Option<LocationRef> = None;
							let mut death = false;
							let mut response_code: u32 = 108; // FIXME: tailor to individual obstructions
							if obstruction.borrow().is(constants::ITEM_ID_BUCCANEER) {
								if !self.has_invisibility() {
									response_code = 120;
								} else {
									next_loc_option = Some(next.clone());
									response_code = 118;
								}
							} else if obstruction.borrow().is(constants::ITEM_ID_CORSAIR) {
								if self.inventory.contains_item(constants::ITEM_ID_BOOTS) {
									death = true;
									response_code = 117;
								} else {
									response_code = 119;
								}
							}
							return (next_loc_option, death, Some(response_code));
						}
					}
				}

				if !next.borrow().has_air() && !self.inventory.has_air() { // Refuse to proceed if there is no air at the next location
					return (None, false, Some(66));
				}
				if dir == Direction::Up && self.has_gravity() && self_loc.needsno_gravity() { // Gravity is preventing the player from going up
					return (None, false, Some(67));
				}
				if dir == Direction::Down && self.has_gravity() && !self_loc.has_floor() {
					return (None, false, Some(68));
				}

				return self.try_move_to(&next);
			},
		}
	}

	// Attempt to go to a location known to be adjacent
	// Return a tuple representing the next location (if move is successful), whether the player died, and any response message to be printed
	fn try_move_to(&mut self, next: &LocationRef) -> (Option<LocationRef>, bool, Option<u32>) {
		let mut rng = rand::thread_rng();
		let death_rand: u32 = rng.gen();
		let death = death_rand % self.death_divisor == 0;
		if !self.has_light() && !next.borrow().has_light() && death {
			return (None, true, Some(91));
		} else if !self.has_nosnomp() && !next.borrow().has_nosnomp() && death {
			return (None, true, Some(142));
		} else {
			return (Some(next.clone()), false, None);
		}
	}

	// Return whether a location is the last place the player was
	fn is_previous_loc(&self, next: &LocationRef) -> bool {
		let previous = self.previous.clone();
		match previous {
			None => return false,
			Some(prev) => prev.borrow().get_id() == next.borrow().get_id(),
		}
	}

	pub fn ignore(&mut self, data: &DataCollection, item: &ItemRef) {
		if item.borrow().is(constants::ITEM_ID_TROLL) {
			self.complete_obstruction_achievement(constants::ITEM_ID_TROLL, data.get_puzzle(22));
		} else {
			 terminal::write_full(data.get_response(55));
		}
	}

	pub fn insert(&mut self, data: &DataCollection, item: &ItemRef) {
		match item.borrow().has_problem_inserting() {
			None => {},
			Some(reason) => {
					terminal::write_full(&data.get_response_param(reason, item.borrow().get_shortname()));
					return;
			},
		}

		// Find out what player wants to insert it into
		let container_str = terminal::read_question(&data.get_response_param(161, item.borrow().get_shortname()));

		// Insert item into container, if container exists and is present
		match data.get_item_by_name(container_str[0].clone()) {
			None => terminal::write_full(data.get_response(98)),
			Some(container) => {
				let container_id = container.borrow().get_id();
				if self.inventory.contains_item(container_id) || self.location.borrow().contains_item(container_id) {
					self.insert_final(data, item, container)
				} else {
					terminal::write_full(&data.get_response_param(100, container.borrow().get_shortname()));
				}
			},
		}
	}

	fn insert_final(&mut self, data: &DataCollection, item: &ItemRef, container: &ItemRef) {
		match container.borrow().has_problem_accepting(item) {
			None => {},
			Some(reason) => {
					terminal::write_full(&data.get_response_param(reason, container.borrow().get_shortname()));
					return;
			},
		}

		let item_id = item.borrow().get_id();
		let mut self_loc = self.location.borrow_mut();
		if self_loc.contains_item(item_id) {
			if !self.inventory.can_fit(&item) {
				terminal::write_full(data.get_response(147));
				return;
			}
			self_loc.remove_item_certain(item_id);
		} else if self.inventory.contains_item(item_id) {
			self.inventory.remove_item_certain(item_id);
		}
		container.borrow_mut().set_within(Some(item.clone()));
		terminal::write_full(data.get_response(58));
	}

	pub fn knit(&mut self, data: &DataCollection) {
		if !self.inventory.contains_item(constants::ITEM_ID_NEEDLES) || !self.inventory.contains_item(constants::ITEM_ID_YARN) {
			terminal::write_full(data.get_response(77));
			return;
		}
		self.inventory.remove_item_certain(constants::ITEM_ID_YARN);
		self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_JUMPER).clone(), true);
		self.complete_achievement(data.get_puzzle(11));
	}

	pub fn light(&mut self, data: &DataCollection, item: &ItemRef) {
		self.switch_item(data, item, true);
	}

	// Return a description of what the player sees when they look
	pub fn get_look(&self, data: &DataCollection) -> String {
		self.get_effective_appearance(data, self.mk_location_string())
	}

	pub fn quench(&mut self, data: &DataCollection, item: &ItemRef) {
		self.switch_item(data, item, false);
	}

	pub fn get_score_str(&self, data: &DataCollection) -> String {
		let total_score = self.calculate_score(data);
		String::from(data.get_response(132)) + &total_score.to_string() +
		data.get_response(133) + &data.get_max_score().to_string() +
		data.get_response(128) + &self.deaths.to_string() +
		data.get_response(129) + &self.instructions.to_string() +
		data.get_response(131) + &self.hints.to_string() + data.get_response(130)
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

	pub fn mk_inventory_string(&self) -> String {
		self.inventory.mk_string()
	}

	pub fn mk_location_string(&self) -> String {
		self.location.borrow().mk_full_string()
	}

	pub fn play(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_WHISTLE => {
				let tune_words = terminal::read_question(data.get_response(163));
				let tune = &tune_words[0];
				terminal::write_full(&data.get_response_param(121, tune));

				if tune == data.get_response(13) {
					let lion_present = self.location.borrow().contains_item(constants::ITEM_ID_LION);
					if lion_present {
						let lion = data.get_item_by_id_certain(constants::ITEM_ID_LION);
						let lion_obstruction = lion.borrow().is_obstruction();
						if lion_obstruction {
							lion.borrow_mut().set_obstruction(false);
							self.complete_achievement(data.get_puzzle(12));
						}
					}
				}
			},
			constants::ITEM_ID_PLAYER => {
				self.play_player(data, item);
			},
			_ => {
				terminal::write_full(data.get_response(94));
			},
		}
	}

	pub fn pour(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().is_liquid() {
			terminal::write_full(data.get_response(99));
			return;
		}

		// Find out what player wants to pour liquid onto
		let recipient_str = terminal::read_question(&data.get_response_param(174, item.borrow().get_shortname()));

		// Pour liquid onto recipient
		match data.get_item_by_name(recipient_str[0].clone()) {
			None => terminal::write_full(data.get_response(98)),
			Some(recipient) => {
				let recipient_id = recipient.borrow().get_id();
				if self.inventory.contains_item(recipient_id) || self.location.borrow().contains_item(recipient_id) {
					self.transfer_item(data, item, recipient);
				} else {
					terminal::write_full(&data.get_response_param(100, &recipient.borrow().get_shortname()));
				}
			},
		}
	}

	pub fn read(&mut self, data: &DataCollection, item: &ItemRef) {
		self.observe_item(data, item, Player::read_final);
	}

	fn read_final(&mut self, data: &DataCollection, item: &ItemRef) {
		terminal::write_full(&item.borrow().mk_writing_string(data.get_response(107), data.get_response(169)));
	}

	pub fn repair(&mut self, data: &DataCollection, item: &ItemRef) {
		if item.borrow().is(constants::ITEM_ID_CONSOLE_FIXED) {
			terminal::write_full(data.get_response(157));
		} else if item.borrow().is(constants::ITEM_ID_CONSOLE_BROKEN) {
			if !self.inventory.contains_item(constants::ITEM_ID_WIRE) {
				terminal::write_full(data.get_response(158));
			} else {
				let panel = data.get_item_by_id_certain(constants::ITEM_ID_CONSOLE_FIXED);
				self.location.borrow_mut().insert_item(panel.clone(), true);
				self.inventory.remove_item_certain(constants::ITEM_ID_WIRE);
				self.complete_obstruction_achievement(constants::ITEM_ID_CONSOLE_BROKEN, data.get_puzzle(6));
			}
		} else {
			terminal::write_full(data.get_response(94));
		}
	}

	pub fn rob(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_BUCCANEER => {
				let kill_condition = !self.has_invisibility();
				self.rob_pirate(data, item, constants::ITEM_ID_MEDALLION, kill_condition, 116, 16);
			},
			constants::ITEM_ID_CORSAIR => {
				let kill_condition = self.inventory.contains_item(constants::ITEM_ID_BOOTS);
				self.rob_pirate(data, item, constants::ITEM_ID_KEY, kill_condition, 113, 15);
			},
			_ => terminal::write_full(data.get_response(94)),
		}
	}

	pub fn rub(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_LAMP => terminal::write_full(data.get_response(47)),
			constants::ITEM_ID_DRAGON => {
				let tooth = data.get_item_by_id_certain(constants::ITEM_ID_TOOTH);
				self.location.borrow_mut().insert_item(tooth.clone(), true);
				self.complete_obstruction_achievement(constants::ITEM_ID_DRAGON, data.get_puzzle(8));
			},
			constants::ITEM_ID_PENDANT => {
				let thor = data.get_location_certain(constants::LOCATION_ID_THOR);
				let rod = data.get_item_by_id_certain(constants::ITEM_ID_ROD);
				self.unlink_item(data, rod);
				thor.borrow_mut().insert_item(rod.clone(), true);
				terminal::write_full(data.get_response(111));
			},
			_ => terminal::write_full(data.get_response(89)),
		}
	}

	pub fn say(&mut self, data: &DataCollection, statement: &str) {
		terminal::write_full(&data.get_response_param(170, statement));
		if self.location.borrow().contains_item(constants::ITEM_ID_CORSAIR) { // Pirate hears player
			terminal::write_full(data.get_response(114));
			self.die(data);
			return;
		}
		if statement == data.get_response(171) {
			let alien_present = self.location.borrow().contains_item(constants::ITEM_ID_ALIEN);
			if alien_present {
				let chart_used = data.get_item_by_id_certain(constants::ITEM_ID_CHART).borrow().is_retired();
				let transmitter_used = data.get_item_by_id_certain(constants::ITEM_ID_TRANSMITTER).borrow().is_retired();
				if transmitter_used {
					terminal::write_full(data.get_response(53));
				} else if chart_used {
					terminal::write_full(data.get_response(51));
				} else {
					terminal::write_full(data.get_response(52));
				}
			}
		}
	}

	pub fn sleep(&mut self, data: &DataCollection) {
		self.teleport(data, data.get_tp_map_sleep(), false, 141, 140);
	}

	pub fn stare(&mut self, data: &DataCollection) {
		if !self.has_light() {
			terminal::write_full(data.get_response(15));
			return;
		}
		if self.location.borrow().is(constants::LOCATION_ID_REFLECTION) || self.inventory.contains_item(constants::ITEM_ID_MIRROR) {
			if self.has_invisibility() {
				terminal::write_full(data.get_response(124));
			} else if self.strong {
				terminal::write_full(data.get_response(127));
			} else {
				terminal::write_full(data.get_response(125));
			}
		} else {
			terminal::write_full(data.get_response(126));
		}
	}

	pub fn tezazzle(&mut self, data: &DataCollection) {
		self.teleport(data, data.get_tp_map_witch(), true, 86, 165);
	}

	pub fn throw(&mut self, data: &DataCollection, item: &ItemRef) {
		terminal::write_full(data.get_response(151));
		self.release_item(data, item, true);
	}

	pub fn xyro(&mut self, data: &DataCollection) {
		let wizard_present = self.location.borrow().contains_item(constants::ITEM_ID_WIZARD);
		let mirror_present = self.inventory.contains_item(constants::ITEM_ID_MIRROR);
		if wizard_present {
			if self.has_invisibility() {
				terminal::write_full(data.get_response(167));
			} else if mirror_present {
				self.complete_obstruction_achievement(constants::ITEM_ID_WIZARD, data.get_puzzle(23));
			} else {
				terminal::write_full(data.get_response(166));
				self.die(data);
			}
		} else {
			terminal::write_full(data.get_response(139));
		}
	}

	pub fn ziqua(&mut self, data: &DataCollection) {
		let at_treetop = self.location.borrow().is(constants::LOCATION_ID_TREETOP);
		if at_treetop {
			let acorn = data.get_item_by_id_certain(constants::ITEM_ID_ACORN);
			let acorn_is_new = acorn.borrow().is_new();
			if acorn_is_new {
				let garden = data.get_location_certain(constants::LOCATION_ID_GARDEN);
				garden.borrow_mut().insert_item(acorn.clone(), true);
				self.complete_achievement(data.get_puzzle(0));
				return;
			}
		}
		terminal::write_full(data.get_response(86));
	}
}
