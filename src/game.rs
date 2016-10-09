use data_collection::DataCollection;
use player::Player;
use terminal;

pub struct Game {
	data: DataCollection,
	player: Player,
}

impl Game {

	pub fn new(data: DataCollection, player: Player) -> Game {
		Game {
			data: data,
			player: player,
		}
	}

	pub fn play(&mut self) {

		terminal::write_full(self.data.get_response(57));

		while self.player.is_playing() {
			self.process_input();

			if !self.player.has_air() {
				terminal::write_full(self.data.get_response(73));
				self.player.die(&self.data);
			}

			if !self.player.has_gravity() {
				self.player.float(&self.data);
			}

			if !self.player.is_alive() {
				self.player.drop_on_death(self.data.get_location_safe());
				self.process_reincarnation();
			} else if self.player.is_playing() {
				match self.data.get_event(self.player.get_instructions()) {
					None => {},
					Some(event) => terminal::write_full(event),
				}
			}
		}
	}

	// Process commands from player
	fn process_input(&mut self) {
		let inputs: Vec<String> = terminal::read_stub(&self.player.get_location_stubname());
		if inputs.is_empty() {
			return;
		}

		self.player.increment_instructions();

		// First try verb-noun
		let mut cmd_name_tentative = inputs[0].clone();
		match self.data.get_command(cmd_name_tentative.clone()) {
			None => {},
			Some(cmd) => {
				let arg: String = if inputs.len() > 1 { inputs[1].clone() } else { String::from("") };
				(**cmd).execute(&self.data, arg, &mut self.player);
				return;
			},
		}

		// That didn't parse, so try noun-verb instead
		if inputs.len() >= 2 {
			cmd_name_tentative = inputs[1].clone();
			match self.data.get_command(cmd_name_tentative.clone()) {
				None => {},
				Some(cmd) => {
					if cmd.is_invertible() {
						let arg: String = inputs[0].clone();
						(**cmd).execute(&self.data, arg, &mut self.player);
						return;
					}
				}
			}
		}

		terminal::write_full(self.data.get_response(103));
	}

	// Reincarnate the player, if requested
	fn process_reincarnation(&mut self) {
		terminal::write_full(self.data.get_response(27));
		let reincarnate: bool = terminal::get_yes_no(self.data.get_response(8), self.data.get_response(103));
		match reincarnate {
			true => {
				terminal::write_full(self.data.get_response(28));
				self.player.set_alive(true);
			},
			false => {
				terminal::write_full(self.data.get_response(110));
				self.player.set_playing(false);
			},
		}
	}
}
