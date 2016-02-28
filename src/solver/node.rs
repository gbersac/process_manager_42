use std::rc::Rc;
use project::{Project, ProjectPtr, ResourceList, ProcessList, Process};
use solver::end_process_stack::EndProcessStack;

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
    processes_to_end: EndProcessStack,
}

impl Node {
    /// Return a list of process ready to be launch.
    fn processes_ready(&self, project: ProjectPtr) -> ProcessList {
        let mut to_return = ProcessList::new();
        for i_process in 0..project.nb_process() {
            let process = project.get_process_by_index(i_process);
            let nb_process = process.borrow().can_trigger(&self.resources);
            let process_time = self.time + process.borrow().get_time();
            if nb_process > 0 && process_time < project.get_delay() {
                to_return.add(process.clone(), nb_process);
            }
        }
        to_return
    }

    fn child_from_conflicting_processes(&self,
                                        project: ProjectPtr)
                                        -> (i32, NodePtr) {
        project.get_final_processes().iter().map(|final_process| {
            let mut new_resources = self.resources.clone();
            let new_processes =
                    Process::trigger_and_providers(final_process.clone(),
                                                   &mut new_resources,
                                                   &mut Vec::new());
            let mut processes_to_end = self.processes_to_end.clone();
            processes_to_end.add_process_list(new_processes);
            Node::new(project.clone(), self.time, new_resources, processes_to_end)
        })
       .max_by_key(|&(weight, _)| weight)
       .unwrap()
    }

    /// Return the best child of all the child created after they launched
    /// a process in `processes_ready`
    fn child_from_process(&self,
                          project: ProjectPtr,
                          processes_ready: ProcessList)
                          -> (i32, NodePtr) {
        processes_ready.iter().map(|&(ref process, nb_process)| {
            if processes_ready.len() == 1 {
                let resources = self.resources
                                    .new_launch_process(process.clone(),
                                                        nb_process);
                let processes_to_end = self.processes_to_end
                    .add_processes(project.clone(), process.clone(), nb_process);
                Node::new(project.clone(), self.time,
                          resources, processes_to_end)
            } else {
               self.child_from_conflicting_processes(project.clone())
            }
       })
       .max_by_key(|&(weight, _)| weight)
       .unwrap()
    }

    /// Return the best child after a new turn has been passed
    fn child_from_new_turn(&self, project: ProjectPtr) -> (i32, NodePtr) {
        let mut new_time = self.time;
        let mut processes_to_end = self.processes_to_end.clone();
        loop {
            new_time += 1;

            // check for end of simulation
            if new_time >= project.get_delay() {
                return (self.compute_weight(project), Rc::new(self.clone()));
            }

            // check if something happen in the turn
            if !processes_to_end.process_terminate_at_next_turn() {
                processes_to_end.decrement();
                continue;
            }

            // at least one process terminate, create new node
            let mut resource = processes_to_end.pop_and_terminate(project.clone(), &self.resources);
            return Node::new(project.clone(), new_time, resource, processes_to_end);
        }
    }

    pub fn new(project: ProjectPtr,
               time: usize,
               resources: ResourceList,
               processes_to_end: EndProcessStack)
               -> (i32, NodePtr) {
        let mut new_node = Node {
            time: time,
            child: None,
            resources: resources,
            processes_to_end: processes_to_end,
        };

        // check for end of simulation
        if new_node.time >= project.get_delay() {
            return (new_node.compute_weight(project), Rc::new(new_node));
        }

        // create all the possible child and select the best one
        let processes_ready = new_node.processes_ready(project.clone());
        let (weight, child) = if processes_ready.len() > 0 {
            new_node.child_from_process(project.clone(), processes_ready)
        } else {
            new_node.child_from_new_turn(project.clone())
        };
        new_node.child = Some(child);
        (weight, Rc::new(new_node))
    }

    /// Create the root of the tree and all its childs.
    pub fn launch_node_tree(project: ProjectPtr) -> (i32, NodePtr) {
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

    pub fn get_child(&self) -> Option<NodePtr> {
        self.child.clone()
    }

    pub fn get_final_resources(&self) -> &ResourceList {
        match self.child {
            Some(ref child) => child.get_final_resources(),
            None => &self.resources,
        }
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} -> {}", self.time, self.resources);
        Ok(())
    }
}
