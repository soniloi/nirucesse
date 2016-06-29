use std::collections::HashMap;

use command::Command;

pub struct CommandCollection<'a> {
	commands: HashMap<&'a str, *const Command>,
}

impl<'a> CommandCollection<'a> {

	pub fn new() -> CommandCollection<'a> {
		CommandCollection {
			commands: HashMap::new(),
		}
	}

	pub fn put(&mut self, key: &'a str, val: *const Command) {
		self.commands.insert(key, val);
	}

	pub fn get(&self, key: &str) -> Option<&*const Command> {
		self.commands.get(key)
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