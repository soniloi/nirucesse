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
	previous_true: LocationRef,
	previous: Option<LocationRef>,
	achievement_count: u32, // number of puzzles player has solved
	playing: bool, // whether player is currently playing
	hints: u32, // number of hints player has requested
	instructions: u32, // number of instructions player has entered
	deaths: u32, // number of times player has died
	alive: bool,
	strong: bool,
}

impl Player {

	pub fn new(initial: LocationRef) -> Player {
		Player {
			inventory: Inventory::new(16),
			location: initial.clone(),
			previous_true: initial.clone(),
			previous: None,
			achievement_count: 0u32,
			playing: true,
			hints: 0u32,
			instructions: 0u32,
			deaths: 0u32,
			alive: true,
			strong: false,
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

	pub fn contains_item(&self, item: &ItemRef) -> bool {
		self.inventory.contains_item(item.borrow().get_id())
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
		self.location = data.get_location_wake().clone();
	}

	pub fn drop_on_death(&mut self, safe_loc: &LocationRef) {
		self.inventory.drop_all(&self.previous_true, safe_loc, true, true);
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
		self.get_effective_description(String::from(data.get_response("cantseeh")), String::from(data.get_response("cantseed")), default_description)
	}

	pub fn get_location_stubname(&self) -> String {
		self.get_effective_description(String::from("???"), String::from("???"), self.location.borrow().get_shortname())
	}

	fn observe_item(&mut self, data: &DataCollection, item: &ItemRef, act: ItemManipFinalFn) {
		if !self.has_light() {
			terminal::write_full(data.get_response("cantseed"));
			return;
		}
		self.manipulate_item_present(data, item, act);
	}

	// Manipulate an item present either in the player's inventory or at the player's location
	fn manipulate_item_present(&mut self, data: &DataCollection, item: &ItemRef, act: ItemManipFinalFn) {
		let item_id = item.borrow().get_id();
		if !self.inventory.contains_item(item_id) && !self.location.borrow().contains_item(item_id) {
			terminal::write_full(&data.get_response_param("noseeh", &item.borrow().get_shortname()));
			return;
		}
		act(self, data, item);
	}

	// Manipulate an item present strictly in the player's inventory
	fn manipulate_item_inventory(&mut self, data: &DataCollection, item: &ItemRef, act: ItemManipFinalFn) {
		let item_id = item.borrow().get_id();
		if !self.inventory.contains_item(item_id) {
			terminal::write_full(&data.get_response_param("nocarry", &item.borrow().get_shortname()));
			return;
		}
		act(self, data, item);
	}

	fn complete_obstruction_achievement(&mut self, obstruction_id: u32, response: &str) {
		self.location.borrow_mut().remove_item_certain(obstruction_id);
		self.complete_achievement(response);
	}

	fn complete_achievement(&mut self, response: &str) {
		self.achievement_count = self.achievement_count + 1;
		terminal::write_full(response);
	}

	fn teleport(&mut self, data: &DataCollection, tp_map: &HashMap<u32, u32>, permanent: bool,
		response_tag_no_teleport: &str, response_tag_teleport: &str) {
		let loc_id = self.location.borrow().get_id();
		match tp_map.get(&loc_id) {
			None => terminal::write_full(data.get_response(response_tag_no_teleport)),
			Some(next_id) => {
				self.inventory.drop_all(&self.location, data.get_location_safe(), false, permanent);
				self.location = data.get_location_certain(*next_id).clone();
				self.previous = None;
				self.location.borrow_mut().release_temporary(&mut self.inventory);
				terminal::write_full(data.get_response(response_tag_teleport));
			},
		}
	}

	pub fn attack(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::attack_final);
	}

	fn attack_final(&mut self, data: &DataCollection, item: &ItemRef) {
		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_DOGS | constants::ITEM_ID_DRAGON | constants::ITEM_ID_LION | constants::ITEM_ID_WOLF => {
				terminal::write_full(data.get_response("nowiseat"))
			},
			constants::ITEM_ID_BOULDER => {
				if self.strong {
					self.complete_obstruction_achievement(constants::ITEM_ID_BOULDER, data.get_puzzle("bouldpul"));
					self.location.borrow_mut().insert_item(data.get_item_by_id_certain(constants::ITEM_ID_DUST).clone(), true);
					self.strong = false;
				} else {
					terminal::write_full(data.get_response("bouldatt"));
				}
			}
			_ => {
				terminal::write_full(data.get_response("nonohow"));
			},
		}
	}

