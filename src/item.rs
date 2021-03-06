use constants;
use data_collection::{Id, ItemId, ItemProperties, ItemRef, StringId};

pub type ItemCheckFn = fn(primary: &Item, other: &ItemRef) -> Option<StringId>;

const STR_CONTAINS_LONG: &'static str = ". It contains ";
const STR_CONTAINS_SHORT: &'static str = "containing ";
const STR_DOT: &'static str = ".";
const STR_EMPTY_LONG: &'static str = ". It is empty";
const STR_EMPTY_SHORT: &'static str = " (empty)";
const STR_EXCLAMATION: &'static str = "!";
const STR_OFF: &'static str = "off";
const STR_ON: &'static str = "on";
const STR_SWITCH_LONG: &'static str = ". It is currently $0";
const STR_SWITCH_SHORT: &'static str = " (currently $0)";
const STR_THERE_IS: &'static str = "\nThere is $0 here";
const STR_WEARING: &'static str = "(wearing) ";

pub struct Item {
	id: ItemId,
	properties: ItemProperties,
	size: u32,
	shortname: String,
	longname: String,
	description: String,
	writing: Option<String>,
	location: Id, // This may be a LocationId, an InventoryId, or an ItemId
	on: bool,
	within: Option<ItemRef>,
}

impl Item {

	pub fn new(id: ItemId, properties: ItemProperties, size: u32, shortname: String, longname: String, description: String, writing: Option<String>, location: Id) -> Item {
		Item {
			id: id,
			properties: properties,
			size: size,
			shortname: shortname,
			longname: longname,
			description: description,
			writing: writing,
			location: location,
			on: false,
			within: None,
		}
	}

	pub fn is(&self, id: ItemId) -> bool {
		id == self.id
	}

	pub fn is_new(&self) -> bool {
		self.location == constants::LOCATION_ID_NURSERY
	}

	pub fn retire(&mut self) {
		self.location = constants::LOCATION_ID_GRAVEYARD;
	}

	pub fn is_retired(&self) -> bool {
		self.location == constants::LOCATION_ID_GRAVEYARD
	}

	pub fn contains_item(&self, id: ItemId) -> bool {
		match self.within.clone() {
			None => false,
			Some(within) => within.borrow().is_or_contains_item(id),
		}
	}

	pub fn is_or_contains_item(&self, id: ItemId) -> bool {
		if self.id == id {
			return true;
		}
		self.contains_item(id)
	}

	pub fn get_location(&self) -> Id {
		self.location
	}

	pub fn set_location(&mut self, loc: Id) {
		self.location = loc;
	}

	pub fn has_property(&self, property_code: ItemProperties) -> bool {
		self.properties & property_code != 0
	}

	// If second parameter is set, then the item must be switched on in order for the property to be active
	pub fn has_or_contains_with_property(&self, property_code: ItemProperties, on_optional: bool) -> bool {
		if (on_optional || self.on) && self.has_property(property_code) {
			return true;
		}
		match self.within.clone() {
			None => return false,
			Some (within) => return within.borrow().has_or_contains_with_property(property_code, on_optional),
		}
	}

	pub fn set_property(&mut self, property_code: ItemProperties, next: bool) {
		if next {
			self.properties |= property_code;
		} else {
			self.properties &= !property_code;
		}
	}

	pub fn get_id(&self) -> ItemId {
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
	pub fn has_problem_accepting(&self, item: &ItemRef) -> Option<StringId> {
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

	// Check that an item can be emptied
	// If there is a problem, return the string tag of the reason, otherwise return None
	#[allow(unused_variables)]
	pub fn has_problem_emptying(&self, other: &ItemRef) -> Option<StringId> {
		if !self.has_property(constants::CTRL_ITEM_CONTAINER) {
			return Some(constants::STR_ID_NOT_CONTAINER);
		}
		None
	}

	// Check that an item can be inserted
	// If there is a problem, return the string tag of the reason, otherwise return None
	#[allow(unused_variables)]
	pub fn has_problem_inserting(&self, other: &ItemRef) -> Option<StringId> {
		if self.has_property(constants::CTRL_ITEM_WEARABLE) {
			return Some(constants::STR_ID_CANNOT_INSERT_WEARABLE);
		}
		if !self.has_property(constants::CTRL_ITEM_MOBILE) {
			return Some(constants::STR_ID_CANNOT_TAKE);
		}
		None
	}

	fn get_description_ender(&self) -> &str {
		if self.has_property(constants::CTRL_ITEM_OBSTRUCTION) || self.has_property(constants::CTRL_ITEM_TREASURE) {STR_EXCLAMATION} else {STR_DOT}
	}

	fn get_switch_status(&self) -> &str {
		if self.on {STR_ON} else {STR_OFF}
	}

	fn get_switch_status_short(&self) -> String {
		String::from(STR_SWITCH_SHORT).replace("$0", self.get_switch_status())
	}

	fn get_switch_status_long(&self) -> String {
		String::from(STR_SWITCH_LONG).replace("$0", self.get_switch_status())
	}

	fn get_within_status_short(&self, nest: bool, depth: u32) -> String {
		let mut result = String::new();
		if self.has_property(constants::CTRL_ITEM_CONTAINER) {
			match self.within.clone() {
				None => return String::from(STR_EMPTY_SHORT),
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
					result = result + &pre + STR_CONTAINS_SHORT + &contained.borrow().get_longname() + &contained.borrow().get_within_status_short(nest_next, depth + 1) + &post;
				},
			}
		}
		result
	}

	fn get_within_status_long(&self) -> String {
		match self.within.clone() {
			None => String::from(STR_EMPTY_LONG),
			Some(contained) => String::from(STR_CONTAINS_LONG) + &contained.borrow().get_longname() + &contained.borrow().get_within_status_short(false, 1),
		}
	}

	// Return the name of this item as it would be displayed in an inventory listing
	pub fn get_inventoryname(&self) -> String {

		let mut result: String = String::new();
		if self.has_property(constants::CTRL_ITEM_WEARABLE) {
			result = result + STR_WEARING;
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
			result = result + &self.longname;
			if self.has_property(constants::CTRL_ITEM_SWITCHABLE) {
				result = result + &self.get_switch_status_short();
			}
			result = result + &self.get_within_status_short(false, 1);
			result = String::from(STR_THERE_IS).replace("$0", &result);
			result = result + self.get_description_ender();
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

	pub fn remove_item_certain(&mut self, id: ItemId) {
		let within = self.within.clone().expect("Data corruption seeking item contained within item, fail.");
		let is_item = within.borrow().is(id);
		if is_item {
			let is_liquid = within.borrow().has_property(constants::CTRL_ITEM_LIQUID);
			if !is_liquid {
				within.borrow_mut().retire();
			}
			self.within = None;
		} else {
			within.borrow_mut().remove_item_certain(id);
		}
	}

	pub fn is_empty(&self) -> bool {
		match self.within {
			None => true,
			_ => false,
		}
	}

	pub fn set_within(&mut self, within: Option<ItemRef>) {
		if let Some(with) = within.clone() {
			with.borrow_mut().set_location(self.id);
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
