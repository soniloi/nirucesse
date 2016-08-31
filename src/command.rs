const CTRL_COMMAND_DEBUG: u32 = 0x01; // Whether the command is a debug command
const CTRL_COMMAND_INVENTORY: u32 = 0x02; // Whether the argument the command takes must be in the inventory
const CTRL_COMMAND_PRESENT: u32 = 0x04; // Whether the argument must be somewhere in the player's vicinity
const CTRL_COMMAND_TAKES_ARG: u32 = 0x08; // Whether the command must take an argument
const CTRL_COMMAND_SECRET: u32 = 0x10; // Whether the command is secret (not to be listed)
const CTRL_COMMAND_INVERTIBLE: u32 = 0x20; // Whether the command appears in order contrary to the usual e.g. "off" in "lamp off"
const CTRL_COMMAND_MOVEMENT: u32 = 0x40; // Whether the command intends movement
const CTRL_COMMAND_COMPOUND: u32 = 0x80; // Whether the command is compound e.g. "go" in "go north"

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

	fn is_compound(&self) -> bool {
		self.has_property(CTRL_COMMAND_COMPOUND)
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

		// Argument counting
		if self.takes_arg() {
			// Command takes an argument, but player didn't give one
			if !self.is_movement() && actual_arg.is_empty() {
				let question = String::from(data.get_response("whatstar")) + &self.name + data.get_response("whatend");
				let further_args = terminal::read_question(&question);

				if further_args.len() != 1 || (!further_args.is_empty() && further_args[0].is_empty()) {
					terminal::write_full(data.get_response("notuigin"));
					return;
				} else {
					// FIXME: fix the lifetime on actual_arg/further_args
					let actual_arg = String::new() + &further_args[0];
					h(data, actual_arg, player);
					return;
				}
			}
		} else {
			// Command takes no argument, but player gave one anyway
			if !actual_arg.is_empty() {
				terminal::write_full(data.get_response("notuigin"));
				return;
			}
		}

		// Movement aliasing
		if self.is_movement() {
			if !self.is_compound() {
				actual_arg = String::new() + &self.name;
			}
		}

		h(data, actual_arg, player);
	}
}
