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

		terminal::write_full(self.data.get_response("initial"));

		// Process self.player instructions
		while self.player.is_alive() && self.player.is_playing() {
			let inputs: Vec<String> = terminal::read_stub(self.player.get_location().borrow().get_stubname());
			let cmd_name = inputs[0].clone();
			if !cmd_name.is_empty() {
				self.player.increment_instructions();
				match self.data.get_command(cmd_name.clone()) {
					Some(cmd) => {
						let arg: String = if inputs.len() > 1 { inputs[1].clone() } else { String::from("") };
						(**cmd).execute(&self.data, arg, &mut self.player);
					},
					None => {
						terminal::write_full(self.data.get_response("notuigin"));
					},
				}
			}
			// Something in this move killed the self.player; see whether they want to continue
			if !self.player.is_alive() {
				terminal::write_full(self.data.get_response("desreinc"));

				let reincarnate: bool = get_yes_no(self.data.get_response("askreinc"), self.data.get_response("notuigse"));
				match reincarnate {
					true => {
						terminal::write_full(self.data.get_response("doreinc"));
						self.player.set_alive(true);
					},
					false => {
						terminal::write_full(self.data.get_response("ok"));
					},
				}
			}

			else if self.player.is_playing() && !self.player.has_light() {
				terminal::write_full(self.data.get_response("lampno"));
			}
		}
	}
}

// Look for an answer to a yes-no question
fn get_yes_no(question: &str, default: &str) -> bool {

	loop {
		let mut response: Vec<String> = terminal::read_question(question);
		while response.is_empty() {
			response = terminal::read_question(question);
		}

		match response[0].as_ref() {
			"yes" | "y" | "true" => return true,
			"no" | "n" | "false" => return false,
			_ => terminal::write_full(default),
		}
	}
}
