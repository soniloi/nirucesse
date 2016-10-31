use constants;
use data_collection::ItemRef;

pub struct Item {
	id: u32,
	properties: u32,
	size: u32,
	shortname: String,
	longname: String,
	description: String,
	writing: Option<String>,
	location_stated: u32,
	location_true: u32,
	on: bool,
	within: Option<ItemRef>,
}

impl Item {

	pub fn new(id: u32, properties: u32, size: u32, shortname: String, longname: String, description: String, writing: Option<String>, location: u32) -> Item {
		Item {
			id: id,
			properties: properties,
			size: size,
			shortname: shortname,
			longname: longname,
			description: description,
			writing: writing,
			location_stated: location,
			location_true: location,
			on: false,
			within: None,
		}
	}

	pub fn is(&self, id: u32) -> bool {
		id == self.id
	}

	pub fn is_new(&self) -> bool {
		self.location_true == constants::LOCATION_ID_NURSERY
	}

	pub fn retire(&mut self) {
		self.location_true = constants::LOCATION_ID_GRAVEYARD;
	}

	pub fn is_retired(&self) -> bool {
		self.location_true == constants::LOCATION_ID_GRAVEYARD
	}

	// FIXME: probably refactor this out
	pub fn contains_item(&self, id: u32) -> bool {
		match self.within.clone() {
			None => false,
			Some(within) => within.borrow().is_or_contains_item(id),
		}
	}

	pub fn is_or_contains_item(&self, id: u32) -> bool {
		if self.id == id {
			return true;
		}
		match self.within.clone() {
			None => false,
			Some(within) => within.borrow().is_or_contains_item(id),
		}
	}

	pub fn get_location_stated(&self) -> u32 {
		self.location_stated
	}

	pub fn get_location_true(&self) -> u32 {
		self.location_true
	}

	pub fn set_location_stated(&mut self, loc: u32) {
		self.location_stated = loc;
	}

	pub fn set_location_true(&mut self, loc: u32) {
		self.location_true = loc;
	}

	pub fn set_locations(&mut self, loc: u32) {
		self.location_stated = loc;
		self.location_true = loc;
	}

	pub fn has_property(&self, property_code: u32) -> bool {
		self.properties & property_code != 0
	}

	fn has_or_contains_with_property_generic(&self, property_code: u32, on_optional: bool) -> bool {
		if (on_optional || self.on) && self.has_property(property_code) {
			return true;
		}
		match self.within.clone() {
			None => return false,
			Some (within) => return within.borrow().has_or_contains_with_property_generic(property_code, on_optional),
		}
	}

	// Whether the item has some property
	pub fn has_or_contains_with_property(&self, property_code: u32) -> bool {
		self.has_or_contains_with_property_generic(property_code, true)
	}

	// Whether the item has some property, but must be switched on in order for that property to be active
	pub fn has_or_contains_with_switchable_property(&self, property_code: u32) -> bool {
		self.has_or_contains_with_property_generic(property_code, false)
	}

