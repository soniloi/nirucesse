use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Read in compressed-format datafile
pub fn read_compressed(filename: &str) -> Vec<u8> {
	let path = Path::new(filename);
	let display = path.display();

	let mut file = match File::open(&path) {
		Err(why) => panic!("Unable to open {}: {}", display, why.description()),
		Ok(file) => file,
	};

	// Open and read file
	let mut contents: Vec<u8> = Vec::new();
	file.read_to_end(&mut contents).unwrap();

	contents
}

// Decompress byte vector into readable char vector
pub fn decompress(compressed: &Vec<u8>) -> Vec<char> {
	let mut expanded: Vec<char> = Vec::with_capacity(compressed.len()/7*8);
	let mut i = 0;
	while i < compressed.len()-1 {
		decompress_chunk(&compressed[i..i+7], &mut expanded);
		i += 7;
	}
	expanded
}

// Decompress a 7-byte chunk into 8 bytes, adding them to a vector
fn decompress_chunk(compressed: &[u8], expanded: &mut Vec<char>){
	expanded.push(((compressed[0] >> 1) & 0x7f) as char);
	for i in 1..7 {
		let ch = (((compressed[i] as u8) >> (i+1)) | (((compressed[i-1]) & ((1<<i)-1)) << (7-i))) & 0x7f;
		expanded.push(ch as char);
	}
	expanded.push((compressed[6] & 0x7f) as char);
}
