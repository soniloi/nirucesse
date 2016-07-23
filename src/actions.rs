use data_collection::DataCollection;
use player::Player;
use terminal;

#[allow(unused_variables)]
pub fn do_commands(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&data.get_commands_non_secret());
}

pub fn do_describe(data: &DataCollection, arg: String, player: &mut Player) {
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
pub fn do_go(data: &DataCollection, arg: String, player: &mut Player) {
	player.go(String::from(arg));
}

#[allow(unused_variables)]
pub fn do_go_disambiguate(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full("Use compass points or directions (e.g. \"north\", \"down\") to travel to a new location.");
}

#[allow(unused_variables)]
pub fn do_inventory(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&player.mk_inventory_string());
}

#[allow(unused_variables)]
pub fn do_look(data: &DataCollection, arg: String, player: &mut Player) {
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
	terminal::write_full("OK.");
}
