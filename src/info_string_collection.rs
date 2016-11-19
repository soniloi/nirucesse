use std::collections::HashMap;

use constants;
use data_collection::{self, StringId};
use file_buffer::FileBuffer;

const FILE_INDEX_STRING_TAG: usize = 0;
const FILE_INDEX_STRING_CONTENT: usize = 1;

pub struct InfoStringCollection {
	strings: HashMap<StringId, String>,
}

impl InfoStringCollection {

	pub fn new() -> InfoStringCollection {
		InfoStringCollection {
			strings: HashMap::new(),
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer, expected_count: u32, validate: bool) {

		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				constants::FILE_SECTION_SEPARATOR => break,
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
			self.validate(expected_count);
		}
	}

	fn parse_string(words: &Vec<&str>) -> (StringId, String) {
		let tag = data_collection::str_to_u32(words[FILE_INDEX_STRING_TAG], 10);
		let content = words[FILE_INDEX_STRING_CONTENT];
		return (tag, String::from(content))
	}

	// Ensure that all the necessary ids will be available
	fn validate(&self, expected_count: u32) {
		if self.strings.len() as u32 != expected_count {
			panic!("Error in string collection. Expected [{}] tags, found [{}]", expected_count, self.strings.len());
		}
		for id in 0..expected_count {
			if !self.strings.contains_key(&id) {
				panic!("Error in string collection. ID [{}] not found", id);
			}
		}
	}

	pub fn count_strings(&self) -> u32 {
		self.strings.len() as u32
	}

	pub fn get_keys(&self) -> Vec<StringId> {
		let mut result: Vec<StringId> = Vec::new();
		for key in self.strings.keys() {
			result.push(*key);
		}
		result
	}

	// Return a String we are certain is in the collection
	pub fn get_certain(&self, key: StringId) -> &str {
		match self.strings.get(&key) {
			None => panic!("Error: Data collection corrupt, or key [{}] malformed.", key),
			Some(st) => return st,
		}
	}
}
