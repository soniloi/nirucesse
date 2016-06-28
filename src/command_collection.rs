use std::collections::HashMap;

use command::Command;

pub struct CommandCollection {
	commands: HashMap<String, *const Command>,
}

impl CommandCollection {

	pub fn new() -> CommandCollection {
		CommandCollection {
			commands: HashMap::new(),
		}
	}

	pub fn put(&mut self, key: String, val: *const Command) {
		self.commands.insert(key, val);
	}

	pub fn get(&self, key: String) -> Option<&*const Command> {
		self.commands.get(&key)
	}

	pub fn write_all(&self) {
		for (key, val) in self.commands.iter() {
			unsafe {
				print!("\t[{}]\t", key);
				(**val).write_out();
			}
		}
	}
}