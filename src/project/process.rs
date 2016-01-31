use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use super::{ResourcePtr};
use project::{ArcPtr};

pub type ProcessPtr = Rc<RefCell<Process>>;

/// Created by parsing. To be transformed to a process.
// #[derive(Debug)]
pub struct TokenProcess {
    pub name: String,
    pub prerequisites: BTreeMap<String, usize>,
    pub products: BTreeMap<String, usize>,
    pub time: usize
}

impl TokenProcess {
    pub fn new(
    	name: String,
    	prerequisites: BTreeMap<String, usize>,
    	products: BTreeMap<String, usize>,
    	time: usize
    ) -> TokenProcess {
    	TokenProcess {
    		name: name,
    		prerequisites: prerequisites,
    		products: products,
    		time: time
    	}
    }
}

#[derive(Debug)]
pub struct Process {
    name: String,
    prerequisites: Vec<ArcPtr>,
    products: Vec<ArcPtr>,
    time: usize
}

impl Process {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn new(
        name: String,
        time: usize
    ) -> Process {
        Process {
            name: name,
            prerequisites: Vec::new(),
            products: Vec::new(),
            time: time
        }
    }

    pub fn new_ptr(
        name: String,
        time: usize
    ) -> ProcessPtr {
        Rc::new(RefCell::new(Process::new(name, time)))
    }

    pub fn add_prerequisite(&mut self, resource: ArcPtr) {
        self.prerequisites.push(resource);
    }

    pub fn add_product(&mut self, resource: ArcPtr) {
        self.products.push(resource);
    }
}
