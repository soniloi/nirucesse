use std::collections::HashMap;

use data_collection;
use file_buffer::FileBuffer;

const FILE_INDEX_STRING_TAG: usize = 0;
const FILE_INDEX_STRING_CONTENT: usize = 1;
const FILE_INDEX_STRING_PUZZLE: usize = 2;
const SEP_SECTION: &'static str = "---"; // String separating sections

pub struct StringCollection {
	strings: HashMap<String, String>,
}

impl StringCollection {

	pub fn new() -> StringCollection {
		StringCollection {
			strings: HashMap::new(),
		}
	}

	pub fn init(&mut self, buffer: &mut FileBuffer, mut achievement_count: &mut u32) {

		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => return,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					let string_parsed = StringCollection::parse_string(&words, &mut achievement_count);
					self.strings.insert(string_parsed.0, string_parsed.1);
				},
			}
			line = buffer.get_line();
		}
	}

	fn parse_string(words: &Vec<&str>, achievement_count: &mut u32) -> (String, String) {
		let tag = words[FILE_INDEX_STRING_TAG];
		let content = words[FILE_INDEX_STRING_CONTENT];
		*achievement_count = *achievement_count + data_collection::str_to_u32(words[FILE_INDEX_STRING_PUZZLE], 10);
		return (String::from(tag), String::from(content))
	}

	// Return a String Option
	pub fn get_uncertain(&self, key: &str) -> Option<&String> {
		self.strings.get(&String::from(key))
	}

	// Return a String we are certain is in the collection
	pub fn get_certain(&self, key: &str) -> &String {
		match self.strings.get(&String::from(key)) {
			None => panic!("Error: Data collection corrupt, or key [{}] malformed.", key),
			Some(st) => return st,
		}
	}
}
