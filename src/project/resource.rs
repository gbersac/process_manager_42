use std::cell::RefCell;
use std::rc::Rc;
use project::{ProcessPtr, ArcPtr};

pub type ResourcePtr = Rc<RefCell<Resource>>;

#[derive(Debug)]
pub struct Resource {
    /// The name is the id of the resource. Two resource can't have the same
    /// name.
    name: String,

    /// Quantity of this resource at the beginning of the simulation
    begin_quantity: usize,
    is_optimized: bool,
    index: usize,

    /// List of process which create this resource
    creators: Vec<ArcPtr>,

    /// List of process which use this resource
    consumers: Vec<ArcPtr>,
}

impl Resource {
    pub fn new(name: &str, index: usize) -> Resource {
        Resource {
            name: name.to_string(),
            begin_quantity: 0,
            is_optimized: false,
            creators: Vec::new(),
            consumers: Vec::new(),
            index: index,
        }
    }

    pub fn new_ptr(name: &str, index: usize) -> ResourcePtr {
        Rc::new(RefCell::new(Resource::new(name, index)))
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_begin_quantity(&self) -> usize {
        self.begin_quantity
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn add(&mut self, begin_quantity: usize) {
        self.begin_quantity += begin_quantity;
    }

    pub fn is_optimized(&self) -> bool {
        self.is_optimized
    }

    pub fn set_is_optimized(&mut self) {
        self.is_optimized = true;
    }

    pub fn add_creator(&mut self, creator_ptr: ArcPtr) {
        self.creators.push(creator_ptr);
    }

    pub fn add_consumer(&mut self, consumer_ptr: ArcPtr) {
        self.consumers.push(consumer_ptr);
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Resource) -> bool {
        self.name == other.name
    }
}
