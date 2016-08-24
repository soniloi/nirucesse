const CTRL_ITEM_MOBILE: u32 = 0x2; // Whether the item is fixed or mobile (carryable)
const CTRL_ITEM_OBSTRUCTION: u32 = 0x4; // Whether the item is an obstruction
const CTRL_ITEM_GIVES_LIGHT: u32 = 0x10; // Whether the item emits light
const CTRL_ITEM_GIVES_AIR: u32 = 0x20; // Whether the item enables player to breathe
const CTRL_ITEM_FRAGILE: u32 = 0x200; // Whether the item would survive throwing, dropping from heights, etc
const CTRL_ITEM_WEARABLE: u32 = 0x400; // Whether the item is to be worn by the player rather than carried
const CTRL_ITEM_ESSENTIAL: u32 = 0x1000; // Whether the item is essential to basic gameplay

pub struct Item {
	id: u32,
	properties: u32,
	size: u32,
	shortname: String,
	longname: String,
	description: String,
	writing: Option<String>,
}

impl Item {

	pub fn new(id: u32, properties: u32, size: u32, shortname: String, longname: String, description: String, writing: Option<String>) -> Item {
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

	pub fn has_air(&self) -> bool {
		self.has_property(CTRL_ITEM_GIVES_AIR)
	}

	pub fn is_obstruction(&self) -> bool {
		self.has_property(CTRL_ITEM_OBSTRUCTION)
	}

	pub fn is_wearable(&self) -> bool {
		self.has_property(CTRL_ITEM_WEARABLE)
	}

	pub fn is_essential(&self) -> bool {
		self.has_property(CTRL_ITEM_ESSENTIAL)
	}

	pub fn is_fragile(&self) -> bool {
		self.has_property(CTRL_ITEM_FRAGILE)
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

	// Return the name of this item as it would be displayed in an inventory listing
	pub fn get_inventoryname(&self) -> String {
		let mut result: String = String::new();
		if self.is_wearable() {
			result = result + "(wearing) ";
		}
		result + &self.longname
	}

	pub fn get_size(&self) -> u32 {
		self.size
	}

	pub fn mk_full_string(&self, description_start: &str, description_end: &str) -> String {
		String::from(description_start) + &self.description + description_end
	}

	pub fn mk_writing_string(&self, no_writing: &str, writing_start: &str, writing_end: &str) -> String {
		match self.writing.clone() {
			None => String::from(no_writing),
			Some(writ) => String::from(writing_start) + &writ + writing_end,
		}
	}
}
