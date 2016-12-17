use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use actions;
use command::{ActionFn, Command};
use constants;
use data_collection::{self, CommandId, CommandRef};
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

	fn add_actions_common(acts: &mut HashMap<CommandId, ActionFn>) {
		acts.insert(constants::COMMAND_ID_ATTACK, actions::do_attack);
		acts.insert(constants::COMMAND_ID_ROBOT, actions::do_robot);
		acts.insert(constants::COMMAND_ID_BACK, actions::do_go);
		acts.insert(constants::COMMAND_ID_BURN, actions::do_burn);
		acts.insert(constants::COMMAND_ID_CALL, actions::do_call);
		acts.insert(constants::COMMAND_ID_FAIRY, actions::do_fairy);
		acts.insert(constants::COMMAND_ID_CLIMB, actions::do_climb);
		acts.insert(constants::COMMAND_ID_COMMANDS, actions::do_commands);
		acts.insert(constants::COMMAND_ID_COOK, actions::do_cook);
		acts.insert(constants::COMMAND_ID_DESCRIBE, actions::do_describe);
		acts.insert(constants::COMMAND_ID_DOWN, actions::do_go);
		acts.insert(constants::COMMAND_ID_DRINK, actions::do_drink);
		acts.insert(constants::COMMAND_ID_DROP, actions::do_drop);
		acts.insert(constants::COMMAND_ID_EAST, actions::do_go);
		acts.insert(constants::COMMAND_ID_EAT, actions::do_eat);
		acts.insert(constants::COMMAND_ID_EMPTY, actions::do_empty);
		acts.insert(constants::COMMAND_ID_EXCHANGE, actions::do_exchange);
		acts.insert(constants::COMMAND_ID_EXPLAIN, actions::do_explain);
		acts.insert(constants::COMMAND_ID_FEED, actions::do_feed);
		acts.insert(constants::COMMAND_ID_FISH, actions::do_fish);
		acts.insert(constants::COMMAND_ID_FLY, actions::do_fly);
		acts.insert(constants::COMMAND_ID_GIVE, actions::do_give);
		acts.insert(constants::COMMAND_ID_GO, actions::do_go_disambiguate);
		acts.insert(constants::COMMAND_ID_HELP, actions::do_help);
		acts.insert(constants::COMMAND_ID_HINT, actions::do_hint);
		acts.insert(constants::COMMAND_ID_IGNORE, actions::do_ignore);
		acts.insert(constants::COMMAND_ID_INSERT, actions::do_insert);
		acts.insert(constants::COMMAND_ID_INVENTORY, actions::do_inventory);
		acts.insert(constants::COMMAND_ID_JUMP, actions::do_jump);
		acts.insert(constants::COMMAND_ID_KNIT, actions::do_knit);
		acts.insert(constants::COMMAND_ID_LIGHT, actions::do_light);
		acts.insert(constants::COMMAND_ID_LOOK, actions::do_look);
		acts.insert(constants::COMMAND_ID_NORTH, actions::do_go);
		acts.insert(constants::COMMAND_ID_NORTHEAST, actions::do_go);
		acts.insert(constants::COMMAND_ID_NORTHWEST, actions::do_go);
		acts.insert(constants::COMMAND_ID_OUT, actions::do_go);
		acts.insert(constants::COMMAND_ID_PLAY, actions::do_play);
		acts.insert(constants::COMMAND_ID_PLUGH, actions::do_plugh);
		acts.insert(constants::COMMAND_ID_POUR, actions::do_pour);
		acts.insert(constants::COMMAND_ID_PUSH, actions::do_push);
		acts.insert(constants::COMMAND_ID_QUENCH, actions::do_quench);
		acts.insert(constants::COMMAND_ID_QUIT, actions::do_quit);
		acts.insert(constants::COMMAND_ID_READ, actions::do_read);
		acts.insert(constants::COMMAND_ID_REPAIR, actions::do_repair);
		acts.insert(constants::COMMAND_ID_ROB, actions::do_rob);
		acts.insert(constants::COMMAND_ID_ROLL, actions::do_roll);
		acts.insert(constants::COMMAND_ID_RUB, actions::do_rub);
		acts.insert(constants::COMMAND_ID_SAY, actions::do_say);
		acts.insert(constants::COMMAND_ID_SCORE, actions::do_score);
		acts.insert(constants::COMMAND_ID_SLEEP, actions::do_sleep);
		acts.insert(constants::COMMAND_ID_SOUTH, actions::do_go);
		acts.insert(constants::COMMAND_ID_SOUTHEAST, actions::do_go);
		acts.insert(constants::COMMAND_ID_SOUTHWEST, actions::do_go);
		acts.insert(constants::COMMAND_ID_STARE, actions::do_stare);
		acts.insert(constants::COMMAND_ID_SWIM, actions::do_swim);
		acts.insert(constants::COMMAND_ID_TAKE, actions::do_take);
		acts.insert(constants::COMMAND_ID_TETHER, actions::do_tether);
		acts.insert(constants::COMMAND_ID_TEZAZZLE, actions::do_tezazzle);
		acts.insert(constants::COMMAND_ID_THROW, actions::do_throw);
		acts.insert(constants::COMMAND_ID_UP, actions::do_go);
		acts.insert(constants::COMMAND_ID_WATER, actions::do_water);
		acts.insert(constants::COMMAND_ID_WAVE, actions::do_wave);
		acts.insert(constants::COMMAND_ID_WEST, actions::do_go);
		acts.insert(constants::COMMAND_ID_WIZARD, actions::do_wizard);
		acts.insert(constants::COMMAND_ID_XYZZY, actions::do_xyzzy);
		acts.insert(constants::COMMAND_ID_ACORN, actions::do_acorn);
	}

	#[cfg(debug_assertions)]
	fn add_actions_additional(acts: &mut HashMap<CommandId, ActionFn>) {
		acts.insert(constants::COMMAND_ID_FLASH, actions::do_flash);
		acts.insert(constants::COMMAND_ID_GRAB, actions::do_grab);
		acts.insert(constants::COMMAND_ID_NODE, actions::do_node);
	}

	#[cfg(not(debug_assertions))]
	#[allow(unused_variables)]
	fn add_actions_additional(acts: &mut HashMap<CommandId, ActionFn>) {
	}

	// TODO: make properly static
	fn init_actions() -> HashMap<CommandId, ActionFn> {
		let mut acts: HashMap<CommandId, ActionFn> = HashMap::new();
		CommandCollection::add_actions_common(&mut acts);
		CommandCollection::add_actions_additional(&mut acts);
		acts
	}

	// TODO: make static
	// Create map of command tags (note: not primary names) to Directions
	fn get_tag_dir_map() -> HashMap<CommandId, Direction> {
		let mut tag_dirs = HashMap::new();
		tag_dirs.insert(constants::COMMAND_ID_NORTH, Direction::North);
		tag_dirs.insert(constants::COMMAND_ID_SOUTH, Direction::South);
		tag_dirs.insert(constants::COMMAND_ID_EAST, Direction::East);
		tag_dirs.insert(constants::COMMAND_ID_WEST, Direction::West);
		tag_dirs.insert(constants::COMMAND_ID_NORTHEAST, Direction::Northeast);
		tag_dirs.insert(constants::COMMAND_ID_SOUTHWEST, Direction::Southwest);
		tag_dirs.insert(constants::COMMAND_ID_SOUTHEAST, Direction::Southeast);
		tag_dirs.insert(constants::COMMAND_ID_NORTHWEST, Direction::Northwest);
		tag_dirs.insert(constants::COMMAND_ID_UP, Direction::Up);
		tag_dirs.insert(constants::COMMAND_ID_DOWN, Direction::Down);
		tag_dirs.insert(constants::COMMAND_ID_OUT, Direction::Out);
		tag_dirs.insert(constants::COMMAND_ID_BACK, Direction::Back);
		tag_dirs
	}

	pub fn init(&mut self, buffer: &mut FileBuffer, expected_count: u32) {
		let tag_dirs = CommandCollection::get_tag_dir_map();
		let acts = CommandCollection::init_actions();
		let mut ids: HashSet<CommandId> = HashSet::new(); // Set of id numbers of all commands found in datafile

		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				constants::FILE_SECTION_SEPARATOR => break,
				x => {
					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();
					let id = self.parse_and_insert_command(&words, &acts, &tag_dirs);
					ids.insert(id);
				},
			}
			line = buffer.get_line();
		}
		self.validate(expected_count, &ids);
	}

	fn parse_and_insert_command(&mut self, words: &Vec<&str>, acts: &HashMap<CommandId, ActionFn>, tag_dirs: &HashMap<CommandId, Direction>) -> CommandId {
		let primary = String::from(words[FILE_INDEX_COMMAND_PRIMARY]);
		let properties = data_collection::str_to_u32_certain(words[FILE_INDEX_COMMAND_STATUS], 16);
		let id = data_collection::str_to_u32_certain(words[FILE_INDEX_COMMAND_TAG], 10);

		if let Some(act) = acts.get(&id) {
			let cmd: CommandRef = Rc::new(Box::new(Command::new(primary.clone(), properties, *act)));
			// Insert command by primary name and any aliases
			self.commands.insert(primary.clone(), cmd.clone());
			for i in FILE_INDEX_COMMAND_ALIAS_START..words.len() {
				if !words[i].is_empty() {
					self.commands.insert(String::from(words[i]), cmd.clone());
				}
			}
			// Map localized primary names (as opposed to tags) to Directions
			if cmd.has_property(constants::CTRL_COMMAND_MOVEMENT) {
				match tag_dirs.get(&id) {
					None => panic!("Unknown movement command {}, fail.", id),
					Some(dir) => self.direction_map.insert(primary, *dir),
				};
			}
		}
		id
	}

	// Ensure that all the necessary ids will be available
	fn validate(&self, expected_count: u32, ids: &HashSet<CommandId>) {
		if ids.len() as u32 != expected_count {
			panic!("Error in command collection. Expected [{}] tags, found [{}]", expected_count, ids.len());
		}
		for id in 0..expected_count {
			if !ids.contains(&id) {
				panic!("Error in command collection. Id [{}] not found", id);
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
