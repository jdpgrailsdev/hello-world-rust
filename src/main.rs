use ferris_says::say;
use std::io::{stdout, BufWriter};

pub mod time_utils;

fn main() {
//    println!("Hello, world!");
	let string1 = "Hello, world! Today is ".to_string();
	let string2 = string1 + & time_utils::current_date(); 
	let stdout = stdout();
	let message = String::from(string2);
	let width = message.chars().count();
	
	let mut writer = BufWriter::new(stdout.lock());
	say(message.as_bytes(), width, &mut writer).unwrap();	
}