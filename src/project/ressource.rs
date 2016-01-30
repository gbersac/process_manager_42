use std::cell::RefCell;
use std::rc::Rc;

pub type RessourcePtr = Rc<RefCell<Ressource>>;

#[derive(Debug)]
pub struct Ressource {
    name: String,
    quantity: usize
}

impl Ressource {
    pub fn new(name: String) -> Ressource {
        Ressource {
        	name: name,
        	quantity: 0
        }
    }

    pub fn add(&mut self, quantity: usize) {
        self.quantity += quantity;
    }
}
