use command::ArgumentType;
use data_collection::DataCollection;
use player::ItemManipFn;
use player::Player;

use terminal;

pub fn do_attack(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::attack);
}

#[allow(unused_variables)]
pub fn do_avnarand(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.avnarand(data);
}

pub fn do_burn(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::burn);
}

pub fn do_call(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::call);
}

#[allow(unused_variables)]
pub fn do_chimbu(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.chimbu(data);
}

#[allow(unused_variables)]
pub fn do_commands(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(&data.get_commands_non_secret());
}

pub fn do_cook(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::cook);
}

pub fn do_describe(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::describe);
}

pub fn do_drink(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::drink);
}

pub fn do_drop(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::drop);
}

pub fn do_empty(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::empty);
}

#[allow(unused_variables)]
pub fn do_explain(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_explanation(&arg));
}

#[allow(unused_variables)]
pub fn do_fish(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.fish(data);
}

pub fn do_give(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::give);
}

#[allow(unused_variables)]
pub fn do_go(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	// FIXME: this should be passing the direction tag, not the localized primary name
	let dir = data.get_direction_enum(&arg);
	player.go(data, dir);
}

pub fn do_feed(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::feed);
}

#[allow(unused_variables)]
pub fn do_go_disambiguate(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(49));
}

#[allow(unused_variables)]
pub fn do_help(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.decrement_instructions(); // Requesting help does not count as an instruction
	terminal::write_full(data.get_response(50));
}

#[allow(unused_variables)]
pub fn do_hint(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	match data.get_hint(&arg) {
		None => terminal::write_full(data.get_hint_certain("default")),
		Some(hint) => {
			terminal::write_full(data.get_response(54));
			let confirm = terminal::get_yes_no(data.get_response(9), data.get_response(104));
			if confirm {
				terminal::write_full(hint);
				player.increment_hints();
			} else {
				terminal::write_full(data.get_response(110));
			}
		},
	}
}

pub fn do_ignore(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::ignore);
}

pub fn do_insert(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::insert);
}

#[allow(unused_variables)]
pub fn do_inventory(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(&player.mk_inventory_string());
}

#[allow(unused_variables)]
pub fn do_knit(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.knit(data);
}

pub fn do_light(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::light);
}

#[allow(unused_variables)]
pub fn do_look(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(&player.get_look(data));
}

#[cfg(debug_assertions)]
#[allow(unused_variables)]
pub fn do_node(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(&player.get_node(data));
}

#[cfg(not(debug_assertions))]
#[allow(unused_variables)]
pub fn do_node(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(103));
}

pub fn do_play(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::play);
}

#[allow(unused_variables)]
pub fn do_plugh(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(122));
}

pub fn do_pour(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::pour);
}

pub fn do_quench(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::quench);
}

#[allow(unused_variables)]
pub fn do_quit(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.set_playing(false);
}

pub fn do_read(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::read);
}

pub fn do_repair(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::repair);
}

pub fn do_rob(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::rob);
}

pub fn do_rub(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::rub);
}

#[allow(unused_variables)]
pub fn do_say(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.say(data, &arg);
}

#[allow(unused_variables)]
pub fn do_score(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.decrement_instructions(); // Requesting score does not count as an instruction
	terminal::write_full(&player.get_score_str(data));
}

#[allow(unused_variables)]
pub fn do_sleep(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.sleep(data);
}

#[allow(unused_variables)]
pub fn do_stare(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.stare(data);
}

#[allow(unused_variables)]
pub fn do_tezazzle(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.tezazzle(data);
}

pub fn do_take(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::take);
}

pub fn do_throw(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::throw);
}

#[allow(unused_variables)]
pub fn do_xyro(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.xyro(data);
}

#[allow(unused_variables)]
pub fn do_xyzzy(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(110));
}

#[allow(unused_variables)]
pub fn do_ziqua(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.ziqua(data);
}

fn manipulate_item(data: &DataCollection, arg: String, arg_type: ArgumentType, player: &mut Player, act: ItemManipFn) {
	match data.get_item_by_name(arg) {
		None => terminal::write_full(data.get_response(98)),
		Some(i) => {
			let item_id = i.borrow().get_id();
			match problem_with_item_manipulation(player, item_id, arg_type) {
				Some(problem) => terminal::write_full(&data.get_response_param(problem, &i.borrow().get_shortname())),
				None => act(player, data, i),
			}
		},
	}
}

fn problem_with_item_manipulation(player: &Player, item_id: u32, arg_type: ArgumentType) -> Option<u32> {
	if arg_type == ArgumentType::Inventory && !player.has_item_inventory(item_id) {
		Some(74);
	} else if arg_type == ArgumentType::Present && !player.has_item_present(item_id) {
		Some(100);
	}
	None
}