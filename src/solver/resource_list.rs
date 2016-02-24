use project::{ProjectPtr};

#[derive(Debug, Clone)]
pub struct ResourceList {
	/// A list of size nb_resource with list[i] = number of resource which
	/// index is i.
	list: Vec<usize>,
}

impl ResourceList {
	pub fn new(list: Vec<usize>) -> ResourceList {
		ResourceList {
			list: list,
		}
	}

	/// Return a new `ResourceList` with the number of resource decreased by
	/// the number of resource required to launch `nb_process` new process
	/// of index `i_process`.
	pub fn launch_process(&self,
		project: ProjectPtr,
		i_process: usize,
		nb_process: usize
	) -> ResourceList {
		let mut new_list = self.list.clone();
		let process = project.get_process_by_index(i_process).clone();
		let pre_vec = process.borrow().get_pre_vec().clone();
		for i in 1..project.nb_resource() {
			let res_consumed = pre_vec[i + 1];
			new_list[i] -= res_consumed * nb_process;
		}
		ResourceList::new(new_list)
	}

	pub fn get_list(&self) -> &Vec<usize> {
		&self.list
	}

	pub fn add_resource(&mut self, i_resource: usize, value: usize) {
		self.list[i_resource] += value;
	}
}
