use data_collection::DataCollection;
use player::ItemManipFn;
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

pub fn do_drink(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::drink);
}

pub fn do_drop(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::drop);
}

pub fn do_empty(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::empty);
}

#[allow(unused_variables)]
pub fn do_explain(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(data.get_explanation(&arg));
}

#[allow(unused_variables)]
pub fn do_go(data: &DataCollection, arg: String, player: &mut Player) {
	// FIXME: this should be passing the direction tag, not the localized primary name
	let dir = data.get_direction_enum(&arg);
	player.go(data, dir);
}

pub fn do_feed(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::feed);
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
	match data.get_hint(&arg) {
		None => terminal::write_full(data.get_hint_certain("default")),
		Some(hint) => {
			terminal::write_full(data.get_response("hintwarn"));
			let confirm = terminal::get_yes_no(data.get_response("asksure"), data.get_response("notuigse"));
			if confirm {
				terminal::write_full(hint);
				player.increment_hints();
			} else {
				terminal::write_full(data.get_response("ok"));
			}
		},
	}
}

pub fn do_ignore(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::ignore);
}

pub fn do_insert(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::insert);
}

#[allow(unused_variables)]
pub fn do_inventory(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&player.mk_inventory_string());
}

pub fn do_light(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::light);
}

#[allow(unused_variables)]
pub fn do_look(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(&player.get_look(data));
}

pub fn do_play(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::play);
}

pub fn do_quench(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::quench);
}

#[allow(unused_variables)]
pub fn do_quit(data: &DataCollection, arg: String, player: &mut Player) {
	player.set_playing(false);
}

pub fn do_read(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::read);
}

pub fn do_rub(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::rub);
}

#[allow(unused_variables)]
pub fn do_score(data: &DataCollection, arg: String, player: &mut Player) {
	player.decrement_instructions(); // Requesting score does not count as an instruction
	terminal::write_full(&player.get_score_str(data));
}

#[allow(unused_variables)]
pub fn do_tezazzle(data: &DataCollection, arg: String, player: &mut Player) {
	player.tezazzle(data);
}

pub fn do_take(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::take);
}

pub fn do_throw(data: &DataCollection, arg: String, player: &mut Player) {
	manipulate_item(data, arg, player, Player::throw);
}

#[allow(unused_variables)]
pub fn do_xyzzy(data: &DataCollection, arg: String, player: &mut Player) {
	terminal::write_full(data.get_response("ok"));
}

fn manipulate_item(data: &DataCollection, arg: String, player: &mut Player, act: ItemManipFn) {
	match data.get_item(arg) {
		None => terminal::write_full(data.get_response("nonowhat")),
		Some(i) => act(player, data, i),
	}
}
