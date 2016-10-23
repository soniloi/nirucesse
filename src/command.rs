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

#[derive(PartialEq, Eq)]
pub enum ArgumentType {
	Any,
	Present,
	Inventory,
}

pub type ActionFn = fn(items: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType);

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

	fn needs_arg_inventory(&self) -> bool {
		self.has_property(CTRL_COMMAND_INVENTORY)
	}

	fn needs_arg_present(&self) -> bool {
		self.has_property(CTRL_COMMAND_PRESENT)
	}

	pub fn execute(&self, data: &DataCollection, arg: String, player: &mut Player) {
		let h = self.handler;
		let mut actual_arg = arg;

		// Command takes no argument, but player gave one anyway
		if !self.takes_arg() && !actual_arg.is_empty() {
			terminal::write_full(data.get_response(182));
			return;
		}

		// Command takes an argument, but player didn't give one
		if self.takes_arg() && actual_arg.is_empty() && !self.is_movement() {
			let further_args = terminal::read_question(&data.get_response_param(162, &self.name));
			actual_arg = String::new() + &further_args[0];
		}

		// Movement handling
		if self.is_movement() {
			actual_arg = String::new() + &self.name;
		}

		// Argument type
		let mut arg_type = ArgumentType::Any;
		if self.needs_arg_inventory() {
			arg_type = ArgumentType::Inventory;
		} else if self.needs_arg_present() {
			arg_type = ArgumentType::Present;
		}

		h(data, actual_arg, player, arg_type);
	}
}
