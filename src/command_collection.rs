use std::collections::HashMap;
use std::rc::Rc;

use actions;
use command::ActionFn;
use command::Command;
use data_collection;
use data_collection::CommandRef;
use file_buffer::FileBuffer;

const FILE_INDEX_COMMAND_TAG: usize = 0;
const FILE_INDEX_COMMAND_STATUS: usize = 1;
const FILE_INDEX_COMMAND_PRIMARY: usize = 2;
const FILE_INDEX_COMMAND_ALIAS_START: usize = 3;
const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct CommandCollection {
	commands: HashMap<String, CommandRef>,
}

impl CommandCollection {

	pub fn new() -> CommandCollection {
		CommandCollection {
			commands: HashMap::new(),
		}
	}

	// TODO: make properly static
	fn init_actions() -> HashMap<&'static str, ActionFn> {
		let mut acts: HashMap<&str, ActionFn> = HashMap::new();
		acts.insert("avnarand", actions::do_avnarand);
		acts.insert("back", actions::do_go);
		acts.insert("burn", actions::do_burn);
		acts.insert("commands", actions::do_commands);
		acts.insert("describe", actions::do_describe);
		acts.insert("down", actions::do_go);
		acts.insert("drop", actions::do_drop);
		acts.insert("east", actions::do_go);
		acts.insert("explain", actions::do_explain);
		acts.insert("go", actions::do_go_disambiguate);
		acts.insert("help", actions::do_help);
		acts.insert("hint", actions::do_hint);
		acts.insert("inventory", actions::do_inventory);
		acts.insert("light", actions::do_light);
		acts.insert("look", actions::do_look);
		acts.insert("north", actions::do_go);
		acts.insert("northeast", actions::do_go);
		acts.insert("northwest", actions::do_go);
		acts.insert("play", actions::do_play);
		acts.insert("quench", actions::do_quench);
		acts.insert("quit", actions::do_quit);
		acts.insert("read", actions::do_read);
		acts.insert("rub", actions::do_rub);
		acts.insert("score", actions::do_score);
		acts.insert("south", actions::do_go);
		acts.insert("southeast", actions::do_go);
		acts.insert("southwest", actions::do_go);
		acts.insert("take", actions::do_take);
		acts.insert("throw", actions::do_throw);
		acts.insert("up", actions::do_go);
		acts.insert("west", actions::do_go);
		acts.insert("xyzzy", actions::do_xyzzy);
		acts
	}

	pub fn init(&mut self, buffer: &mut FileBuffer) {

		let acts = CommandCollection::init_actions();

		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => return,
				x => {
					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();
					self.parse_and_insert_command(&words, &acts);
				},
			}
			line = buffer.get_line();
		}
	}

	fn parse_and_insert_command(&mut self, words: &Vec<&str>, acts: &HashMap<&str, ActionFn>) {
		let primary = String::from(words[FILE_INDEX_COMMAND_PRIMARY]);
		let key = primary.clone();
		let properties = data_collection::str_to_u32(words[FILE_INDEX_COMMAND_STATUS], 16);
		let tag = words[FILE_INDEX_COMMAND_TAG];

		match acts.get(tag) {
			None => println!("\x1b[31m[Warning: no action function found for tag [{}]; skipping]\x1b[0m", tag),
			Some(act) => {
				let cmd: CommandRef = Rc::new(Box::new(Command::new(primary, properties, *act)));
				self.commands.insert(key, cmd.clone());
				for i in FILE_INDEX_COMMAND_ALIAS_START..words.len() {
					if !words[i].is_empty() {
						self.commands.insert(String::from(words[i]), cmd.clone());
					}
				}
			},
		}
	}

	pub fn get(&self, key: String) -> Option<&CommandRef> {
		self.commands.get(&key)
	}

	pub fn mk_non_secret_string(&self) -> String {
		let mut result: String = String::from("I understand the following commands (and possibly others):\n");
		for (tag, command) in self.commands.iter() {
			if !command.is_secret() {
				result = result + "[" + tag + "] ";
			}
		}
		result
	}
}
