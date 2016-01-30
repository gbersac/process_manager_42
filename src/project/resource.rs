use std::cell::RefCell;
use std::rc::Rc;

pub type ResourcePtr = Rc<RefCell<Resource>>;

#[derive(Debug)]
pub struct Resource {
    name: String,
    quantity: usize
}

impl Resource {
    pub fn new(name: &str) -> Resource {
        Resource {
            name: name.to_string(),
            quantity: 0
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
}

impl PartialEq for Resource {
    fn eq(&self, other: &Resource) -> bool {
        self.name == other.name
    }
}
