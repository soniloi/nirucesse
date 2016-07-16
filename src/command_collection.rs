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

pub struct CommandCollection {
	commands: HashMap<String, Rc<Box<Command>>>,
}

impl<'a> CommandCollection {

	pub fn new() -> CommandCollection {
		CommandCollection {
			commands: HashMap::new(),
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer) {
		// TODO: make static
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
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					let primary = String::from(words[FILE_INDEX_COMMAND_PRIMARY]);
					let key = primary.clone();
					let status_str = words[FILE_INDEX_COMMAND_STATUS];
					let status = match u32::from_str_radix(status_str, 16) {
						Err(why) => panic!("Unable to parse integer field {}: {}", status_str, why),
						Ok(status) => status,
					};

					let tag = words[FILE_INDEX_COMMAND_TAG];
					match acts.get(tag) {
						None => println!("\x1b[31m[Warning: no action function found for tag [{}]; skipping]\x1b[0m", tag),
						Some(act) => {
							let cmd: Rc<Box<Command>> = Rc::new(Box::new(Command::new(primary, status, *act)));
							self.commands.insert(key, cmd.clone());
							for i in FILE_INDEX_COMMAND_ALIAS_START..words.len() {
								self.commands.insert(String::from(words[i]), cmd.clone());
							}
						},
					}
				},
			}
			line = buffer.get_line();
		}
	}

	pub fn put(&mut self, key: String, val: Rc<Box<Command>>) {
		self.commands.insert(key, val);
	}

	pub fn get(&self, key: String) -> Option<&Rc<Box<Command>>> {
		self.commands.get(&key)
	}

}
