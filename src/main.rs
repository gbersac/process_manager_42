extern crate regex;

mod project;
mod parse;
mod error;

use std::env;
use std::fs::File;
use std::io::Read;
use parse::fc_string;
use error::{KrpSimError};
use project::{Project};

fn file_as_string(file_name: &String) -> String {
    let mut f = File::open(file_name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    s
}

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
	let instructions_str = file_as_string(&file_name);
	match parse::Parser::parse(&instructions_str) {
	    Ok((ressources, optimize, processes)) => {
	    	// launch process resolution
	    	let project = Project::new(ressources, processes, optimize);
	    },
	    Err(e) => {
	    	match e {
	    	    KrpSimError::ParseError(line) => {
					let line_str =
							fc_string::get_line(&instructions_str, line - 1).unwrap();
			    	println!("Error parsing file {} on line {}:\n{}",
			    			file_name, line, line_str);
	    	    },
	    	}
	    },
	}
}
