mod node;
mod end_process_stack;
mod resource_list;
#[cfg(test)]
mod test_solver;
#[cfg(test)]
mod test_file;

use std::rc::Rc;
use project::Project;
pub use self::node::{Node, NodePtr};

pub fn solve(project: Rc<Project>) -> (i32, NodePtr) {
    Node::launch_node_tree(project)
}
