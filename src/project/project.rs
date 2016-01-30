use std::rc::Rc;
use project::{Resource, ResourcePtr, TokenProcess, Process};
use std::collections::{BTreeMap};

#[derive(Debug)]
pub struct Project {
	resources: BTreeMap<String, ResourcePtr>,
	resources_to_optimize: Vec<ResourcePtr>,
	processes: Vec<Process>,
	optimize_time: bool
}

impl Project {
	pub fn get_resource(&self, name: &str) -> Option<ResourcePtr> {
		match self.resources.get(name) {
		    Some(ref expr) => Some((*expr).clone()),
		    None => None,
		}
	}

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
		    let new_process = Process::new(
		    		tok.name,
		    		project.map_to_ressources(tok.prerequisites),
		    		project.map_to_ressources(tok.products),
		    		tok.time);
		    processes.push(new_process);
		}
		project.processes = processes;

		// transform optimize into ResourcePtr
		let mut resources_to_optimize = Vec::new();
		for res in optimize {
			match project.get_resource(&res) {
			    Some(resptr) => resources_to_optimize.push(resptr),
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
}
