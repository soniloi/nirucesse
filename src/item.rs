const CTRL_ITEM_CONTAINER: u32 = 0x1;  // Whether the item may contain other items
const CTRL_ITEM_MOBILE: u32 = 0x2; // Whether the item is fixed or mobile (carryable)
const CTRL_ITEM_OBSTRUCTION: u32 = 0x4; // Whether the item is an obstruction
const CTRL_ITEM_SWITCHABLE: u32 = 0x8; // Whether the item can be lit/quenched
const CTRL_ITEM_GIVES_LIGHT: u32 = 0x10; // Whether the item emits light
const CTRL_ITEM_GIVES_AIR: u32 = 0x20; // Whether the item enables player to breathe
const CTRL_ITEM_GIVES_GRAVITY: u32 = 0x40; // Whether the item holds the player down
const CTRL_ITEM_CONTAINER_LIQUID: u32 = 0x100; // Whether the container may contain liquids rather than solids
const CTRL_ITEM_FRAGILE: u32 = 0x200; // Whether the item would survive throwing, dropping from heights, etc
const CTRL_ITEM_WEARABLE: u32 = 0x400; // Whether the item is to be worn by the player rather than carried
const CTRL_ITEM_LIQUID: u32 = 0x800; // Whether item is a liquid, i.e. needs a special container to carry it
const CTRL_ITEM_ESSENTIAL: u32 = 0x1000; // Whether the item is essential to basic gameplay
const CTRL_ITEM_EDIBLE: u32 = 0x2000; // Whether the item is any sort of food or drink
const CTRL_ITEM_TREASURE: u32 = 0x8000; // Whether the item is a treasure
const CTRL_ITEM_SILENT: u32 = 0x20000; // Whether the item should be shown in location descriptions
const CTRL_ITEM_RECIPIENT: u32 = 0x80000; // Whether the item may be a recipient (i.e. of gifts or food)

use data_collection::ItemRef;

pub struct Item {
	id: u32,
	properties: u32,
	size: u32,
	shortname: String,
	longname: String,
	description: String,
	writing: Option<String>,
	on: bool,
	within: Option<ItemRef>,
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
			on: false,
			within: None,
		}
	}

	pub fn is(&self, id: u32) -> bool {
		id == self.id
	}

	// FIXME: probably refactor this out
	pub fn contains_item(&self, id: u32) -> bool {
		match self.within.clone() {
			None => false,
			Some(within) => within.borrow().is(id) || within.borrow().contains_item(id),
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

	fn has_property(&self, property: u32) -> bool {
		self.properties & property != 0
	}

	fn set_property(&mut self, property: u32) {
		self.properties |= property;
	}

	fn unset_property(&mut self, property: u32) {
		self.properties &= !property;
	}

	fn is_mobile(&self) -> bool {
		self.has_property(CTRL_ITEM_MOBILE)
	}

	pub fn has_light(&self) -> bool {
		if self.has_property(CTRL_ITEM_GIVES_LIGHT) && self.on {
			return true;
		}
		match self.within.clone() {
			None => return false,
			Some (within) => return within.borrow().has_light(),
		}
	}

	pub fn has_air(&self) -> bool {
		self.has_property(CTRL_ITEM_GIVES_AIR)
	}

	pub fn has_gravity(&self) -> bool {
		self.has_property(CTRL_ITEM_GIVES_GRAVITY)
	}

	pub fn is_obstruction(&self) -> bool {
		self.has_property(CTRL_ITEM_OBSTRUCTION)
	}

	pub fn set_obstruction(&mut self, on: bool) {
		if on {
			self.set_property(CTRL_ITEM_OBSTRUCTION);
		} else {
			self.unset_property(CTRL_ITEM_OBSTRUCTION);
		}
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

	pub fn is_switchable(&self) -> bool {
		self.has_property(CTRL_ITEM_SWITCHABLE)
	}

	pub fn is_edible(&self) -> bool {
		self.has_property(CTRL_ITEM_EDIBLE)
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
		self.is_mobile() && !self.is_obstruction()
	}

	pub fn is_recipient(&self) -> bool {
		self.has_property(CTRL_ITEM_RECIPIENT)
	}

	fn is_treasure(&self) -> bool {
		self.has_property(CTRL_ITEM_TREASURE)
	}

	pub fn is_container(&self) -> bool {
		self.has_property(CTRL_ITEM_CONTAINER)
	}

	pub fn is_container_liquid(&self) -> bool {
		self.has_property(CTRL_ITEM_CONTAINER_LIQUID)
	}

	fn is_silent(&self) -> bool {
		self.has_property(CTRL_ITEM_SILENT)
	}

	pub fn is_liquid(&self) -> bool {
		self.has_property(CTRL_ITEM_LIQUID)
	}

	pub fn get_treasure_value(&self) -> u32 {
		self.count_treasure_value(0)
	}

	fn count_treasure_value(&self, acc: u32) -> u32 {
		let mut result = acc;
		if self.is_treasure() {
			result += 1;
		}
		match self.within.clone() {
			None => result,
			Some(within) => within.borrow().count_treasure_value(result)
		}
	}

	// Return whether an item could fit inside this item, assuming it is a container
	pub fn can_accept(&self, item: &ItemRef) -> bool {
		item.borrow().get_capacity() < self.size
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
		if self.is_container() {
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
		if self.is_wearable() {
			result = result + "(wearing) ";
		}
		result = result + &self.longname;
		if self.is_switchable() {
			result = result + &self.get_switch_status_short();
		}
		result + &self.get_within_status_short(true, 1)
	}

	// Return the name of this item as it would be displayed in a location listing
	pub fn get_locationname(&self) -> String {
		let mut result = String::new();
		if !self.is_silent() {
			result = result + "\nThere is " + &self.longname;
			if self.is_switchable() {
				result = result + &self.get_switch_status_short();
			}
			result = result + &self.get_within_status_short(false, 1) + " here";
			result = result + if self.is_obstruction() || self.is_treasure() {"!"} else {"."};
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

	pub fn remove_item_certain(&mut self, id: u32) -> ItemRef {
		match self.within.clone() {
			None => panic!("Data corruption seeking item [{}], fail.", id),
			Some(within) => {
				if within.borrow().is(id) {
					self.within = None;
					return within;
				}
				return within.borrow_mut().remove_item_certain(id);
			},
		}
	}

	pub fn is_empty(&self) -> bool {
		match self.within {
			None => true,
			_ => false,
		}
	}

	pub fn get_within(&self) -> Option<ItemRef> {
		self.within.clone()
	}

	pub fn set_within(&mut self, within: Option<ItemRef>) {
		self.within = within;
	}

	pub fn remove_within(&mut self) -> Option<ItemRef> {
		let within = self.within.clone();
		self.within = None;
		within
	}

	pub fn mk_full_string(&self, description_start: &str, description_end: &str) -> String {
		let mut result = String::from(description_start) + &self.description;
		if self.is_switchable() {
			result = result + &self.get_switch_status_long();
		}
		if self.is_container() {
			result = result + &self.get_within_status_long();
		}
		result + description_end
	}

	pub fn mk_writing_string(&self, no_writing: &str, writing_start: &str, writing_end: &str) -> String {
		match self.writing.clone() {
			None => String::from(no_writing),
			Some(writ) => String::from(writing_start) + &writ + writing_end,
		}
	}
}
