use std::collections::HashMap;

use file_buffer::FileBuffer;

const FILE_INDEX_STRING_TAG: usize = 0;
const FILE_INDEX_STRING_CONTENT: usize = 1;
const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct HelpStringCollection {
	strings: HashMap<String, String>,
}

impl HelpStringCollection {

	pub fn new() -> HelpStringCollection {
		HelpStringCollection {
			strings: HashMap::new(),
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer) {

		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => break,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					let string_parsed = HelpStringCollection::parse_string(&words);
					self.strings.insert(string_parsed.0, string_parsed.1);
				},
			}
			line = buffer.get_line();
		}

		self.validate();
	}

	fn parse_string(words: &Vec<&str>) -> (String, String) {
		let tag = words[FILE_INDEX_STRING_TAG];
		let content = words[FILE_INDEX_STRING_CONTENT];
		return (String::from(tag), String::from(content))
	}

	fn validate(&self) {
		if !self.strings.contains_key("default") {
			panic!("Error in help string collection. Key [default] not found.");
		}
	}

	// Return a String Option
	pub fn get_uncertain(&self, key: &str) -> Option<&String> {
		self.strings.get(&String::from(key))
	}

	// Return a String we are certain is in the collection
	pub fn get_certain(&self, key: &str) -> &str {
		match self.strings.get(&String::from(key)) {
			None => panic!("Error: Data collection corrupt, or key [{}] malformed.", key),
			Some(st) => return st,
		}
	}
}
