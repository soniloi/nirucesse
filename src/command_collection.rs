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
		acts.insert("3", actions::do_attack);
		acts.insert("4", actions::do_robot);
		acts.insert("5", actions::do_go);
		acts.insert("6", actions::do_burn);
		acts.insert("7", actions::do_call);
		acts.insert("8", actions::do_fairy);
		acts.insert("9", actions::do_climb);
		acts.insert("10", actions::do_commands);
		acts.insert("11", actions::do_cook);
		acts.insert("12", actions::do_describe);
		acts.insert("13", actions::do_go);
		acts.insert("14", actions::do_drink);
		acts.insert("15", actions::do_drop);
		acts.insert("16", actions::do_go);
		acts.insert("17", actions::do_eat);
		acts.insert("18", actions::do_empty);
		acts.insert("19", actions::do_exchange);
		acts.insert("20", actions::do_explain);
		acts.insert("21", actions::do_feed);
		acts.insert("22", actions::do_fish);
		acts.insert("23", actions::do_fly);
		acts.insert("24", actions::do_give);
		acts.insert("25", actions::do_go_disambiguate);
		acts.insert("26", actions::do_help);
		acts.insert("27", actions::do_hint);
		acts.insert("28", actions::do_ignore);
		acts.insert("29", actions::do_insert);
		acts.insert("30", actions::do_inventory);
		acts.insert("31", actions::do_knit);
		acts.insert("32", actions::do_light);
		acts.insert("33", actions::do_look);
		acts.insert("34", actions::do_go);
		acts.insert("35", actions::do_go);
		acts.insert("36", actions::do_go);
		acts.insert("37", actions::do_go);
		acts.insert("38", actions::do_play);
		acts.insert("39", actions::do_plugh);
		acts.insert("40", actions::do_pour);
		acts.insert("41", actions::do_push);
		acts.insert("42", actions::do_quench);
		acts.insert("43", actions::do_quit);
		acts.insert("44", actions::do_read);
		acts.insert("45", actions::do_repair);
		acts.insert("46", actions::do_rob);
		acts.insert("47", actions::do_roll);
		acts.insert("48", actions::do_rub);
		acts.insert("49", actions::do_say);
		acts.insert("50", actions::do_score);
		acts.insert("51", actions::do_sleep);
		acts.insert("52", actions::do_go);
		acts.insert("53", actions::do_go);
		acts.insert("54", actions::do_go);
		acts.insert("55", actions::do_stare);
		acts.insert("56", actions::do_take);
		acts.insert("57", actions::do_tether);
		acts.insert("58", actions::do_tezazzle);
		acts.insert("59", actions::do_throw);
		acts.insert("60", actions::do_go);
		acts.insert("61", actions::do_water);
		acts.insert("62", actions::do_go);
		acts.insert("63", actions::do_wizard);
		acts.insert("64", actions::do_xyzzy);
		acts.insert("65", actions::do_acorn);
	}

	#[cfg(debug_assertions)]
	fn add_actions_additional(acts: &mut HashMap<&str, ActionFn>) {
		acts.insert("0", actions::do_flash);
		acts.insert("1", actions::do_grab);
		acts.insert("2", actions::do_node);
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
					None => panic!("Unknown movement command {}, fail.", tag),
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
