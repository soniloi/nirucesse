use std::collections::HashMap;
use std::rc::Rc;

use command::Command;
use file_buffer::FileBuffer;

const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct CommandCollection<'a> {
	commands: HashMap<&'a str, Rc<Box<Command<'a>>>>,
}

impl<'a> CommandCollection<'a> {

	pub fn new() -> CommandCollection<'a> {
		CommandCollection {
			commands: HashMap::new(),
		}
	}

	pub fn init(&self, buffer: &mut FileBuffer) {
		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => return,
				_ => {
					println!("{}", line);
					line = buffer.get_line();
				}
			}

		}
	}

	pub fn put(&mut self, key: &'a str, val: Rc<Box<Command<'a>>>) {
		self.commands.insert(key, val);
	}

	pub fn get(&self, key: &str) -> Option<&Rc<Box<Command>>> {
		self.commands.get(key)
	}

}
