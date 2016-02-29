use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use super::ResourcePtr;
use project::{ArcPtr, ResourceList, ProcessList};
use std;

pub type ProcessPtr = Rc<RefCell<Process>>;

/// Created by parsing. To be transformed to a process.
pub struct TokenProcess {
    pub name: String,
    pub prerequisites: BTreeMap<String, usize>,
    pub products: BTreeMap<String, usize>,
    pub time: usize,
}

impl TokenProcess {
    pub fn new(name: String,
               prerequisites: BTreeMap<String, usize>,
               products: BTreeMap<String, usize>,
               time: usize)
               -> TokenProcess {
        TokenProcess {
            name: name,
            prerequisites: prerequisites,
            products: products,
            time: time,
        }
    }
}

#[derive(Debug)]
pub struct Process {
    name: String,
    prerequisites: Vec<ArcPtr>,
    products: Vec<ArcPtr>,
    time: usize,
    index: usize,
}

impl Process {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn new(name: String, time: usize, index: usize) -> Process {
        Process {
            name: name,
            prerequisites: Vec::new(),
            products: Vec::new(),
            time: time,
            index: index,
        }
    }

    pub fn new_ptr(name: String, time: usize, index: usize) -> ProcessPtr {
        Rc::new(RefCell::new(Process::new(name, time, index)))
    }

    pub fn add_prerequisite(&mut self, resource: ArcPtr) {
        self.prerequisites.push(resource);
    }

    pub fn add_product(&mut self, resource: ArcPtr) {
        self.products.push(resource);
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_time(&self) -> usize {
        self.time
    }

    pub fn get_prerequisites(&self) -> &Vec<ArcPtr> {
        &self.prerequisites
    }

    pub fn get_products(&self) -> &Vec<ArcPtr> {
        &self.products
    }

    /// Return true if this process produce at least one of the resources
    /// listed in `resources`.
    pub fn produce_resources(&self, resources: &Vec<ResourcePtr>) -> bool {
        for product in &self.products {
            for resource in resources {
                if product.is_resource(resource.clone()) {
                    return true;
                }
            }
        }
        false
    }

    /// Return the number of time this process can be launched.
    pub fn can_trigger(&self, resources: &ResourceList) -> usize {
        // check if there is enough of each resource (except time)
        let mut nb_match: usize = 0;
        for pre in &self.prerequisites {
            if pre.get_value() == 0 {
                continue;
            }
            let nb_match_i = resources.nb_resource(pre.get_resource()) /
                             pre.get_value() as usize;
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

    /// Try to launch the process, then the providers of this process...
    ///
    /// Return the list of processes launched, resulting resource list.
    pub fn trigger_and_providers(selfp: ProcessPtr,
                                 resources: &mut ResourceList,
                                 already_tested: &mut Vec<ProcessPtr>)
                                 -> ProcessList {
        let nb_process = selfp.borrow().can_trigger(&resources);
        resources.launch_process(selfp.clone(), nb_process);
        let mut new_processes = ProcessList::new();
        new_processes.add(selfp.clone(), nb_process);
        if resources.is_empty() {
            (new_processes)
        } else {
            already_tested.push(selfp.clone());
            for pre in selfp.borrow().get_prerequisites() {
                let mut new_processes2 =
                        pre.trigger_and_providers(resources,
                                                  already_tested);
                new_processes.append(&mut new_processes2);
            }
            new_processes
        }
    }
}
