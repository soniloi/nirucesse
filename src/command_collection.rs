use std::collections::HashMap;
use std::rc::Rc;

use actions;
use command::ActionFn;
use command::Command;
use constants;
use data_collection;
use data_collection::CommandRef;
use file_buffer::FileBuffer;
use location::Direction;

const FILE_INDEX_COMMAND_TAG: usize = 0;
const FILE_INDEX_COMMAND_STATUS: usize = 1;
const FILE_INDEX_COMMAND_PRIMARY: usize = 2;
const FILE_INDEX_COMMAND_ALIAS_START: usize = 3;

pub struct CommandCollection {
	commands: HashMap<String, CommandRef>,
	direction_map: HashMap<String, Direction>, // Map of direction strings to direction enum
}

impl CommandCollection {

	pub fn new() -> CommandCollection {
		CommandCollection {
			commands: HashMap::new(),
			direction_map: HashMap::new(),
		}
	}

	fn add_actions_common(acts: &mut HashMap<&str, ActionFn>) {
		acts.insert("attack", actions::do_attack);
		acts.insert("avnarand", actions::do_robot);
		acts.insert("back", actions::do_go);
		acts.insert("burn", actions::do_burn);
		acts.insert("call", actions::do_call);
		acts.insert("chimbu", actions::do_fairy);
		acts.insert("climb", actions::do_climb);
		acts.insert("commands", actions::do_commands);
		acts.insert("cook", actions::do_cook);
		acts.insert("describe", actions::do_describe);
		acts.insert("down", actions::do_go);
		acts.insert("drink", actions::do_drink);
		acts.insert("drop", actions::do_drop);
		acts.insert("east", actions::do_go);
		acts.insert("eat", actions::do_eat);
		acts.insert("empty", actions::do_empty);
		acts.insert("exchange", actions::do_exchange);
		acts.insert("explain", actions::do_explain);
		acts.insert("feed", actions::do_feed);
		acts.insert("fish", actions::do_fish);
		acts.insert("fly", actions::do_fly);
		acts.insert("give", actions::do_give);
		acts.insert("go", actions::do_go_disambiguate);
		acts.insert("help", actions::do_help);
		acts.insert("hint", actions::do_hint);
		acts.insert("ignore", actions::do_ignore);
		acts.insert("insert", actions::do_insert);
		acts.insert("inventory", actions::do_inventory);
		acts.insert("knit", actions::do_knit);
		acts.insert("light", actions::do_light);
		acts.insert("look", actions::do_look);
		acts.insert("north", actions::do_go);
		acts.insert("northeast", actions::do_go);
		acts.insert("northwest", actions::do_go);
		acts.insert("out", actions::do_go);
		acts.insert("play", actions::do_play);
		acts.insert("plugh", actions::do_plugh);
		acts.insert("pour", actions::do_pour);
		acts.insert("push", actions::do_push);
		acts.insert("quench", actions::do_quench);
		acts.insert("quit", actions::do_quit);
		acts.insert("read", actions::do_read);
		acts.insert("repair", actions::do_repair);
		acts.insert("rob", actions::do_rob);
		acts.insert("roll", actions::do_roll);
		acts.insert("rub", actions::do_rub);
		acts.insert("say", actions::do_say);
		acts.insert("score", actions::do_score);
		acts.insert("sleep", actions::do_sleep);
		acts.insert("south", actions::do_go);
		acts.insert("southeast", actions::do_go);
		acts.insert("southwest", actions::do_go);
		acts.insert("stare", actions::do_stare);
		acts.insert("tezazzle", actions::do_tezazzle);
		acts.insert("take", actions::do_take);
		acts.insert("tether", actions::do_tether);
		acts.insert("throw", actions::do_throw);
		acts.insert("up", actions::do_go);
		acts.insert("water", actions::do_water);
		acts.insert("west", actions::do_go);
		acts.insert("xyro", actions::do_wizard);
		acts.insert("xyzzy", actions::do_xyzzy);
		acts.insert("ziqua", actions::do_acorn);
	}

