pub struct Item {
	id: u32,
	status: u32,
	size: u32,
	shortname: String,
	longname: String,
	description: String,
	writing: String,
}

impl Item {

	pub fn new(id: u32, status: u32, size: u32, shortname: String, longname: String, description: String, writing: String) -> Item {
		Item {
			id: id,
			status: status,
			size: size,
			shortname: shortname,
			longname: longname,
			description: description,
			writing: writing,
		}
	}

	pub fn get_id(&self) -> u32 {
		self.id
	}

	pub fn get_shortname(&self) -> &str {
		&self.shortname
	}

	pub fn get_longname(&self) -> &str {
		&self.longname
	}

	pub fn mk_full_string(&self) -> String {
		String::from("It is ") + &self.description + "."
	}

	pub fn mk_writing_string(&self) -> String {
		if self.writing.is_empty() {
			String::from("There is no writing to read there.")
		} else {
			String::from("It reads \"") + &self.writing + "\"."
		}
	}
}
