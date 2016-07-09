use std::collections::HashMap;
use std::rc::Rc;

use command::Command;

pub struct CommandCollection<'a> {
	commands: HashMap<&'a str, Rc<Box<Command<'a>>>>,
}

impl<'a> CommandCollection<'a> {

	pub fn new() -> CommandCollection<'a> {
		CommandCollection {
			commands: HashMap::new(),
		}
	}

	pub fn put(&mut self, key: &'a str, val: Rc<Box<Command<'a>>>) {
		self.commands.insert(key, val);
	}

	pub fn get(&self, key: &str) -> Option<&Rc<Box<Command>>> {
		self.commands.get(key)
	}

}
