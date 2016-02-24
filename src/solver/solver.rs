use project::Project;

#[derive(Debug)]
pub struct Solver {
	/// The root of the tree of possibility
	root: Vec<Node>,

	/// Number of turn for which to run the project
	delay: usize
}


impl Solver {
	pub fn solve(project: &Project) {
		let mut solver = Solver {
			root: Vec::new(),
			delay: delay
		}

	}
}
