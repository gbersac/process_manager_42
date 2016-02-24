mod node;
mod end_process_stack;
mod resource_list;

use std::rc::Rc;
use project::{Project};
pub use self::node::{Node};

pub fn solve(project: Rc<Project>) -> Vec<Node> {
	unimplemented!()
}
