const CTRL_ITEM_MOBILE: u32 = 0x2; // Whether the item is fixed or mobile (carryable)
const CTRL_ITEM_OBSTRUCTION: u32 = 0x4; // Whether the item is an obstruction
const CTRL_ITEM_GIVES_LIGHT: u32 = 0x10; // Whether the item emits light

pub struct Item {
	id: u32,
	properties: u32,
	size: u32,
	shortname: String,
	longname: String,
	description: String,
	writing: String,
}

impl Item {

	pub fn new(id: u32, properties: u32, size: u32, shortname: String, longname: String, description: String, writing: String) -> Item {
		Item {
			id: id,
			properties: properties,
			size: size,
			shortname: shortname,
			longname: longname,
			description: description,
			writing: writing,
		}
	}

	pub fn is(&self, id: u32) -> bool {
		id == self.id
	}

	fn has_property(&self, property: u32) -> bool {
		self.properties & property != 0
	}

	pub fn is_mobile(&self) -> bool {
		self.has_property(CTRL_ITEM_MOBILE)
	}

	pub fn has_light(&self) -> bool {
		self.has_property(CTRL_ITEM_GIVES_LIGHT)
	}

	pub fn is_obstruction(&self) -> bool {
		self.has_property(CTRL_ITEM_OBSTRUCTION)
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

	pub fn get_size(&self) -> u32 {
		self.size
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
