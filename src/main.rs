extern crate regex;

mod project;
mod parse;
mod fn_string;
mod error;
mod matrix;
#[cfg(test)]
mod test_matrix;
mod solver;

use std::env;
use project::{Project};
use std::rc::Rc;

fn usage(arg0: String) {
    println!("usage: {} <file> <delay>", arg0);
    std::process::exit(0);
}

fn main() {
	// parse args
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 {
		usage(args[0].clone());
	}
	let file_name = args[1].clone();
	match args[2].parse::<usize>() {
	    Ok(delay) => {
			let project = Rc::new(Project::project_from_file(&file_name, delay));
			solver::solve(project.clone());
			println!("{:?}", project);
	    },
	    Err(_) => usage(args[0].clone()),
	}
}
