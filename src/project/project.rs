use std::rc::Rc;
use project::{Resource, ResourcePtr, TokenProcess, Process, ProcessPtr, ArcPtr
		, Arc};
use std::collections::{BTreeMap};
use fn_string;
use error::{KrpSimError};
use parse::Parser;
use matrix::Matrix;

#[derive(Debug)]
pub struct Project {
	resources: BTreeMap<usize, ResourcePtr>,
	resources_to_optimize: Vec<ResourcePtr>,
	processes: BTreeMap<usize, ProcessPtr>,
	pre_arc: Vec<ArcPtr>,
	post_arc: Vec<ArcPtr>,
	optimize_time: bool
}

impl Project {
	pub fn get_resource_by_name(&self, name: &str) -> Option<ResourcePtr> {
		for (_, ref res) in &self.resources {
		    if name == res.borrow().get_name() {
		    	return Some((*res).clone());
		    }
		}
		None
	}

	pub fn get_resource_by_index(&self, index: usize) -> ResourcePtr {
		self.resources.get(&index).unwrap().clone()
	}

    pub fn nb_resource(&self) -> usize {
        self.resources.len()
    }

    pub fn nb_process(&self) -> usize {
        self.processes.len()
    }

	/**************************************************************************/
	/* New Project                                                            */
	/**************************************************************************/

	/// Add resource `resource_name` if it doesn't already exist.
	fn add_ressource(&mut self, resource_name: String) -> ResourcePtr {
		match self.get_resource_by_name(&resource_name) {
		    Some(res) => res,
		    None => {
		    	let index = self.resources.len();
				let new_resource = Resource::new_ptr(&resource_name, index);
				self.resources.insert(index, new_resource.clone());
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
		let mut map_resources: BTreeMap<usize, ResourcePtr> = BTreeMap::new();
		for res in resources {
			let res_index = res.borrow().get_index().clone();
		    map_resources.insert(res_index, res);
		}

		// create the project struct
		let mut project = Project {
			resources: map_resources,
			resources_to_optimize: Vec::new(),
			processes: BTreeMap::new(),
			pre_arc: Vec::new(),
			post_arc: Vec::new(),
			optimize_time: false
		};

		// transform TokenProcess into Process
		let mut processes = BTreeMap::new();
		for (num, ref tok) in token_processes.iter().enumerate() {
		    let new_process = Process::new_ptr(tok.name.clone(), tok.time, num);
		    processes.insert(num, new_process.clone());
		    for (prerequisite, number) in tok.prerequisites.clone() {
		    	let res = project.add_ressource(prerequisite);
		        let new_arc = Arc::new_pre(res, new_process.clone(), number);
		        project.pre_arc.push(new_arc);
		    }
		    for (product, number) in tok.products.clone() {
		    	let res = project.add_ressource(product);
		        let new_arc = Arc::new_post(res, new_process.clone(), number);
		        project.post_arc.push(new_arc);
		    }
		}
		project.processes = processes;

		// transform optimize into ResourcePtr
		let mut resources_to_optimize = Vec::new();
		for res in optimize {
			match project.get_resource_by_name(&res) {
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

    /// Return a matrix[nb_process, nb_resource + 1] with
    /// M[i_process, i_resource] set to the number of resource i_resource
    /// required to trigger a process i_process.
    ///
    /// Line 0 is the line which contain the time to complete a process.
    pub fn pre_mat(&self) -> Matrix {
        let mut to_return = Matrix::new(self.processes.len(),
                                        self.resources.len() + 1);
        for (_, process) in &self.processes {
            let i_process = process.borrow().get_index();
            let time = process.borrow().get_time();
            to_return.set(i_process, 0, time as i32);
        }
        for arc in &self.pre_arc {
            let process = arc.get_process();
            let i_process = process.borrow().get_index();
            let resour = arc.get_resource();
            let i_resource = resour.borrow().get_index();
            let value = arc.get_value();
            to_return.set(i_process, i_resource + 1, value as i32);
        }
        to_return
    }

    /// Return a matrix[nb_process, nb_resource] with M[i_process, i_resource]
    /// set to the number of resource i_resource produced by the process
    /// i_process.
    pub fn post_mat(&self) -> Matrix {
        let mut to_return = Matrix::new(self.processes.len(),
                                        self.resources.len());
        for arc in &self.post_arc {
            let process = arc.get_process();
            let i_process = process.borrow().get_index();
            let resour = arc.get_resource();
            let i_resource = resour.borrow().get_index();
            let value = arc.get_value();
            to_return.set(i_process, i_resource, value as i32);
        }
        to_return
    }

    /// Return the number of process of index `i_process` that can be launch.
    ///
    /// `resources` is a vector with `resources[i]` being the number of
    /// available resource of index `i`.
    pub fn can_trigger_process(&self,
        i_process: usize,
        resources: Vec<i32>
    ) -> usize {
        // get the vector of prerequisite for the process i_process
        let prerequisites = self.pre_mat().get_col(i_process);

        // check if there is enough of each resource (except time)
        let mut nb_match : usize = 0;
        for i in 0..self.nb_resource() {
            if prerequisites[i + 1] == 0 {
                continue ;
            }
            let nb_match_i = (resources[i] / prerequisites[i + 1]) as usize;
            if nb_match_i == 0 {
                return 0;
            } else if nb_match_i < nb_match {
                nb_match = nb_match_i;
            } else if nb_match == 0 {
                nb_match = nb_match_i;
            }
        }
        nb_match
    }
}