	pub fn avnarand(&mut self, data: &DataCollection) {
		let robot_present = self.location.borrow().contains_item(constants::ITEM_ID_ROBOT);
		if robot_present {
			self.complete_obstruction_achievement(constants::ITEM_ID_ROBOT, data.get_puzzle("robot"));
		} else {
			terminal::write_full(data.get_response("nohappen"));
		}
	}

	pub fn burn(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::burn_final);
	}

	fn burn_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if !self.inventory.contains_item(constants::ITEM_ID_MATCHES) {
			terminal::write_full(data.get_response("nomatch"));
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
				terminal::write_full(data.get_response("bread"));
			},
			constants::ITEM_ID_TOAST => {
				if self.inventory.contains_item(item_id) {
					self.inventory.remove_item_certain(item_id);
				} else {
					self.location.borrow_mut().remove_item_certain(item_id);
				}
				terminal::write_full(data.get_response("toast"));
				let at_airlocke = self.location.borrow().is(constants::LOCATION_ID_AIRLOCKE);
				if at_airlocke {
					let out_loc = data.get_location_certain(constants::LOCATION_ID_AIRLOCKEOUT);
					self.location.borrow_mut().set_direction(Direction::Southwest, out_loc.clone());
					self.location.borrow_mut().set_air(false);
					self.complete_achievement(data.get_puzzle("toastalm"));
				} else {
					terminal::write_full(data.get_response("ashmouse"));
				}
			},
			_ => {
				terminal::write_full(data.get_response("nonohow"));
			},
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
			self.complete_obstruction_achievement(constants::ITEM_ID_FAIRY, data.get_puzzle("fairy"));
		} else {
			terminal::write_full(data.get_response("nohappen"));
		}
	}

	// Have player attempt to pick up item from current location
	pub fn take(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::take_final);
	}

	fn take_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if self.contains_item(item) && !item.borrow().is_liquid() {
			terminal::write_full(data.get_response("takealre"));
			return;
		}

		if !item.borrow().is_portable() {
			terminal::write_full(data.get_response("takenoca"));
			return;
		}

		if !self.inventory.can_fit(&item) {
			terminal::write_full(data.get_response("takeover"));
			return;
		}

		// Liquids require a container
		if item.borrow().is_liquid() {
			self.insert_portable(data, item);
			return;
		}

		self.location.borrow_mut().remove_item_certain(item.borrow().get_id());
		self.insert_item(item.clone());

		if item.borrow().is_wearable() {
			terminal::write_full(data.get_response("wear"));
		} else {
			terminal::write_full(data.get_response("takegood"));
		}
	}

	pub fn cook(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::cook_final);
	}

	fn cook_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if !self.location.borrow().contains_item(constants::ITEM_ID_CAULDRON) {
			terminal::write_full(data.get_response("nocooker"));
			return;
		}

		let cauldron = data.get_item_by_id_certain(constants::ITEM_ID_CAULDRON);
		if !cauldron.borrow().is_empty() {
		        terminal::write_full(data.get_response("caulfull"));
		        return;
		}

		let item_id = item.borrow().get_id();
		match item_id {
			constants::ITEM_ID_KOHLRABI => {
			        self.inventory.remove_item_certain(constants::ITEM_ID_KOHLRABI);
			        let stew = data.get_item_by_id_certain(constants::ITEM_ID_STEW);
			        cauldron.borrow_mut().set_within(Some(stew.clone()));
			        terminal::write_full(data.get_response("cabbcook"));
			},
			constants::ITEM_ID_RADISHES => {
				self.inventory.remove_item_certain(constants::ITEM_ID_RADISHES);
				let elixir = data.get_item_by_id_certain(constants::ITEM_ID_ELIXIR);
				cauldron.borrow_mut().set_within(Some(elixir.clone()));
				terminal::write_full(data.get_puzzle("radicook"));
			},
			_ => terminal::write_full(data.get_response("nonohow")),
		}
	}

	// Describe an item in the player's inventory or at the player's location
	pub fn describe(&mut self, data: &DataCollection, item: &ItemRef) {
		self.observe_item(data, item, Player::describe_final);
	}

	fn describe_final(&mut self, data: &DataCollection, item: &ItemRef) {
		terminal::write_full(&item.borrow().mk_full_string(data.get_response("descstar"), data.get_response("dotend")));
	}

	pub fn drink(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_inventory(data, item, Player::drink_final);
	}

	fn drink_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().is_liquid() {
			terminal::write_full(data.get_response("nonodrin"));
			return;
		}

		let item_id = item.borrow().get_id();
		self.inventory.remove_item_certain(item_id);
		terminal::write_full(data.get_response("drink"));

		match item_id {
			constants::ITEM_ID_AQUA => terminal::write_full(data.get_response("drinkaqu")),
			constants::ITEM_ID_WATER => terminal::write_full(data.get_response("drinkwat")),
			constants::ITEM_ID_STEW => terminal::write_full(data.get_response("drinkste")),
			constants::ITEM_ID_ELIXIR => {
				self.strong = true;
				terminal::write_full(data.get_response("drinkeli"));
			}
			constants::ITEM_ID_POTION => {
				terminal::write_full(data.get_response("drinkpot"));
				self.die(data);
			},
			_ => terminal::write_full(data.get_response("nohappen")),
		}
	}

	// Have player attempt to drop item from inventory to current location
	pub fn drop(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_inventory(data, item, Player::drop_final);
	}

	fn drop_final(&mut self, data: &DataCollection, item: &ItemRef) {
		let it = self.inventory.remove_item_certain(item.borrow().get_id());

		// When dropped, liquids drain away
		let liquid = it.borrow().is_liquid();
		if !liquid {
			self.location.borrow_mut().insert_item(it, true);
			terminal::write_full(data.get_response("dropgood"));
		} else {
			terminal::write_full(data.get_response("emptliqu"));
		}

		// Specific item drops
		if item.borrow().is(constants::ITEM_ID_LION) {
			let wolf_present = self.location.borrow().contains_item(constants::ITEM_ID_WOLF);
			if wolf_present {
				self.complete_obstruction_achievement(constants::ITEM_ID_WOLF, data.get_puzzle("lionwolf"));
			}
		}
	}

	pub fn empty(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::empty_final);
	}

	fn empty_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().is_container() {
			terminal::write_full(&data.get_response_param("contnot", &item.borrow().get_shortname()));
			return;
		}

		let within_ref = item.borrow_mut().remove_within();
		match within_ref {
			None => terminal::write_full(data.get_response("emptalre")),
			Some(within) => {
				let is_liquid = within.borrow().is_liquid();
				if is_liquid {
					terminal::write_full(data.get_response("emptliqu"));
				} else {
					let item_id = item.borrow().get_id();
					if self.inventory.contains_item(item_id) {
						self.inventory.insert_item(within.clone());
						terminal::write_full(&data.get_response_param("emptcarr", &within.borrow().get_shortname()));
					} else {
						self.location.borrow_mut().insert_item(within.clone(), true);
						terminal::write_full(&data.get_response_param("emptloca", &within.borrow().get_shortname()));
					}
				}
			},
		}
	}

	pub fn feed(&mut self, data: &DataCollection, item: &ItemRef) {
		if item.borrow().is_recipient() {
			self.manipulate_item_present(data, item, Player::feed_dative);
		} else {
			self.manipulate_item_inventory(data, item, Player::feed_accusative);
		}
	}

	// Feed, where the direct object is known and the indirect is not
	fn feed_accusative(&mut self, data: &DataCollection, direct: &ItemRef) {

		// Find out what player wants to feed it to
		let indirect_str = terminal::read_question(&data.get_response_param("whatfeac", direct.borrow().get_shortname()));

		// Feed food to recipient, if it exists and player is carrying it
		match data.get_item_by_name(indirect_str[0].clone()) {
			None => terminal::write_full(data.get_response("nonowhat")),
			Some(indirect) => {
				let indirect_id = indirect.borrow().get_id();
				if self.inventory.contains_item(indirect_id) || self.location.borrow().contains_item(indirect_id) {
					self.feed_final(data, direct, indirect)
				} else {
					terminal::write_full(&data.get_response_param("noseeh", &indirect.borrow().get_shortname()));
				}
			},
		}
	}

	// Feed, where the indirect object is known and the direct is not
	fn feed_dative(&mut self, data: &DataCollection, indirect: &ItemRef) {

		// Find out what player wants to feed to it
		let direct_str = terminal::read_question(&data.get_response_param("whatfeda", indirect.borrow().get_shortname()));

		// Feed food to recipient, if it exists and player is carrying it
		match data.get_item_by_name(direct_str[0].clone()) {
			None => terminal::write_full(data.get_response("nonowhat")),
			Some(direct) => {
				let direct_id = direct.borrow().get_id();
				if self.inventory.contains_item(direct_id) {
					self.feed_final(data, direct, indirect)
				} else {
					terminal::write_full(&data.get_response_param("nocarry", &direct.borrow().get_shortname()));
				}
			},
		}
	}

	fn feed_final(&mut self, data: &DataCollection, direct: &ItemRef, indirect: &ItemRef) {

		// Cannot feed non-feedable items
		if !indirect.borrow().is_recipient() {
			terminal::write_full(&data.get_response_param("nofeed", indirect.borrow().get_shortname()));
			return;
		}

		// The lion's reactions when we attempt to feed her various things
		if indirect.borrow().is(constants::ITEM_ID_LION) {
			if direct.borrow().is_edible() {
				self.inventory.remove_item_certain(direct.borrow().get_id());
				if direct.borrow().is(constants::ITEM_ID_KOHLRABI) {
					terminal::write_full(data.get_response("lionkill"));
					self.die(data);
				} else {
					terminal::write_full(data.get_response("lionwhet"));
				}
				return;
			}
		}

		if indirect.borrow().is(constants::ITEM_ID_TROLL) {
			if direct.borrow().is_edible() {
				self.inventory.remove_item_certain(direct.borrow().get_id());
				terminal::write_full(data.get_response("trolled"));
				self.die(data);
			} else {
				terminal::write_full(data.get_response("trolyawn"));
			}
		}

		// Default response: not interested
		let response = String::from(data.get_response("thestar")) + indirect.borrow().get_shortname() + data.get_response("nointerd") +
			direct.borrow().get_shortname() + data.get_response("dotend");
		terminal::write_full(&response);
	}

	// Have player travel to an adjacent location
	// TODO: I don't really like this very much, especially the 'back' part; there's probably a better way
	pub fn go(&mut self, data: &DataCollection, dir: &Direction) {

		self.previous_true = self.location.clone();
		let temp_loc = self.location.clone();

		let move_success = match *dir {
			Direction::Back => self.try_move_back(data),
			_ => self.try_move_other(data, dir),
		};

		if move_success && !self.has_light() {
			terminal::write_full(data.get_response("lampno"));
		}

		self.update_previous(move_success, &temp_loc);
		self.location.borrow_mut().set_visited(true);
	}

	// Attempt to move to previous location; return true if move was successful
	fn try_move_back(&mut self, data: &DataCollection) -> bool {
		match self.previous.clone() {
			None => {
				terminal::write_full(data.get_response("movnorem"));
				return false;
			},
			Some(prev) => {
				let prev_loc = prev.clone();
				return self.try_move_to(data, &prev_loc);
			},
		}
	}

	// Attempt to move to some location, which may not be reachable from the current location; return true if move was successful
	fn try_move_other(&mut self, data: &DataCollection, dir: &Direction) -> bool {
		let loc_clone = self.location.clone();
		let self_loc = loc_clone.borrow();

		match self_loc.get_direction(dir) {
			None => {
				terminal::write_full(data.get_response("movnoway"));
				return false;
			},
			Some(next) => {
				if !self.is_previous_loc(&next) {
					match (**self_loc).get_obstruction() {
						None => {},
						Some(obstruction) => {
							let mut response =  String::from(data.get_response("obststar"));
							if self.has_light() {
								response = response + data.get_response("themid") + obstruction.borrow().get_shortname() + data.get_response("dotend");
							} else {
								response = response + data.get_response("obstend");
							}
							terminal::write_full(&response);
							return false;
						}
					}
				}

				if !next.borrow().has_air() && !self.inventory.has_air() {
					terminal::write_full(data.get_response("movnoair"));
					return false;
				}

				return self.try_move_to(data, &next);
			},
		}
	}

	// Attempt to go to a location known to be adjacent; return true if move successful
	fn try_move_to(&mut self, data: &DataCollection, next: &LocationRef) -> bool {
		let mut rng = rand::thread_rng();
		let death_rand: u32 = rng.gen();
		let death = death_rand % 4 == 0;
		if !self.has_light() && !next.borrow().has_light() && death {
			terminal::write_full(data.get_response("nolight"));
			self.die(data);
			return false;
		} else {
			self.location = next.clone();
			terminal::write_full(&self.get_effective_appearance(data, self.location.borrow().mk_arrival_string()));
			return true;
		}
	}

	// Update player's 'previous' field as appropriate
	fn update_previous(&mut self, move_success: bool, temp_loc: &LocationRef) {
		if move_success {
			if self.location.borrow().can_reach(&temp_loc) {
				self.previous = Some(temp_loc.clone());
			} else {
				self.previous = None;
			}
		}

		if !self.is_alive() {
			self.previous = None;
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
		self.manipulate_item_present(data, item, Player::ignore_final);
	}

	pub fn ignore_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if item.borrow().is(constants::ITEM_ID_TROLL) {
			self.complete_obstruction_achievement(constants::ITEM_ID_TROLL, data.get_puzzle("troll"));
		} else {
			 terminal::write_full(data.get_response("ignogain"));
		}
	}

	pub fn insert(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::insert_portable);
	}

	fn insert_portable(&mut self, data: &DataCollection, item: &ItemRef) {
		// Objects cannot be inserted if they are immobile
		if !item.borrow().is_portable() {
			terminal::write_full(data.get_response("takenoca"));
			return;
		}

		// Objects cannot be inserted if they would be worn
		if item.borrow().is_wearable() {
			terminal::write_full(data.get_response("takenoca"));
			return;
		}

		// Find out what player wants to insert it into
		let container_str = terminal::read_question(&data.get_response_param("whatinse", item.borrow().get_shortname()));

		// Insert item into container, if container exists and is present
		match data.get_item_by_name(container_str[0].clone()) {
			None => terminal::write_full(data.get_response("nonowhat")),
			Some(container) => {
				let container_id = container.borrow().get_id();
				if self.inventory.contains_item(container_id) || self.location.borrow().contains_item(container_id) {
					self.insert_final(data, item, container)
				} else {
					terminal::write_full(&data.get_response_param("noseeh", container.borrow().get_shortname()));
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

		let within = container.borrow().get_within();
		match within {
			Some(it) => {
				// Make sure there is nothing already in the container
				if it.borrow().is(item.borrow().get_id()) {
					terminal::write_full(data.get_response("contitem"));
				} else {
					terminal::write_full(data.get_response("contfull"));
				}
			},
			None => {
				// Make sure item will fit in container
				if !container.borrow().can_fit(&item) {
				    terminal::write_full(data.get_response("nofit"));
				    return;
				}

				let item_id = item.borrow().get_id();
				let mut self_loc = self.location.borrow_mut();
				if self_loc.contains_item(item_id) {
					if !self.inventory.can_fit(&item) {
						terminal::write_full(data.get_response("takeover"));
						return;
					}
					self_loc.remove_item_certain(item_id);
				} else if self.inventory.contains_item(item_id) {
					self.inventory.remove_item_certain(item_id);
				}
				container.borrow_mut().set_within(Some(item.clone()));
				terminal::write_full(data.get_response("insegood"));
			}
		}
	}

	pub fn light(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::light_final);
	}

	fn light_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().is_switchable() {
			terminal::write_full(data.get_response("nonoligh"));
			return;
		}
		if item.borrow().is_on() {
			terminal::write_full(data.get_response("alreadon"));
			return;
		}
		terminal::write_full(data.get_response("lit"));
		item.borrow_mut().set_on(true);
	}

	// Return a description of what the player sees when they look
	pub fn get_look(&self, data: &DataCollection) -> String {
		self.get_effective_appearance(data, self.mk_location_string())
	}

	pub fn quench(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::quench_final);
	}

	fn quench_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if !item.borrow().is_switchable() {
			terminal::write_full(data.get_response("nonoquen"));
			return;
		}
		if !item.borrow().is_on() {
			terminal::write_full(data.get_response("alreadof"));
			return;
		}
		terminal::write_full(data.get_response("quenched"));
		item.borrow_mut().set_on(false);
	}

	pub fn get_score_str(&self, data: &DataCollection) -> String {
		let total_score = self.calculate_score(data);
		String::from(data.get_response("scorintr")) + &total_score.to_string() +
		data.get_response("scorpnts") + &data.get_max_score().to_string() +
		data.get_response("scordied") + &self.deaths.to_string() +
		data.get_response("scordths") + &self.instructions.to_string() +
		data.get_response("scorinss") + &self.hints.to_string() + data.get_response("scorhnts")
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
		self.observe_item(data, item, Player::play_final);
	}

	fn play_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if item.borrow().is(constants::ITEM_ID_WHISTLE) {
			let tune_words = terminal::read_question(data.get_response("whatplay"));
			let tune = &tune_words[0];
			terminal::write_full(&data.get_response_param("playwhis", tune));

			if tune == data.get_response("cabbage") {
				let lion_present = self.location.borrow().contains_item(constants::ITEM_ID_LION);
				if lion_present {
					let lion = data.get_item_by_id_certain(constants::ITEM_ID_LION);
					let lion_obstruction = lion.borrow().is_obstruction();
					if lion_obstruction {
						lion.borrow_mut().set_obstruction(false);
						self.complete_achievement(data.get_puzzle("liontune"));
					}
				}
			}
		} else {
			terminal::write_full(data.get_response("nonohow"));
		}
	}

	pub fn read(&mut self, data: &DataCollection, item: &ItemRef) {
		self.observe_item(data, item, Player::read_final);
	}

	fn read_final(&mut self, data: &DataCollection, item: &ItemRef) {
		terminal::write_full(&item.borrow().mk_writing_string(data.get_response("nowritin"), data.get_response("writstar"), data.get_response("writend")));
	}

	pub fn rub(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_present(data, item, Player::rub_final);
	}

	fn rub_final(&mut self, data: &DataCollection, item: &ItemRef) {
		if item.borrow().is(constants::ITEM_ID_LAMP) {
			terminal::write_full(data.get_response("genie"));
		} else if item.borrow().is(constants::ITEM_ID_DRAGON) {
			let tooth = data.get_item_by_id_certain(constants::ITEM_ID_TOOTH);
			self.location.borrow_mut().insert_item(tooth.clone(), true);
			self.complete_obstruction_achievement(constants::ITEM_ID_DRAGON, data.get_puzzle("dragon"));
		} else {
			terminal::write_full(data.get_response("nointere"));
		}
	}

	pub fn sleep(&mut self, data: &DataCollection) {
		self.teleport(data, data.get_tp_map_sleep(), false, "sleepno", "sleep");
	}

	pub fn tezazzle(&mut self, data: &DataCollection) {
		self.teleport(data, data.get_tp_map_witch(), true, "nohappen", "witch");
	}

	pub fn throw(&mut self, data: &DataCollection, item: &ItemRef) {
		self.manipulate_item_inventory(data, item, Player::throw_final);
	}

	fn throw_final(&mut self, data: &DataCollection, item: &ItemRef) {
		let it = self.inventory.remove_item_certain(item.borrow().get_id());
		terminal::write_full(data.get_response("throw"));
		let is_fragile = it.borrow().is_fragile();
		if is_fragile {
			terminal::write_full(data.get_response("shatthro"));
		} else {
			self.location.borrow_mut().insert_item(it, true);
			terminal::write_full(data.get_response("dropgood"));
		}
	}
}
