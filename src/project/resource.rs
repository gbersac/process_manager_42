use std::cell::RefCell;
use std::rc::Rc;
use project::{ProcessPtr};

pub type ResourcePtr = Rc<RefCell<Resource>>;

#[derive(Debug)]
pub struct Resource {
    name: String,
    quantity: usize,
    is_optimized: bool,

    /// List of process which create this resource
    creators: Vec<ProcessPtr>,

    /// List of process which use this resource
    consumers: Vec<ProcessPtr>
}

impl Resource {
    pub fn new(name: &str) -> Resource {
        Resource {
            name: name.to_string(),
            quantity: 0,
            is_optimized: false,
            creators: Vec::new(),
            consumers: Vec::new(),
        }
    }

    pub fn new_ptr(name: &str) -> ResourcePtr {
        Rc::new(RefCell::new(Resource::new(name)))
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add(&mut self, quantity: usize) {
        self.quantity += quantity;
    }

    pub fn is_optimized(&self) -> bool {
        self.is_optimized
    }

    pub fn set_is_optimized(&mut self) {
        self.is_optimized = true;
    }

    pub fn add_creator(&mut self, creator_ptr: ProcessPtr) {
        self.creators.push(creator_ptr);
    }

    pub fn add_consumer(&mut self, consumer_ptr: ProcessPtr) {
        self.consumers.push(consumer_ptr);
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Resource) -> bool {
        self.name == other.name
    }
}
