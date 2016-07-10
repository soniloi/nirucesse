const CTRL_COMMAND_DEBUG: u32 = 0x01; // Whether the command is a debug command
const CTRL_COMMAND_INVENTORY: u32 = 0x02; // Whether the argument the command takes must be in the inventory
const CTRL_COMMAND_PRESENT: u32 = 0x04; // Whether the argument must be somewhere in the player's vicinity
const CTRL_COMMAND_ARGS: u32 = 0x08; // Whether the command must take an argument
const CTRL_COMMAND_SECRET: u32 = 0x10; // Whether the command is secret (not to be listed)
const CTRL_COMMAND_INVERTIBLE: u32 = 0x20; // Whether the command appears in order contrary to the usual e.g. "off" in "lamp off"
const CTRL_COMMAND_MOVEMENT: u32 = 0x40; // Whether the command intends movement
const CTRL_COMMAND_COMPOUND: u32 = 0x80; // Whether the command is compound e.g. "go" in "go north"

use item_collection::ItemCollection;
use player::Player;

pub struct Command<'a> {
	name: &'a str,
	properties: u32,
	handler: fn(items: &ItemCollection, arg: &str, player: &mut Player),
}

impl<'a> Command<'a> {

	pub fn new(name: &str, properties: u32, handler: fn(items: &ItemCollection, arg: &str, player: &mut Player)) -> Command {
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

	pub fn execute(&self, items: &ItemCollection, arg: &str, player: &mut Player) {
		let mut actual_arg = arg;
		if self.is_movement() {
			if !self.is_compound() {
				actual_arg = self.name;
			}
		}
		let h = self.handler;
		h(items, actual_arg, player);
	}
}
