use project::{ProjectPtr};
use solver::resource_list::{ResourceList};
use std;

/// List of all the process which are going to end in the future.
#[derive(Debug, Clone)]
pub struct EndProcessStack {
	max_process_time: usize,
	nb_process: usize,

	/// This is a M[`max_process_time`, `nb_process`] matrix (one sub vector
	/// for each turn to come).
	/// The processes_to_end[i_time, i_process] value is the number of process
	/// of index i_process which are going to end in i_time turn.
	processes_to_end: Vec<Vec<usize>>,
}

impl EndProcessStack {

	fn push_zeroed_vector(&mut self) {
		let new_vec = std::iter::repeat(0).take(self.nb_process)
		        .collect::<Vec<usize>>();
		self.processes_to_end.push(new_vec);
	}

	pub fn new(project: ProjectPtr) -> EndProcessStack {
		let new_vec = Vec::with_capacity(project.get_max_process_time());
		let mut to_return = EndProcessStack {
			processes_to_end: new_vec,
			nb_process: project.nb_process(),
			max_process_time: project.get_max_process_time()
		};
		for _ in 0..to_return.max_process_time + 3 {
			to_return.push_zeroed_vector();
		}
		to_return
	}

	pub fn add_processes(&self,
		project: ProjectPtr,
		i_process: usize,
		nb_process: usize,
	) -> EndProcessStack {
		let mut to_return = self.clone();
		let process = project.get_process_by_index(i_process);
		let time = process.borrow().get_time();
		to_return.processes_to_end[time - 1][i_process] += nb_process;
		to_return
	}

	#[cfg(test)]
	pub fn add_processes__next_turn(&self,
		project: ProjectPtr,
		i_process: usize,
		nb_process: usize,
	) -> EndProcessStack {
		let mut to_return = self.clone();
		let process = project.get_process_by_index(i_process);
		let time = process.borrow().get_time();
		to_return.processes_to_end[0][i_process] = nb_process;
		to_return
	}

	/// Return true if at least one process terminate at the next turn
	pub fn process_terminate_at_next_turn(&self) -> bool {
		let next = &self.processes_to_end[0];
		for nb_process in next {
		    if *nb_process > 0 {
		        return true;
		    }
		}
		false
	}

	pub fn pop_one_turn(&mut self) {
	    self.processes_to_end.remove(0);
	    self.push_zeroed_vector();
	}

	/// Pop all processes that terminate at the next turn and return all the
	/// created resources added to `resources`.
	pub fn pop_and_terminate(&mut self,
		project: ProjectPtr,
		resources: &ResourceList
	) -> ResourceList {
		let terminated_processes = self.processes_to_end.remove(0);
		let mut to_return = resources.clone();
		for i_process in 0..self.nb_process {
			let nb_terminated_process = terminated_processes[i_process];
			if nb_terminated_process == 0 {
			    continue ;
			}
			let process = project.get_process_by_index(i_process).clone();
			let post = process.borrow().get_post_vec().clone();
			for i in 0..project.nb_resource() {
				to_return.add_resource(i, nb_terminated_process * post[i]);
			}
		}
		to_return
	}
}
