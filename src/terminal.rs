const COLOUR_IN: &'static str = "\x1b[0m";
const COLOUR_OUT: &'static str = "\x1b[32m";
const CONSOLE_RESET: &'static str = "\x1b[0m";
const CONSOLE_WIDTH: usize = 80;
const PROMPT_END: &'static str = " > ";
const PROMPT_FULL: &'static str = "---------> ";
const PROMPT_TAB: &'static str = "         > ";
const PROMPT_WIDTH: usize = 11;

pub fn write_full(st: &str) {
	let raw: Vec<char> = st.chars().collect();
	let bound = CONSOLE_WIDTH - PROMPT_WIDTH;
	if raw.len() < bound {
		write_prompted(st, PROMPT_FULL);
		return;
	}

	let mut start_index: i32 = 0;
	let mut stop_index: i32 = get_newline_index_within_width(&raw[(start_index as usize)..bound]);
	if stop_index == -1 {
		stop_index = get_last_space_index_within_width(&raw[(start_index as usize)..bound]);
	}
	if stop_index == -1 {
		stop_index = raw.len() as i32;
	}

	let mut content: String = to_str(&raw[0..(stop_index as usize)]);
	write_prompted(&content, PROMPT_FULL);

	start_index = stop_index + 1;
	content = to_str(&raw[(start_index as usize)..]);
	write_prompted(&content, PROMPT_TAB);
}

// Write to console using a 'tabbed' (empty) prompt
// TODO: this is public for now, but possibly should be made private later, or nemoved entirely
pub fn write_tabbed(st: &str) {
	write_prompted(st, PROMPT_TAB);
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
	let mut i: usize = chs.len() - 1;
	while i >= 0 {
		if chs[i] == ' ' {
			return i as i32;
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
