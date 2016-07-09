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
	status: u32,
	handler: fn(items: &ItemCollection, arg: &str, player: &mut Player),
}

impl<'a> Command<'a> {

	pub fn new(name: &str, status: u32, handler: fn(items: &ItemCollection, arg: &str, player: &mut Player)) -> Command {
		Command {
			name: name,
			status: status,
			handler: handler,
		}
	}

	pub fn execute(&self, items: &ItemCollection, arg: &str, player: &mut Player) {
		let h = self.handler;
		//println!("Received instruction for player to [{}] the [{}]", self.name, arg);
		h(items, arg, player);
	}
}
