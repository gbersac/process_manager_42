use std::rc::Rc;
use project::{Project, ProjectPtr};
use solver::end_process_stack::{EndProcessStack};
use solver::resource_list::{ResourceList};

pub type NodePtr = Rc<Node>;

/// A node is the execution of one process at a moment in time.
#[derive(Debug, Clone)]
pub struct Node {
	/// The turn this Node refer to
	time: usize,

	/// Parent node
	child: Option<NodePtr>,

	/// Available resources
	resources: ResourceList,

	/// Keep track of when the launched processes end.
	processes_to_end: EndProcessStack
}

impl Node {

	/// Return a list of process ready to be launch.
	fn processes_ready(&self,
		project: ProjectPtr,
	) -> Vec<(usize, usize)> {
		let mut to_return = Vec::new();
		for i_process in 0..project.nb_process() {
			let nb_process = project.can_trigger_process(i_process,
					self.resources.get_list());
		    if nb_process > 0 {
		    	to_return.push((i_process, nb_process));
		    }
		}
		to_return
	}

	/// Return the best child of all the child created after they launched
	/// a process in `processes_ready`
	fn child_from_process(&self,
		project: ProjectPtr,
		processes_ready: Vec<(usize, usize)>
	) -> (i32, Node) {
		println!("child_from_process");
		processes_ready.iter().map(|&(i_process, nb_process)| {
			println!("child_from_process inner {}", i_process);
			let resources = self.resources
					.launch_process(project.clone(), i_process, nb_process);
			let processes_to_end = self.processes_to_end
					.add_processes(project.clone(), i_process, nb_process);
			Node::new(project.clone(), self.time, resources, processes_to_end)
		}).max_by_key(|&(weight, _)| weight).unwrap()
	}

	/// Return the best child after a new turn has been passed
	fn child_from_new_turn(&self, project: ProjectPtr) -> (i32, Node) {
		let mut new_time = self.time;
		let mut processes_to_end = self.processes_to_end.clone();
		loop {
			new_time += 1;

			// check for end of simulation
			if new_time >= project.get_delay() {
			    return (self.compute_weight(project), self.clone());
			}
			println!("processes_to_end {:?}", processes_to_end);

			// check if something happen in the turn
			if !processes_to_end.process_terminate_at_next_turn() {
			    processes_to_end.pop_one_turn();
			    continue ;
			}

			// at least one process terminate, create new node
			let mut resource = processes_to_end
					.pop_and_terminate(project.clone(), &self.resources);
			println!("pop_and_terminate {:?}", resource);
			return Node::new(project.clone(), new_time, resource,
					processes_to_end);
		}
	}

	pub fn new(
		project: ProjectPtr,
		time: usize,
		resources: ResourceList,
		processes_to_end: EndProcessStack
	) -> (i32, Node) {
		let mut new_node = Node {
			time: time,
			child: None,
			resources: resources,
			processes_to_end: processes_to_end
		};
		println!("new {:?}", new_node);

		// check for end of simulation
		if new_node.time >= project.get_delay() {
		    return (new_node.compute_weight(project), new_node);
		}

		// create all the possible child and select the best one
		let processes_ready = new_node.processes_ready(project.clone());
		println!("processes_ready {:?}", processes_ready);
		let (weight, child) = if processes_ready.len() > 0 {
			println!("ici");
			new_node.child_from_process(project.clone(), processes_ready)
		} else {
			println!("la");
		    new_node.child_from_new_turn(project.clone())
		};
		new_node.child = Some(Rc::new(child));
		(weight, new_node)
	}

	/// Create the root of the tree and all its childs.
	pub fn launch_node_tree(project: ProjectPtr) -> (i32, Node) {
		let end_process_stack = EndProcessStack::new(project.clone());
		let resource_list = ResourceList::new(project.begin_resources());
		Node::new(project, 1, resource_list, end_process_stack)
	}

	/// Return how much this node produced according to the project resources
	/// to optimize.
	fn compute_weight(&self, project: ProjectPtr) -> i32 {
		let mut to_return: i32 = 0;
		for res in project.get_resources_to_optimize() {
			let i_resource = (*res).borrow().get_index();
			to_return += self.resources.nb_resource(i_resource) as i32;
		}
		if project.optimize_time() {
		    to_return -= self.resources.time_consumed() as i32;
		}
		to_return
	}
}