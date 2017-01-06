use command::ArgumentType;
use constants;
#[cfg(debug_assertions)]
use data_collection;
use data_collection::{DataCollection, ItemId, StringId};
use player::ItemManipFn;
use player::Player;

use terminal;

#[cfg(debug_assertions)]
#[allow(unused_variables)]
pub fn do_flash(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	let mut actual_arg = arg;
	if actual_arg.is_empty() {
		let further_args = terminal::read_question(&data.get_response(constants::STR_ID_WHERE_FLASH));
		actual_arg = String::new() + &further_args[0];
	}
	match data_collection::str_to_u32(&actual_arg, 10) {
		Err(why) => terminal::write_full(data.get_response(constants::STR_ID_INVALID_NUMBER)),
		Ok(next_id) => {
			match data.get_location(next_id) {
				None => terminal::write_full(&data.get_response_param(constants::STR_ID_INVALID_LOCATION, &next_id.to_string())),
				Some(next) => player.flash(data, next.clone()),
			};
		},
	};
}

#[cfg(debug_assertions)]
pub fn do_grab(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::grab);
}

#[cfg(debug_assertions)]
#[allow(unused_variables)]
pub fn do_node(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(&player.get_node(data));
}

#[allow(unused_variables)]
pub fn do_acorn(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.acorn(data);
}

pub fn do_attack(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::attack);
}

pub fn do_burn(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::burn);
}

pub fn do_call(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::call);
}

#[allow(unused_variables)]
pub fn do_climb(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(constants::STR_ID_DISAMBIGUATE_CLIMB));
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

pub fn do_eat(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::eat);
}

pub fn do_empty(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::empty);
}

pub fn do_exchange(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::exchange);
}

#[allow(unused_variables)]
pub fn do_explain(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_explanation(&arg));
}

#[allow(unused_variables)]
pub fn do_fairy(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.fairy(data);
}

pub fn do_feed(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::feed);
}

#[allow(unused_variables)]
pub fn do_fish(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.fish(data);
}

pub fn do_fly(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::fly);
}

pub fn do_give(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::give);
}

#[allow(unused_variables)]
pub fn do_go(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	let dir = data.get_direction_enum(&arg);
	player.go(data, dir);
}

#[allow(unused_variables)]
pub fn do_go_disambiguate(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(constants::STR_ID_DISAMBIGUATE_GO));
}

#[allow(unused_variables)]
pub fn do_help(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.decrement_instructions(); // Requesting help does not count as an instruction
	terminal::write_full(data.get_response(constants::STR_ID_WELCOME));
}

#[allow(unused_variables)]
pub fn do_hint(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	match data.get_hint(&arg) {
		None => terminal::write_full(data.get_hint_certain(constants::STR_DEFAULT)),
		Some(hint) => {
			terminal::write_full(data.get_response(constants::STR_ID_HINT_FOUND));
			let confirm = terminal::get_yes_no(data.get_response(constants::STR_ID_SURE_ASK), data.get_response(constants::STR_ID_NO_UNDERSTAND_SELECTION));
			if confirm {
				terminal::write_full(hint);
				player.increment_hints();
			} else {
				terminal::write_full(data.get_response(constants::STR_ID_OK));
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
	terminal::write_full(&player.mk_inventory_string(data));
}

#[allow(unused_variables)]
pub fn do_jump(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.jump(data);
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
	if !arg.is_empty() {
		terminal::write_full(data.get_response(constants::STR_ID_DISAMBIGUATE_LOOK));
	} else {
		terminal::write_full(&player.get_look(data));
	}
}

#[allow(unused_variables)]
pub fn do_marble(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.marble(data);
}

pub fn do_play(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::play);
}

#[allow(unused_variables)]
pub fn do_plugh(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(constants::STR_ID_HOLLOW));
}

pub fn do_pour(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::pour);
}

pub fn do_push(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::push);
}

pub fn do_quench(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::quench);
}

#[allow(unused_variables)]
pub fn do_quit(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.decrement_instructions(); // Quitting does not count as an instruction
	let confirm = terminal::get_yes_no(data.get_response(constants::STR_ID_SURE_ASK), data.get_response(constants::STR_ID_NO_UNDERSTAND_SELECTION));
	if confirm {
		player.set_playing(false);
	} else {
		terminal::write_full(data.get_response(constants::STR_ID_OK));
	}
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

#[allow(unused_variables)]
pub fn do_robot(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.robot(data);
}

pub fn do_roll(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::roll);
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
	terminal::write_full(&player.get_score_str(data, constants::STR_ID_SCORE_CURRENT));
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
pub fn do_swim(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_HOW));
}

#[allow(unused_variables)]
pub fn do_tezazzle(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.tezazzle(data);
}

pub fn do_take(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::take);
}

pub fn do_tether(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::tether);
}

pub fn do_throw(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	manipulate_item(data, arg, arg_type, player, Player::throw);
}

#[allow(unused_variables)]
pub fn do_water(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(constants::STR_ID_DISAMBIGUATE_WATER));
}

#[allow(unused_variables)]
pub fn do_wave(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.wave(data);
}

#[allow(unused_variables)]
pub fn do_wizard(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	player.wizard(data);
}

#[allow(unused_variables)]
pub fn do_xyzzy(data: &DataCollection, arg: String, player: &mut Player, arg_type: ArgumentType) {
	terminal::write_full(data.get_response(constants::STR_ID_OK));
}

fn manipulate_item(data: &DataCollection, arg: String, arg_type: ArgumentType, player: &mut Player, act: ItemManipFn) {
	match data.get_item_by_name(arg) {
		None => terminal::write_full(data.get_response(constants::STR_ID_NO_KNOW_WHO_WHAT)),
		Some(i) => {
			let item_id = i.borrow().get_id();
			let is_mobile = i.borrow().has_property(constants::CTRL_ITEM_MOBILE);
			match problem_with_item_manipulation(player, item_id, arg_type, is_mobile) {
				Some(problem) => terminal::write_full(&data.get_response_param(problem, &i.borrow().get_shortname())),
				None => act(player, data, i),
			}
		},
	}
}

fn problem_with_item_manipulation(player: &Player, item_id: ItemId, arg_type: ArgumentType, is_mobile: bool) -> Option<StringId> {
	if arg_type == ArgumentType::Inventory && !player.has_item_inventory(item_id) {
		return Some(constants::STR_ID_NO_HAVE_INVENTORY);
	} else if arg_type == ArgumentType::Present && !player.has_item_present(item_id) {
		return Some(constants::STR_ID_NO_SEE_HERE);
	}
	if !is_mobile {
		if let Some(_) = player.get_current_obstruction() {
			return Some(constants::STR_ID_FIXTURE_OBSTRUCTED);
		}
	}
	None
}