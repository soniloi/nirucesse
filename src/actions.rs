use data_collection::DataCollection;
use player::Player;
use terminal;

#[allow(unused_variables)]
pub fn do_commands(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&data.get_commands_non_secret());
}

pub fn do_describe(data: &DataCollection, arg: String, player: &mut Player) {
	if !player.has_light() {
		terminal::write_full(data.get_response("cantsee"));
		return;
	}
	match data.get_item(arg) {
		None => {
			terminal::write_full("I do not know who or what that is.");
			return;
		},
		Some(i) => {
			player.describe(i);
		}
	}
}

pub fn do_drop(data: &DataCollection, arg: String, player: &mut Player) {
	match data.get_item(arg) {
		None => {
			terminal::write_full("I do not know who or what that is.");
			return;
		},
		Some(i) => {
			player.drop(i);
		}
	}
}

#[allow(unused_variables)]
pub fn do_explain(data: &DataCollection, arg: String, player: &mut Player) {
	match data.get_explanation(&arg) {
		None => terminal::write_full("I have no explanation for that."),
		Some(explanation) => terminal::write_full(explanation),
	}
}

#[allow(unused_variables)]
pub fn do_go(data: &DataCollection, arg: String, player: &mut Player) {
	player.go(data, String::from(arg));
}

#[allow(unused_variables)]
pub fn do_go_disambiguate(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full("Use compass points or directions (e.g. \"north\", \"down\") to travel to a new location.");
}

#[allow(unused_variables)]
pub fn do_help(data: &DataCollection, arg: String, player: &mut Player) {
	player.decrement_instructions(); // Requesting help does not count as an instruction
	terminal::write_full(data.get_response("help"));
}

pub fn do_hint(data: &DataCollection, arg: String, player: &mut Player) {
	match data.get_hint(&arg) {
		None => terminal::write_full("I have no hints to offer about such a thing."),
		Some(hint) => terminal::write_full(hint),
	}
	player.increment_hints();
}

#[allow(unused_variables)]
pub fn do_inventory(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&player.mk_inventory_string());
}

#[allow(unused_variables)]
pub fn do_look(data: &DataCollection, arg: String, player: &mut Player) {
	if !player.has_light() {
		terminal::write_full(data.get_response("cantsee"));
		return;
	}
	terminal::write_full(&player.mk_location_string());
}

#[allow(unused_variables)]
pub fn do_quit(data: &DataCollection, arg: String, player: &mut Player) {
	player.set_playing(false);
}

pub fn do_read(data: &DataCollection, arg: String, player: &mut Player) {
	match data.get_item(arg) {
		None => {
			terminal::write_full("I do not know who or what that is.");
			return;
		},
		Some(i) => {
			player.read(i);
		}
	}
}

#[allow(unused_variables)]
pub fn do_score(data: &DataCollection, arg: String, player: &mut Player) {
	player.decrement_instructions(); // Requesting score does not count as an instruction
	let score_str = String::from("You currently have a score of ") + &player.get_score().to_string() +
		" points. You have died " + &player.get_deaths().to_string() + 
		" times. You have entered " + &player.get_instructions().to_string() +
		" instructions, and requested " + &player.get_hints().to_string() + " hints.";
	terminal::write_full(&score_str);
}

pub fn do_take(data: &DataCollection, arg: String, player: &mut Player) {
	match data.get_item(arg) {
		None => {
			terminal::write_full("I do not know who or what that is.");
			return;
		},
		Some(i) => {
			player.pick_up(i);
		}
	}
}

#[allow(unused_variables)]
pub fn do_xyzzy(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(data.get_response("ok"));
}
