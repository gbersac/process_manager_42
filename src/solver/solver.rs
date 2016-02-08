use project::Project;

#[derive(Debug)]
pub struct Solver {
	/// The root of the tree of possibility
	root: Vec<Node>,
}

impl Solver {
	pub fn new(project: &Project) -> Solver {
		Solver {
			root: Vec::new(),
		}
	}
}
