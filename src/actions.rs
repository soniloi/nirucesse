use std::rc::Rc;

use data_collection::DataCollection;
use item::Item;
use player::Player;

use terminal;

#[allow(unused_variables)]
pub fn do_avnarand(data: &DataCollection, arg: String, player: &mut Player) {
	player.avnarand(data);
}

pub fn do_burn(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::burn);
}

#[allow(unused_variables)]
pub fn do_commands(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&data.get_commands_non_secret());
}

pub fn do_describe(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::describe);
}

pub fn do_drop(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::drop);
}

#[allow(unused_variables)]
pub fn do_explain(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(data.get_explanation(&arg));
}

#[allow(unused_variables)]
pub fn do_go(data: &DataCollection, arg: String, player: &mut Player) {
	// FIXME: this should be passing the direction tag, not the localized primary name
	let dir = data.get_direction_enum(arg);
	player.go(data, dir);
}

#[allow(unused_variables)]
pub fn do_go_disambiguate(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(data.get_response("godisamb"));
}

#[allow(unused_variables)]
pub fn do_help(data: &DataCollection, arg: String, player: &mut Player) {
	player.decrement_instructions(); // Requesting help does not count as an instruction
	terminal::write_full(data.get_response("help"));
}

pub fn do_hint(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(data.get_hint(&arg));
	player.increment_hints();
}

#[allow(unused_variables)]
pub fn do_inventory(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&player.mk_inventory_string());
}

#[allow(unused_variables)]
pub fn do_look(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&player.get_look(data));
}

#[allow(unused_variables)]
pub fn do_quit(data: &DataCollection, arg: String, player: &mut Player) {
	player.set_playing(false);
}

pub fn do_read(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::read);
}

#[allow(unused_variables)]
pub fn do_score(data: &DataCollection, arg: String, player: &mut Player) {
	player.decrement_instructions(); // Requesting score does not count as an instruction
	terminal::write_full(&player.get_score_str());
}

pub fn do_take(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::take)
}

#[allow(unused_variables)]
pub fn do_xyzzy(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(data.get_response("ok"));
}

fn manipulate_item(data: &DataCollection, arg: String, player: &mut Player, act: fn(player: &mut Player, data: &DataCollection, item: &Rc<Box<Item>>)) {
	match data.get_item(arg) {
		None => {
			terminal::write_full(data.get_response("nonowhat"));
			return;
		},
		Some(i) => {
			act(player, data, i);
		}
	}
}
