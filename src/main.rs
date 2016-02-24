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
use solver::{Node, NodePtr};

fn usage(arg0: String) {
    println!("usage: {} <file> <delay>", arg0);
    std::process::exit(0);
}

fn print_solution(solution: Option<NodePtr>) {
	match solution {
	    Some(node) => {
	    	println!("{}", node);
	    	print_solution(node.get_child());
	    },
	    None => {},
	}
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
			let project = Rc::new(Project::from_file(&file_name, delay));
			let (weight, solution) = solver::solve(project.clone());
			print_solution(Some(solution));
	    },
	    Err(_) => usage(args[0].clone()),
	}
}
