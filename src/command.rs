use constants;
use data_collection::DataCollection;
use player::Player;
use terminal;

#[derive(PartialEq, Eq)]
pub enum ArgumentType {
	Any,
	Present,
	Inventory,
}

pub type ActionFn = fn(items: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType);

pub struct Command {
	name: String,
	properties: u32,
	handler: ActionFn,
}

impl Command {

	pub fn new(name: String, properties: u32, handler: ActionFn) -> Command {
		Command {
			name: name,
			properties: properties,
			handler: handler,
		}
	}

	pub fn has_property(&self, property: u32) -> bool {
		self.properties & property != 0
	}

	pub fn execute(&self, data: &DataCollection, arg: String, player: &mut Player) {
		let h = self.handler;
		let mut actual_arg = arg;

		// Command takes no argument, but player gave one anyway
		if !self.has_property(constants::CTRL_COMMAND_ARG_MANDATORY) && !self.has_property(constants::CTRL_COMMAND_ARG_OPTIONAL) && !actual_arg.is_empty() {
			terminal::write_full(data.get_response(constants::STR_ID_ARG_EXTRA));
			return;
		}

		// Command takes an argument, but player didn't give one
		if self.has_property(constants::CTRL_COMMAND_ARG_MANDATORY) && actual_arg.is_empty() && !self.has_property(constants::CTRL_COMMAND_MOVEMENT) {
			let further_args = terminal::read_question(&data.get_response_param(constants::STR_ID_ARG_GET, &self.name));
			actual_arg = String::new() + &further_args[0];
		}

		// Movement handling
		if self.has_property(constants::CTRL_COMMAND_MOVEMENT) {
			actual_arg = String::new() + &self.name;
		}

		// Argument type
		let mut arg_type = ArgumentType::Any;
		if self.has_property(constants::CTRL_COMMAND_INVENTORY) {
			arg_type = ArgumentType::Inventory;
		} else if self.has_property(constants::CTRL_COMMAND_PRESENT) {
			arg_type = ArgumentType::Present;
		}

		h(data, actual_arg, player, arg_type);
	}
}