	pub fn set_property(&mut self, property_code: u32, next: bool) {
		if next {
			self.properties |= property_code;
		} else {
			self.properties &= !property_code;
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

	pub fn is_portable(&self) -> bool {
		self.has_property(constants::CTRL_ITEM_MOBILE) && !self.has_property(constants::CTRL_ITEM_OBSTRUCTION)
	}

	pub fn get_treasure_value(&self) -> u32 {
		self.count_treasure_value(0)
	}

	fn count_treasure_value(&self, acc: u32) -> u32 {
		let mut result = acc;
		if self.has_property(constants::CTRL_ITEM_TREASURE) {
			result += 1;
		}
		match self.within.clone() {
			None => result,
			Some(within) => within.borrow().count_treasure_value(result)
		}
	}

	// Return whether an item could fit inside this item, assuming it is a container
	fn can_fit(&self, item: &ItemRef) -> bool {
		item.borrow().get_capacity() < self.size
	}

	// Check that a potential container is a container, that we are not inserting an item into itself, that it is the right kind of container,
	// 	that it is empty, and that it is large enough to hold the item
	// If there is a problem, return the string tag of the reason, otherwise return None
	pub fn has_problem_accepting(&self, item: &ItemRef) -> Option<u32> {
		// Check attributes of container
		if !self.has_property(constants::CTRL_ITEM_CONTAINER) {
			return Some(constants::STR_ID_NOT_CONTAINER);
		}
		if self.is(item.borrow().get_id()) {
			return Some(constants::STR_ID_CONTAINER_INTO_SELF);
		}
		if self.has_property(constants::CTRL_ITEM_CONTAINER_LIQUID) && !item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
			return Some(constants::STR_ID_NOT_SOLID_CONTAINER);
		}
		if !self.has_property(constants::CTRL_ITEM_CONTAINER_LIQUID) && item.borrow().has_property(constants::CTRL_ITEM_LIQUID) {
			return Some(constants::STR_ID_NOT_LIQUID_CONTAINER);
		}

		// Make sure there is nothing already in the container
		match self.within.clone() {
			Some(within) => {
				if within.borrow().is(item.borrow().get_id()) {
					return Some(constants::STR_ID_ALREADY_CONTAINED);
				} else {
					return Some(constants::STR_ID_CONTAINER_FULL);
				}
			},
			None => {
				if !self.can_fit(&item) {
					return Some(constants::STR_ID_NO_FIT);
				}
			},
		}
		None
	}

	// Check that an item can be inserted
	// If there is a problem, return the string tag of the reason, otherwise return None
	pub fn has_problem_inserting(&self) -> Option<u32> {
		if !self.has_property(constants::CTRL_ITEM_MOBILE) || self.has_property(constants::CTRL_ITEM_WEARABLE) { // Items cannot be inserted if they are immobile or would be worn
			return Some(constants::STR_ID_CANNOT_TAKE);
		}
		None
	}

	fn get_switch_status(&self) -> String {
		String::from("currently ") + if self.on {"on"} else {"off"}
	}

	fn get_switch_status_short(&self) -> String {
		String::from(" (") + &self.get_switch_status() + ")"
	}

	fn get_switch_status_long(&self) -> String {
		String::from(". It is ") + &self.get_switch_status()
	}

	fn get_within_status_short(&self, nest: bool, depth: u32) -> String {
		let mut result = String::new();
		if self.has_property(constants::CTRL_ITEM_CONTAINER) {
			match self.within.clone() {
				None => return String::from(" (empty)"),
				Some(contained) => {
					let mut nest_next = false;
					let mut pre = String::new();
					let mut post = String::new();
					if nest {
						nest_next = true;
						pre = pre + "\n\t";
						for _ in 0..depth {
						    pre = pre + "\t";
						}
						pre = pre + " ";
					} else {
						pre = pre + " (";
						post = post + ")";
					}
					result = result + &pre + "containing " + &contained.borrow().get_longname() + &contained.borrow().get_within_status_short(nest_next, depth + 1) + &post;
				},
			}
		}
		result
	}

	fn get_within_status_long(&self) -> String {
		let mut result = String::from(". It ");
		match self.within.clone() {
			None => result = result + "is empty",
			Some(contained) => result = result + "contains " + &contained.borrow().get_longname() + &contained.borrow().get_within_status_short(false, 1),
		}
		result
	}

	// Return the name of this item as it would be displayed in an inventory listing
	pub fn get_inventoryname(&self) -> String {

		let mut result: String = String::new();
		if self.has_property(constants::CTRL_ITEM_WEARABLE) {
			result = result + "(wearing) ";
		}
		result = result + &self.longname;
		if self.has_property(constants::CTRL_ITEM_SWITCHABLE) {
			result = result + &self.get_switch_status_short();
		}
		result + &self.get_within_status_short(true, 1)
	}

	// Return the name of this item as it would be displayed in a location listing
	pub fn get_locationname(&self) -> String {
		let mut result = String::new();
		if !self.has_property(constants::CTRL_ITEM_SILENT) {
			result = result + "\nThere is " + &self.longname;
			if self.has_property(constants::CTRL_ITEM_SWITCHABLE) {
				result = result + &self.get_switch_status_short();
			}
			result = result + &self.get_within_status_short(false, 1) + " here";
			result = result + if self.has_property(constants::CTRL_ITEM_OBSTRUCTION) || self.has_property(constants::CTRL_ITEM_TREASURE) {"!"} else {"."};
		}
		result
	}

	pub fn get_capacity(&self) -> u32 {
		self.size
	}

	// Return size of item; this is safe, as non-containers simply have a None within
	pub fn get_size(&self) -> u32 {
		match self.within.clone() {
			None => return self.size,
			Some(within) => return self.size + within.borrow().get_size(),
		}
	}

	pub fn is_on(&self) -> bool {
		self.on
	}

	pub fn set_on(&mut self, next: bool) {
		self.on = next;
	}

	pub fn remove_item_certain(&mut self, id: u32) {
		match self.within.clone() {
			None => panic!("Data corruption seeking item [{}], fail.", id),
			Some(within) => {
				let is_item = within.borrow().is(id);
				let is_liquid = within.borrow().has_property(constants::CTRL_ITEM_LIQUID);
				if is_item {
					if !is_liquid {
						within.borrow_mut().retire();
					}
					self.within = None;
				} else {
					within.borrow_mut().remove_item_certain(id);
				}
			},
		}
	}

	pub fn is_empty(&self) -> bool {
		match self.within {
			None => true,
			_ => false,
		}
	}

	pub fn set_within(&mut self, within: Option<ItemRef>) {
		match within.clone() {
			None => {},
			Some(with) => with.borrow_mut().set_locations(self.id),
		}
		self.within = within;
	}

	pub fn remove_within(&mut self) -> Option<ItemRef> {
		let within = self.within.clone();
		self.within = None;
		within
	}

	pub fn mk_full_string(&self, description_wrapper: &str) -> String {
		let mut full = String::new() + &self.description;
		if self.has_property(constants::CTRL_ITEM_SWITCHABLE) {
			full = full + &self.get_switch_status_long();
		}
		if self.has_property(constants::CTRL_ITEM_CONTAINER) {
			full = full + &self.get_within_status_long();
		}
		String::from(description_wrapper).replace("$0", &full)
	}

	pub fn mk_writing_string(&self, no_writing: &str, writing_wrapper: &str) -> String {
		match self.writing.clone() {
			None => String::from(no_writing),
			Some(writ) => String::from(writing_wrapper).replace("$0", &writ),
		}
	}
}
