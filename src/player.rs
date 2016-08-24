use std::cell::RefCell;
use rand;
use rand::Rng;
use std::rc::Rc;

use data_collection::DataCollection;
use inventory::Inventory;
use item::Item;
use location::Direction;
use location::Location;
use terminal;

pub type ItemManipFinalFn = fn(player: &mut Player, data: &DataCollection, item: &Rc<Box<Item>>);
pub type ItemManipFn = ItemManipFinalFn;

pub struct Player {
	inventory: Inventory,
	location: Rc<RefCell<Box<Location>>>,
	previous_true: Rc<RefCell<Box<Location>>>,
	previous: Option<Rc<RefCell<Box<Location>>>>,
	score: u32, // player's current score
	playing: bool, // whether player is currently playing
	hints: u32, // number of hints player has requested
	instructions: u32, // number of instructions player has entered
	deaths: u32, // number of times player has died
	alive: bool,
}

impl Player {

	pub fn new(initial: Rc<RefCell<Box<Location>>>) -> Player {
		Player {
			inventory: Inventory::new(16),
			location: initial.clone(),
			previous_true: initial.clone(),
			previous: None,
			score: 0u32,
			playing: true,
			hints: 0u32,
			instructions: 0u32,
			deaths: 0u32,
			alive: true,
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

	pub fn contains_item(&self, item_ptr: &Rc<Box<Item>>) -> bool {
		self.inventory.contains_item(item_ptr)
	}

	pub fn insert_item(&mut self, item_ptr: Rc<Box<Item>>) {
		self.inventory.insert_item(item_ptr);
	}

	pub fn get_location(&self) -> &Rc<RefCell<Box<Location>>> {
		&self.location
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
		self.alive = b
	}

	pub fn die(&mut self, data: &DataCollection) {
		self.set_alive(false);
		self.increment_deaths();
		self.location = data.get_location_wake().clone();
	}

	pub fn drop_on_death(&mut self, safe_loc: &Rc<RefCell<Box<Location>>>) {
		self.inventory.drop_on_death(safe_loc, &self.previous_true);
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

	fn observe_item(&mut self, data: &DataCollection, item: &Rc<Box<Item>>, act: ItemManipFinalFn) {
		if !self.has_light() {
			terminal::write_full(data.get_response("cantseed"));
			return;
		}
		self.manipulate_item_present(data, item, act);
	}

	// Manipulate an item present either in the player's inventory or at the player's location
	fn manipulate_item_present(&mut self, data: &DataCollection, item: &Rc<Box<Item>>, act: ItemManipFinalFn) {
		if !self.inventory.contains_item(item) && !self.location.borrow().contains_item(item) {
			let response = String::from(data.get_response("nosee")) + &item.get_shortname() + data.get_response("noseeher");
			terminal::write_full(&response);
			return;
		}
		act(self, data, item);
	}

	// Manipulate an item present strictly in the player's inventory
	fn manipulate_item_inventory(&mut self, data: &DataCollection, item: &Rc<Box<Item>>, act: ItemManipFinalFn) {
		if !self.inventory.contains_item(item) {
			let response = String::from(data.get_response("nocarry")) + &item.get_shortname() + ".";
			terminal::write_full(&response);
			return;
		}
		act(self, data, item);
	}

	pub fn avnarand(&mut self, data: &DataCollection) {
		let mut self_loc = self.location.borrow_mut();
		let mut robot_here = false;
		match self_loc.get_obstruction() {
			None => {},
			Some(obstruction) => {
				if obstruction.is(::ITEM_ID_ROBOT) {
					self_loc.remove_item_certain(&obstruction);
					robot_here = true;
				}
			},
		}
		if robot_here {
			terminal::write_full(data.get_response("robot"));
		} else {
			terminal::write_full(data.get_response("nohappen"));
		}
	}

	pub fn burn(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		self.manipulate_item_present(data, item, Player::burn_final);
	}

	fn burn_final(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		if !self.inventory.contains_item_by_id(::ITEM_ID_MATCHES) {
			terminal::write_full(data.get_response("nomatch"));
			return;
		}
		match item.get_id() {
			::ITEM_ID_BREAD => {
				match self.inventory.remove_item(item) {
					None => self.location.borrow_mut().remove_item_certain(item),
					Some(_) => {},
				};
				let toast = data.get_item_certain(String::from("toast"));
				self.location.borrow_mut().insert_item(toast.clone());
				terminal::write_full(data.get_response("bread"));
			},
			::ITEM_ID_TOAST => {
				match self.inventory.remove_item(item) {
					None => self.location.borrow_mut().remove_item_certain(item),
					Some(_) => {},
				};
				terminal::write_full(data.get_response("toast"));
				let mut self_loc = self.location.borrow_mut();
				if self_loc.is(::LOCATION_ID_AIRLOCKE) {
					let out_loc = data.get_location_certain(::LOCATION_ID_AIRLOCKEOUT);
					self_loc.set_direction(Direction::Southwest, out_loc.clone());
					self_loc.set_air(false);
					terminal::write_full(data.get_response("toastalm"));
				} else {
					terminal::write_full(data.get_response("ashmouse"));
				}
			}
			_ => {
				terminal::write_full(data.get_response("nonohow"));
			}
		}
	}

	// Have player attempt to pick up item from current location
	pub fn take(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		self.manipulate_item_present(data, item, Player::take_final);
	}

	fn take_final(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		if self.contains_item(item) {
			terminal::write_full(data.get_response("takealre"));
			return;
		}

		if !item.is_mobile() {
			terminal::write_full(data.get_response("takenoca"));
			return;
		}

		if !self.inventory.can_accept(&item) {
			terminal::write_full(data.get_response("takeover"));
			return;
		}

		self.location.borrow_mut().remove_item_certain(item);
		self.insert_item(item.clone());

		if item.is_wearable() {
			terminal::write_full(data.get_response("wear"));
		} else {
			terminal::write_full(data.get_response("takegood"));
		}
	}

	// Have player attempt to drop item from inventory to current location
	pub fn drop(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		self.manipulate_item_inventory(data, item, Player::drop_final);
	}

	fn drop_final(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		let it = self.inventory.remove_item_certain(item);
		self.location.borrow_mut().insert_item(it);
		terminal::write_full(data.get_response("dropgood"));
	}

	// Describe an item in the player's inventory or at the player's location
	pub fn describe(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		self.observe_item(data, item, Player::describe_final);
	}

	fn describe_final(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		terminal::write_full(&item.mk_full_string(data.get_response("descstar"), data.get_response("descend")));
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
							let mut response =  String::from("You cannot get past ");
							if self.has_light() {
								response = response + "the " + obstruction.get_shortname() + ".";
							} else {
								response = response + "some obstruction at this location.";
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
	fn try_move_to(&mut self, data: &DataCollection, next: &Rc<RefCell<Box<Location>>>) -> bool {
		let mut rng = rand::thread_rng();
		let death_rand: u32 = rng.gen();
		let death = death_rand % 4 == 0;
		if !self.has_light() && !next.borrow().has_light() && death {
			terminal::write_full(data.get_response("nolight"));
			self.die(data);
			return false;
		} else {
			self.location = next.clone();
			terminal::write_full(&self.get_effective_appearance(data, self.location.borrow().mk_full_string()));
			return true;
		}
	}

	// Update player's 'previous' field as appropriate
	fn update_previous(&mut self, move_success: bool, temp_loc: &Rc<RefCell<Box<Location>>>) {
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
	fn is_previous_loc(&self, next: &Rc<RefCell<Box<Location>>>) -> bool {
		let previous = self.previous.clone();
		match previous {
			None => return false,
			Some(prev) => prev.borrow().get_id() == next.borrow().get_id(),
		}
	}

	// Return a description of what the player sees when they look
	pub fn get_look(&self, data: &DataCollection) -> String {
		self.get_effective_appearance(data, self.mk_location_string())
	}

	pub fn get_score_str(&self) -> String {
		String::from("You currently have a score of ") + &self.score.to_string() +
		" point(s). You have died " + &self.deaths.to_string() +
		" time(s). You have entered " + &self.instructions.to_string() +
		" instruction(s), and requested " + &self.hints.to_string() + " hint(s)."
	}

	pub fn increment_hints(&mut self) {
		self.hints = self.hints + 1;
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

	pub fn read(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		self.observe_item(data, item, Player::read_final);
	}

	fn read_final(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		terminal::write_full(&item.mk_writing_string(data.get_response("nowritin"), data.get_response("writstar"), data.get_response("writend")));
	}

	pub fn throw(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		self.manipulate_item_inventory(data, item, Player::throw_final);
	}

	fn throw_final(&mut self, data: &DataCollection, item: &Rc<Box<Item>>) {
		let it = self.inventory.remove_item_certain(item);
		if it.is_fragile() {
			terminal::write_full(data.get_response("shatthro"));
		} else {
			self.location.borrow_mut().insert_item(it);
			terminal::write_full(data.get_response("dropgood"));
		}
	}
}
