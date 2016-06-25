const COLOUR_IN: &'static str = "\x1b[0m";
const COLOUR_OUT: &'static str = "\x1b[32m";
const CONSOLE_RESET: &'static str = "\x1b[0m";
const CONSOLE_WIDTH: usize = 80;
const PROMPT_END: &'static str = " > ";
const PROMPT_FULL: &'static str = "---------> ";
const PROMPT_TAB: &'static str = "         > ";
const PROMPT_WIDTH: usize = 11;
const CONSOLE_EFFECTIVE_WIDTH: usize = CONSOLE_WIDTH - PROMPT_WIDTH;

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
	if remaining < CONSOLE_EFFECTIVE_WIDTH {
		let content: String = to_str(&chars[(start_index as usize)..]);
		write_prompted(&content, prompt);
		return;
	}

	let max_index = start_index + CONSOLE_EFFECTIVE_WIDTH;

	let mut stop_index: i32 = get_newline_index_within_width(&chars[(start_index as usize)..max_index]);
	if stop_index == -1 {
		stop_index = start_index as i32 + get_last_space_index_within_width(&chars[(start_index as usize)..max_index]);
	}
	if stop_index == -1 {
		stop_index = chars.len() as i32;
	}
	let content: String = to_str(&chars[start_index..(stop_index as usize)]);
	write_prompted(&content, prompt);

	write_sections(chars, stop_index as usize + 1, PROMPT_TAB);
}

// Write to console with a given prompt
fn write_prompted(st: &str, prompt: &str) {
	let mut prompted: String = String::with_capacity(CONSOLE_WIDTH);
	prompted.push_str(prompt);
	prompted.push_str(st);
	write_line(&prompted);
}

fn write_line(st: &str) {
	println!("{}{}", COLOUR_OUT, st);
}

pub fn reset() {
	print!("{}", CONSOLE_RESET);
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
