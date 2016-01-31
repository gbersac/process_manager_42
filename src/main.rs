extern crate regex;

mod project;
mod parse;
mod fn_string;
mod error;

use std::env;
use project::{Project};

fn main() {
	// parse args
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 {
	    println!("usage: {} <file> <delay>", args[0]);
	     std::process::exit(0);
	}
	let file_name = args[1].clone();
	let delay = args[2].clone();

	// parse instructions file
	let project = Project::project_from_file(&file_name);
}
