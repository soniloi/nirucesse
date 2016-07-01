pub struct Item {
	id: u64,
	status: u32,
	size: u32,
	shortname: String,
	longname: String,
	description: String,
	writing: String,
}

impl Item {

	pub fn new(id: u64, status: u32, size: u32, shortname: String, longname: String, description: String, writing: String) -> Item {
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

	pub fn get_id(&self) -> u64 {
		self.id
	}

	pub fn get_longname(&self) -> &str {
		&self.longname
	}

	pub fn write_out(&self) {
		println!("Item [id={}] [status={}] [size={}] [shortname={}] [longname={}] [description={}] [writing={}]", self.id, self.status, self.size, self.shortname, self.longname, self.description, self.writing);
	}
}
