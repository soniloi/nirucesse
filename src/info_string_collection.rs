use std::collections::HashMap;

use data_collection;
use file_buffer::FileBuffer;

const FILE_INDEX_STRING_TAG: usize = 0;
const FILE_INDEX_STRING_CONTENT: usize = 1;
const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct InfoStringCollection {
	strings: HashMap<u32, String>,
}

impl InfoStringCollection {

	pub fn new() -> InfoStringCollection {
		InfoStringCollection {
			strings: HashMap::new(),
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer, expected_max: u32, validate: bool) {

		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => break,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					let string_parsed = InfoStringCollection::parse_string(&words);
					self.strings.insert(string_parsed.0, string_parsed.1);
				},
			}
			line = buffer.get_line();
		}

		if validate {
			self.validate(expected_max);
		}
	}

	fn parse_string(words: &Vec<&str>) -> (u32, String) {
		let tag = data_collection::str_to_u32(words[FILE_INDEX_STRING_TAG], 10);
		let content = words[FILE_INDEX_STRING_CONTENT];
		return (tag, String::from(content))
	}

	// Ensure that all the necessary ids will be available
	fn validate(&self, expected_max: u32) {
		for id in 0..expected_max {
			if !self.strings.contains_key(&id) {
				panic!("Error in string collection. ID [{}] not found", id);
			}
		}
	}

	pub fn count_strings(&self) -> u32 {
		self.strings.len() as u32
	}

	pub fn get_keys(&self) -> Vec<u32> {
		let mut result: Vec<u32> = Vec::new();
		for key in self.strings.keys() {
			result.push(*key);
		}
		result
	}

	// Return a String we are certain is in the collection
	pub fn get_certain(&self, key: u32) -> &str {
		match self.strings.get(&key) {
			None => panic!("Error: Data collection corrupt, or key [{}] malformed.", key),
			Some(st) => return st,
		}
	}
}
