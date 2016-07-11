use file_util;

pub struct FileBuffer {
	data: Vec<char>,
	index: usize,
}

impl FileBuffer {

	pub fn new(filename: &str) -> FileBuffer {
		let raw = file_util::read_compressed(filename);
		FileBuffer {
			data: file_util::decompress(&raw),
			index: 0,
		}
	}

	pub fn eof(&self) -> bool {
		self.index >= self.data.len()
	}

	/*
	 * Seek ahead in the buffer until a newline is found, returning the
	 *   characters between the previous index and the newline and incrementing
	 *   the index pointer to point at the next non-newline character
	*/
	pub fn get_line(&mut self) -> String {
		let mut result = String::new();
		while !self.eof() && self.data[self.index] != '\n' {
			result.push(self.data[self.index]);
			self.index += 1;
		}
		while !self.eof() && self.data[self.index] == '\n' {
			self.index += 1;
		}

		result
	}
}
