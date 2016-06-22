pub struct Item {
	id: u64,
	status: u32,
	size: u32,	
	longname: String,
	fullname: String,
	writing: String,
}

impl Item {

	pub fn new(id: u64, status: u32, size: u32, longname: String, fullname: String, writing: String) -> Item {
		Item {
			id: id,
			status: status,
			size: size,
			longname: longname,
			fullname: fullname,
			writing: writing,
		}
	}

	pub fn write_out(&self) {
		println!("Item [id={}] [status={}] [longname={}] [fullname={}] [writing={}] [size={}]", self.id, self.status, self.size, self.longname, self.fullname, self.writing);
	}
}