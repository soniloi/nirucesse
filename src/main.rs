mod file_util;

use std::env;
use std::process;

fn main() {

	// Get command-line args
	let args: Vec<_> = env::args().collect();
	if args.len() < 2 {
		println!("Filename parameter missing, fail.");
		process::exit(1);
	}
    let filename = &args[1];

    // Read and decompress data file
    let raw = file_util::read_compressed(filename);
    let decompressed = file_util::decompress(&raw);

    // Test print
	let str_contents: Vec<String> = to_str_arr(decompressed);
	for str in str_contents {
		print!("{}\n", str);
	}
}

// Test converter
fn to_str_arr(contents: Vec<char>) -> Vec<String> {

	let mut strs: Vec<String> = vec![];

	let mut current_str: String = String::from("");
	for ch in contents {
		if ch == '\n' {
			strs.push(current_str);
			current_str = String::from("");
		} else {
			current_str.push(ch);
		}
	}

	strs
}
