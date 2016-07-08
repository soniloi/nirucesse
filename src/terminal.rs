use std::cmp;
use std::io;
use std::io::stdout;
use std::io::Write;

const COLOUR_IN: &'static str = "\x1b[0m";
const COLOUR_OUT: &'static str = "\x1b[32m";
const CONSOLE_RESET: &'static str = "\x1b[0m";
const CONSOLE_WIDTH: usize = 80;
const PROMPT_END: &'static str = " > ";
const PROMPT_FULL: &'static str = "---------> ";
const PROMPT_TAB: &'static str = "         > ";
const PROMPT_WIDTH: usize = 11;
const PROMPT_EFFECTIVE_WIDTH: usize = 8;
const CONSOLE_EFFECTIVE_WIDTH: usize = CONSOLE_WIDTH - PROMPT_WIDTH;
const MAX_TOKENS: u32 = 2;

pub fn write_full(st: &str) {
	let raw: Vec<char> = st.chars().collect();
	write_sections(raw, 0, PROMPT_FULL);
}

// Write the next section to the terminal; a section is the characters from start_index up to
// the next newline (if present) or the last whitespace character before the effective console
// width is reached; returns only when a section shorter than the effective width has been
// printed
fn write_sections(chars: Vec<char>, start_index: usize, prompt: &str) {

	let remaining = chars.len() - start_index;
	let max_index = start_index + if CONSOLE_EFFECTIVE_WIDTH > remaining {remaining} else {CONSOLE_EFFECTIVE_WIDTH};
	//let max_index = start_index + cmp::min(remaining, CONSOLE_EFFECTIVE_WIDTH);

	let mut stop_index: i32 = -1;

	// If there is a newline within range, print up to that
	let newline_index = get_newline_index_within_width(&chars[(start_index as usize)..max_index]);
	if newline_index != -1 {
		stop_index = start_index as i32 + newline_index;
		write_content(&chars, start_index, stop_index as usize, prompt);
		write_sections(chars, stop_index as usize + 1, PROMPT_TAB);

	// If the remaining width is less than the console width, print and return
	} else if remaining <= CONSOLE_EFFECTIVE_WIDTH {
		write_content(&chars, start_index, chars.len(), prompt);
		return;

	} else {
		// Write up until the last available space character in the string, if existing
		let space_index = get_last_space_index_within_width(&chars[(start_index as usize)..max_index]);
		if space_index != -1 {
			stop_index = start_index as i32 + space_index;
			write_content(&chars, start_index, stop_index as usize, prompt);
			write_sections(chars, stop_index as usize + 1, PROMPT_TAB);

		// This string is a lost cause, so just dump out whatever is left
		} else {
			write_content(&chars, start_index, chars.len(), prompt);
			write_sections(chars, stop_index as usize + 1, PROMPT_TAB);
		}
	}
}

// Write some content from a character slice
fn write_content(chars: &Vec<char>, start_index: usize, stop_index: usize, prompt: &str) {
	let content: String = to_str(&chars[start_index..stop_index]);
	write_prompted(&content, prompt);
}

// Create a prompt based on a short word and read from stdin
pub fn read_stub(stubname: &str) -> Vec<String> {
	let mut prompt: String = stubname.to_string();
	for i in stubname.len()..PROMPT_EFFECTIVE_WIDTH {
		prompt.push(' ');
	}

	read_prompted(&(prompt + PROMPT_END))
}

// Write a prompt and read tokens from stdin
// Return only the first MAX_TOKENS tokens of input
fn read_prompted(prompt: &str) -> Vec<String> {
	let mut result_raw = String::new();
	write(prompt);
	io::stdin().read_line(&mut result_raw);
	let mut result_iter = result_raw.trim().split(" ");

	let mut result_vec: Vec<String> = vec![];
	for i in 0..MAX_TOKENS {
		match result_iter.next() {
			Some(st) => {result_vec.push(st.to_lowercase()); },
			None => break,
		}
	}

	result_vec
}

pub fn reset() {
	print!("{}", CONSOLE_RESET);
}

// Write to console with a given prompt
fn write_prompted(st: &str, prompt: &str) {
	let mut prompted: String = String::with_capacity(CONSOLE_WIDTH);
	prompted.push_str(prompt);
	prompted.push_str(st);
	write_line(&prompted);
}

fn write(st: &str) {
	print!("{}{}{}", COLOUR_OUT, st, COLOUR_IN);
	stdout().flush();
}

fn write_line(st: &str) {
	println!("{}{}{}", COLOUR_OUT, st, COLOUR_IN);
	stdout().flush();
}

// Return the index of the first newline within a string slice
// If no newline character found, return -1
fn get_newline_index_within_width(chs: &[char]) -> i32 {
	let mut i: usize = 0;
	while i < chs.len() {
		if chs[i] == '\n' {
			return i as i32;
		}
		i += 1;
	}
	-1
}

// Return the index of the last space within a string slice
// If no space character found, return -1
fn get_last_space_index_within_width(chs: &[char]) -> i32 {
	let mut i: i32 = chs.len() as i32 - 1;
	while i >= 0 {
		if chs[i as usize] == ' ' {
			return i;
		}
		i -= 1;
	}
	-1
}

fn to_str(chs: &[char]) -> String {
	let mut result: String = String::with_capacity(chs.len());
	for ch in chs {
		result.push(*ch);
	}
	result
}
