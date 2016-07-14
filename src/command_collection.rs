use std::collections::HashMap;
use std::rc::Rc;

use actions;
use command::Command;
use file_buffer::FileBuffer;
use item_collection::ItemCollection;
use player::Player;

const FILE_INDEX_COMMAND_TAG: usize = 0;
const FILE_INDEX_COMMAND_STATUS: usize = 1;
const FILE_INDEX_COMMAND_PRIMARY: usize = 2;
const FILE_INDEX_COMMAND_ALIAS_START: usize = 3;
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
		let mut acts: HashMap<&str, fn(items: &ItemCollection, arg: &str, player: &mut Player)> = HashMap::new();
		acts.insert("describe", actions::do_describe);
		acts.insert("down", actions::do_go);
		acts.insert("drop", actions::do_drop);
		acts.insert("east", actions::do_go);
		acts.insert("go", actions::do_go);
		acts.insert("inventory", actions::do_inventory);
		acts.insert("look", actions::do_look);
		acts.insert("north", actions::do_go);
		acts.insert("northeast", actions::do_go);
		acts.insert("northwest", actions::do_go);
		acts.insert("quit", actions::do_quit);
		acts.insert("south", actions::do_go);
		acts.insert("southeast", actions::do_go);
		acts.insert("southwest", actions::do_go);
		acts.insert("take", actions::do_take);
		acts.insert("up", actions::do_go);
		acts.insert("down", actions::do_go);

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
