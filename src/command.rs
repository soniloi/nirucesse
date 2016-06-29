const CTRL_COMMAND_DEBUG: u32 = 0x01; // Whether the command is a debug command
const CTRL_COMMAND_INVENTORY: u32 = 0x02; // Whether the argument the command takes must be in the inventory
const CTRL_COMMAND_PRESENT: u32 = 0x04; // Whether the argument must be somewhere in the player's vicinity
const CTRL_COMMAND_ARGS: u32 = 0x08; // Whether the command must take an argument
const CTRL_COMMAND_SECRET: u32 = 0x10; // Whether the command is secret (not to be listed)
const CTRL_COMMAND_INVERTIBLE: u32 = 0x20; // Whether the command appears in order contrary to the usual e.g. "off" in "lamp off"
const CTRL_COMMAND_MOVEMENT: u32 = 0x40; // Whether the command intends movement
const CTRL_COMMAND_COMPOUND: u32 = 0x80; // Whether the command is compound e.g. "go" in "go north"

pub struct Command {
	name: String,
	status: u32,
	handler: fn(&str),
}

impl Command {

	pub fn new(name: String, status: u32, handler: fn(&str)) -> Command {
		Command {
			name: name,
			status: status,
			handler: handler,
		}
	}

	pub fn execute(&self, arg: &str) {
		let h = self.handler;
		println!("Received instruction to [{}] the [{}]", self.name, arg);
		h(arg);
	}

	pub fn write_out(&self) {
		println!("Command [name={}] [status={:X}]", self.name, self.status);
	}
}
