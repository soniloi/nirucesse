use item_collection::ItemCollection;
use player::Player;
use terminal;

pub fn do_describe(items: &ItemCollection, arg: &str, player: &mut Player) {
	match items.get(arg) {
		None => {
			terminal::write_full("I do not know who or what that is.");
			return;
		},
		Some(i) => {
			player.describe(i);
		}
	}
}

pub fn do_drop(items: &ItemCollection, arg: &str, player: &mut Player) {
	match items.get(arg) {
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
pub fn do_go(items: &ItemCollection, arg: &str, player: &mut Player) {
	player.go(String::from(arg));
}

#[allow(unused_variables)]
pub fn do_inventory(items: &ItemCollection, arg: &str, player: &mut Player) {
	terminal::write_full(&player.mk_inventory_string());
}

#[allow(unused_variables)]
pub fn do_look(items: &ItemCollection, arg: &str, player: &mut Player) {
	terminal::write_full(&player.mk_location_string());
}

#[allow(unused_variables)]
pub fn do_quit(items: &ItemCollection, arg: &str, player: &mut Player) {
	player.set_playing(false);
}

pub fn do_take(items: &ItemCollection, arg: &str, player: &mut Player) {
	match items.get(arg) {
		None => {
			terminal::write_full("I do not know who or what that is.");
			return;
		},
		Some(i) => {
			player.pick_up(i);
		}
	}
}
