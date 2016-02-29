use std::rc::Rc;
use project::{Resource, ResourcePtr, TokenProcess, Process, ProcessPtr, ArcPtr, Arc};
use std::collections::BTreeMap;
use fn_string;
use error::KrpSimError;
use parse::Parser;
use matrix::Matrix;
use std::borrow::{BorrowMut};
use project::ResourceList;

pub type ProjectPtr = Rc<Project>;

#[derive(Debug)]
pub struct Project {
    resources: BTreeMap<usize, ResourcePtr>,
    resources_to_optimize: Vec<ResourcePtr>,
    processes: BTreeMap<usize, ProcessPtr>,
    pre_arc: Vec<ArcPtr>,
    post_arc: Vec<ArcPtr>,
    optimize_time: bool,
    delay: usize,

    /// List of all the processes which create one of the resources to optimize
    final_processes: Vec<ProcessPtr>
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
        self.resources[&index].clone()
    }

    pub fn get_processes(&self) -> &BTreeMap<usize, ProcessPtr> {
        &self.processes
    }

    pub fn get_process_by_name(&self, name: &str) -> Option<ProcessPtr> {
        for (_, ref process) in &self.processes {
            if name == process.borrow().get_name() {
                return Some((*process).clone());
            }
        }
        None
    }

    pub fn get_process_by_index(&self, index: usize) -> ProcessPtr {
        self.processes[&index].clone()
    }
    pub fn nb_resource(&self) -> usize {
        self.resources.len()
    }

    pub fn nb_process(&self) -> usize {
        self.processes.len()
    }

    pub fn get_resources_to_optimize(&self) -> &Vec<ResourcePtr> {
        &self.resources_to_optimize
    }

    pub fn optimize_time(&self) -> bool {
        self.optimize_time
    }

    pub fn get_delay(&self) -> usize {
        self.delay
    }

    pub fn get_final_processes(&self) -> &Vec<ProcessPtr> {
        &self.final_processes
    }

    /// Return a list of all the resources available at the begin of the
    /// simulation.
    pub fn begin_resources(&self) -> ResourceList {
        let mut to_return = ResourceList::new();
        for (_, resource) in &self.resources {
            to_return.add_resource(resource.clone(),
                                    resource.borrow().get_begin_quantity());
        }
        to_return
    }

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

    fn map_to_ressources(&mut self, map: BTreeMap<String, usize>) -> Vec<(ResourcePtr, usize)> {
        let mut to_return = Vec::new();
        for (key, value) in map.iter() {
            let res = self.add_ressource(key.clone());
            to_return.push((res, value.clone()));
        }
        to_return
    }

    pub fn new(resources: Vec<ResourcePtr>,
               token_processes: Vec<TokenProcess>,
               optimize: Vec<String>,
               delay: usize)
               -> Project {
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
            optimize_time: false,
            delay: delay,
            final_processes: Vec::new()
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
                Some(resource) => {
                    (*resource).borrow_mut().set_is_optimized();
                    resources_to_optimize.push(resource.clone());
                }
                None => {
                    if res == "time" {
                        // time is a special ressource
                        project.optimize_time = true;
                    } else {
                        println!("Unknow resource to optimize {:?}", res);
                    }
                }
            }
        }
        project.resources_to_optimize = resources_to_optimize;

        // fill the `final_processes` attribute
        for (_, process) in &project.processes {
            if process.borrow().produce_resources(&project.resources_to_optimize) {
                project.final_processes.push(process.clone());
            }
        }

        project
    }

    pub fn from_file(file_name: &str, delay: usize) -> Project {
        let instructions_str = fn_string::file_as_string(file_name);
        match Parser::parse(&instructions_str) {
            Ok((ressources, optimize, processes)) => {
                // launch process resolution
                Project::new(ressources, processes, optimize, delay)
            }
            Err(e) => {
                match e {
                    KrpSimError::ParseError(line) => {
                        let line_str = fn_string::get_line(&instructions_str, line - 1).unwrap();
                        panic!("Error parsing file {} on line {}:\n{}",
                               file_name,
                               line,
                               line_str)
                    }
                }
            }
        }
    }
}