	#[cfg(debug_assertions)]
	fn add_actions_additional(acts: &mut HashMap<&str, ActionFn>) {
		acts.insert("flash", actions::do_flash);
		acts.insert("grab", actions::do_grab);
		acts.insert("node", actions::do_node);
	}

	#[cfg(not(debug_assertions))]
	#[allow(unused_variables)]
	fn add_actions_additional(acts: &mut HashMap<&str, ActionFn>) {
	}

	// TODO: make properly static
	fn init_actions() -> HashMap<&'static str, ActionFn> {
		let mut acts: HashMap<&str, ActionFn> = HashMap::new();
		CommandCollection::add_actions_common(&mut acts);
		CommandCollection::add_actions_additional(&mut acts);
		acts
	}

	// TODO: make static
	// Create map of command tags (note: not primary names) to Directions
	fn get_tag_dir_map() -> HashMap<&'static str, Direction> {
		let mut tag_dirs = HashMap::new();
		tag_dirs.insert(constants::STR_NORTH, Direction::North);
		tag_dirs.insert(constants::STR_SOUTH, Direction::South);
		tag_dirs.insert(constants::STR_EAST, Direction::East);
		tag_dirs.insert(constants::STR_WEST, Direction::West);
		tag_dirs.insert(constants::STR_NORTHEAST, Direction::Northeast);
		tag_dirs.insert(constants::STR_SOUTHWEST, Direction::Southwest);
		tag_dirs.insert(constants::STR_SOUTHEAST, Direction::Southeast);
		tag_dirs.insert(constants::STR_NORTHWEST, Direction::Northwest);
		tag_dirs.insert(constants::STR_UP, Direction::Up);
		tag_dirs.insert(constants::STR_DOWN, Direction::Down);
		tag_dirs.insert(constants::STR_OUT, Direction::Out);
		tag_dirs.insert(constants::STR_BACK, Direction::Back);
		tag_dirs
	}

	pub fn init(&mut self, buffer: &mut FileBuffer) {
		let tag_dirs = CommandCollection::get_tag_dir_map();

		let acts = CommandCollection::init_actions();
		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				constants::FILE_SECTION_SEPARATOR => return,
				x => {
					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();
					self.parse_and_insert_command(&words, &acts, &tag_dirs);
				},
			}
			line = buffer.get_line();
		}
	}

	fn parse_and_insert_command(&mut self, words: &Vec<&str>, acts: &HashMap<&str, ActionFn>, tag_dirs: &HashMap<&'static str, Direction>) {
		let primary = String::from(words[FILE_INDEX_COMMAND_PRIMARY]);
		let properties = data_collection::str_to_u32_certain(words[FILE_INDEX_COMMAND_STATUS], 16);
		let tag = words[FILE_INDEX_COMMAND_TAG];

		if let Some(act) = acts.get(tag) {
			let cmd: CommandRef = Rc::new(Box::new(Command::new(primary.clone(), properties, *act)));
			self.commands.insert(primary.clone(), cmd.clone());
			for i in FILE_INDEX_COMMAND_ALIAS_START..words.len() {
				if !words[i].is_empty() {
					self.commands.insert(String::from(words[i]), cmd.clone());
				}
			}

			// Map localized primary names (as opposed to tags) to Directions
			if cmd.has_property(constants::CTRL_COMMAND_MOVEMENT) {
				match tag_dirs.get(&tag) {
					None => panic!("Unknown movement command {}, fail.", primary),
					Some(dir) => self.direction_map.insert(primary, *dir),
				};
			}
		}
	}

	pub fn get(&self, key: String) -> Option<&CommandRef> {
		self.commands.get(&key)
	}

	pub fn mk_non_secret_string(&self, intro: &str) -> String {
		let mut comms = Vec::new();
		for (tag, command) in self.commands.iter() {
			if !command.has_property(constants::CTRL_COMMAND_SECRET) {
				comms.push(String::from("[") + tag + "] ");
			}
		}
		comms.sort();
		String::from(intro) + "\n" + &comms.concat()
	}

	// Get a Direction from a string
	pub fn get_direction_enum(&self, dir_str: &str) -> &Direction {
		match self.direction_map.get(dir_str) {
		    None => panic!("Command collection corruption, fail."),
			Some(dir) => dir,
		}
	}
}
