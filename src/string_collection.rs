use std::collections::HashMap;

use file_buffer::FileBuffer;

const FILE_INDEX_STRING_TAG: usize = 0;
const FILE_INDEX_STRING_CONTENT: usize = 1;
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

	pub fn init(&mut self, buffer: &mut FileBuffer) {

		let mut line = buffer.get_line();
	    while !buffer.eof() {
			match line.as_ref() {
				SEP_SECTION => return,
				x => {

					let words_split = x.split("\t");
					let words: Vec<&str> = words_split.collect();

					let tag = words[FILE_INDEX_STRING_TAG];
					let content = words[FILE_INDEX_STRING_CONTENT];
					self.strings.insert(String::from(tag), String::from(content));
				},
			}
			line = buffer.get_line();
		}
	}

	pub fn get(&self, key: String) -> Option<&String> {
		self.strings.get(&key)
	}

}
