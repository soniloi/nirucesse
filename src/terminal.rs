use std::io;
use std::io::Write;

const COLOUR_IN: &'static str = "\x1b[0m";
const COLOUR_OUT: &'static str = "\x1b[32m";
const CONSOLE_RESET: &'static str = "\x1b[0m";
const CONSOLE_WIDTH: usize = 80;
const PROMPT_END: &'static str = " > ";
const PROMPT_PRO: &'static str = "---------> ";
const PROMPT_TAB: &'static str = "         > ";

fn write_line(st: &str) {
	println!("{}{}", COLOUR_OUT, st);
}

// Write to console in a 'tabbed' format
// TODO: this is public for now, but possibly should be made private later
pub fn write_tabbed(st: &str) {
	let mut prompted: String = String::with_capacity(CONSOLE_WIDTH);
	prompted.push_str(PROMPT_TAB);
	prompted.push_str(st);
	write_line(&prompted);
}

pub fn reset() {
	print!("{}", CONSOLE_RESET);
}