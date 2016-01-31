use std::rc::Rc;
use project::{Resource, ResourcePtr, TokenProcess, Process, ProcessPtr, ArcPtr
		, Arc};
use std::collections::{BTreeMap};
use fn_string;
use error::{KrpSimError};
use parse::Parser;

#[derive(Debug)]
pub struct Project {
	resources: BTreeMap<String, ResourcePtr>,
	resources_to_optimize: Vec<ResourcePtr>,
	processes: Vec<ProcessPtr>,
	optimize_time: bool
}

impl Project {
	pub fn get_resource(&self, name: &str) -> Option<ResourcePtr> {
		match self.resources.get(name) {
		    Some(ref expr) => Some((*expr).clone()),
		    None => None,
		}
	}

	/**************************************************************************/
	/* New Project                                                            */
	/**************************************************************************/

	/// Add resource `resource_name` if it doesn't already exist.
	fn add_ressource(&mut self, resource_name: String) -> ResourcePtr {
		match self.get_resource(&resource_name) {
		    Some(res) => res,
		    None => {
				let new_resource = Resource::new_ptr(&resource_name);
				self.resources.insert(resource_name.clone(), new_resource.clone());
				new_resource
		    }
		}
	}

	fn map_to_ressources(&mut self,
		map: BTreeMap<String, usize>
	) -> Vec<(ResourcePtr, usize)> {
		let mut to_return = Vec::new();
		for (key, value) in map.iter() {
			let res = self.add_ressource(key.clone());
			to_return.push((res, value.clone()));
		}
		to_return
	}

	pub fn new(
		resources: Vec<ResourcePtr>,
		token_processes: Vec<TokenProcess>,
		optimize: Vec<String>
	) -> Project {
		// transform resources vec into a map
		let mut map_resources: BTreeMap<String, ResourcePtr> = BTreeMap::new();
		for res in resources {
			let res_name = res.borrow().get_name().clone();
		    map_resources.insert(res_name, res);
		}

		// create the project struct
		let mut project = Project {
			resources: map_resources,
			resources_to_optimize: Vec::new(),
			processes: Vec::new(),
			optimize_time: false
		};

		// transform TokenProcess into Process
		let mut processes = Vec::new();
		for tok in token_processes {
		    let new_process = Process::new_ptr(tok.name, tok.time);
		    processes.push(new_process.clone());
		    for (prerequisite, number) in tok.prerequisites {
		    	let res = project.add_ressource(prerequisite);
		        Arc::new_pre(res, new_process.clone(), number);
		    }
		    for (product, number) in tok.products {
		    	let res = project.add_ressource(product);
		        Arc::new_post(res, new_process.clone(), number);
		    }
		}
		project.processes = processes;

		// transform optimize into ResourcePtr
		let mut resources_to_optimize = Vec::new();
		for res in optimize {
			match project.get_resource(&res) {
			    Some(resptr) => {
			    	resptr.borrow_mut().set_is_optimized();
			    	resources_to_optimize.push(resptr);
			    },
			    None => {
			    	if res == "time" { /* time is a special ressource */
			    	    project.optimize_time = true;
			    	}
			    	println!("Unknow ressource to optimize {:?}", res);
			    },
			}
		}
		project.resources_to_optimize = resources_to_optimize;

		project
	}

	pub fn project_from_file(file_name: &str) -> Project {
		let instructions_str = fn_string::file_as_string(file_name);
		match Parser::parse(&instructions_str) {
		    Ok((ressources, optimize, processes)) => {
		    	// launch process resolution
		    	Project::new(ressources, processes, optimize)
		    },
		    Err(e) => {
		    	match e {
		    	    KrpSimError::ParseError(line) => {
						let line_str =
								fn_string::get_line(&instructions_str, line - 1).unwrap();
				    	panic!("Error parsing file {} on line {}:\n{}",
				    			file_name, line, line_str)
		    	    },
		    	}
		    },
		}
	}

	/**************************************************************************/
	/* Resolution                                                             */
	/**************************************************************************/

	/// Return the list of projects to execute in order to maximize the
	/// quantity of resources to optimize (`self.resources_to_optimize`).
	///
	/// In the return value, each process to launch is associated with the
	/// number of time it have to be launch.
	pub fn processes_to_launch(&mut self) -> Vec<(ProcessPtr, usize)> {
		unimplemented!();
	}

	pub fn new_turn(&mut self) -> bool {
		// decrease countdown of launched processes.
		unimplemented!();

		// get new process to launch
		let processes_to_launch = self.processes_to_launch();

		// launch them
		for (process, time) in processes_to_launch {
		    // process.launch(time);
		    unimplemented!();
		}
	    true
	}
}
