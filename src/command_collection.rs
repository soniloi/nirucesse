use std::collections::HashMap;
use std::rc::Rc;

use command::Command;

pub struct CommandCollection<'a> {
	commands: HashMap<&'a str, Rc<Box<Command>>>,
}

impl<'a> CommandCollection<'a> {

	pub fn new() -> CommandCollection<'a> {
		CommandCollection {
			commands: HashMap::new(),
		}
	}

	pub fn put(&mut self, key: &'a str, val: Rc<Box<Command>>) {
		self.commands.insert(key, val);
	}

	pub fn get(&self, key: &str) -> Option<&Rc<Box<Command>>> {
		self.commands.get(key)
	}

	pub fn write_out(&self) {
		for (key, val) in self.commands.iter() {
			print!("\t[{}]\t", key);
			(**val).write_out();
		}
	}
}
