const CTRL_COMMAND_DEBUG: u32 = 0x01; // Whether the command is a debug command
const CTRL_COMMAND_INVENTORY: u32 = 0x02; // Whether the argument the command takes must be in the inventory
const CTRL_COMMAND_PRESENT: u32 = 0x04; // Whether the argument must be somewhere in the player's vicinity
const CTRL_COMMAND_TAKES_ARG: u32 = 0x08; // Whether the command must take an argument
const CTRL_COMMAND_SECRET: u32 = 0x10; // Whether the command is secret (not to be listed)
const CTRL_COMMAND_INVERTIBLE: u32 = 0x20; // Whether the command appears in order contrary to the usual e.g. "off" in "lamp off"
const CTRL_COMMAND_MOVEMENT: u32 = 0x40; // Whether the command intends movement

use data_collection::DataCollection;
use player::Player;
use terminal;

pub type ActionFn = fn(items: &DataCollection, arg: String, player: &mut Player);

pub struct Command {
	name: String,
	properties: u32,
	handler: ActionFn,
}

impl Command {

	pub fn new(name: String, properties: u32, handler: ActionFn) -> Command {
		Command {
			name: name,
			properties: properties,
			handler: handler,
		}
	}

	fn has_property(&self, property: u32) -> bool {
		self.properties & property != 0
	}

	pub fn is_invertible(&self) -> bool {
		self.has_property(CTRL_COMMAND_INVERTIBLE)
	}

	fn is_movement(&self) -> bool {
		self.has_property(CTRL_COMMAND_MOVEMENT)
	}

	fn takes_arg(&self) -> bool{
		self.has_property(CTRL_COMMAND_TAKES_ARG)
	}

	pub fn is_secret(&self) -> bool {
		self.has_property(CTRL_COMMAND_SECRET)
	}

	pub fn execute(&self, data: &DataCollection, arg: String, player: &mut Player) {
		let h = self.handler;
		let mut actual_arg = arg;

		// Command takes no argument, but player gave one anyway
		if !self.takes_arg() && !actual_arg.is_empty() {
			terminal::write_full(data.get_response(103));
			return;
		}

		// Command takes an argument, but player didn't give one
		if self.takes_arg() && actual_arg.is_empty() && !self.is_movement() {
			let question = String::from(data.get_response(162)) + &self.name + data.get_response(158);
			let further_args = terminal::read_question(&question);
			actual_arg = String::new() + &further_args[0];
		}

		// Movement handling
		if self.is_movement() {
			actual_arg = String::new() + &self.name;
		}

		h(data, actual_arg, player);
	}
}
