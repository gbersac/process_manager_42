use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use super::{ResourcePtr};

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
struct ResourceQt
{
    pub res: ResourcePtr,
    pub quantities: usize
}

impl ResourceQt
{
    pub fn new(res: ResourcePtr, quantities: usize ) -> ResourceQt
    {
        ResourceQt {
            res: res,
            quantities: quantities
        }
    }
}

pub struct Process {
    name: String,
    prerequisites: Vec<ResourceQt>,
    products: Vec<ResourceQt>,
    time: usize
}

impl Process {
    fn map_to_resqt(map: Vec<(ResourcePtr, usize)>) -> Vec<ResourceQt> {
        let mut to_return = Vec::new();
        for (key, value) in map {
            to_return.push(ResourceQt::new(key, value));
        }
        to_return
    }

    pub fn new(
        name: String,
        prerequisites: Vec<(ResourcePtr, usize)>,
        products: Vec<(ResourcePtr, usize)>,
        time: usize
    ) -> Process {
        Process {
            name: name,
            prerequisites: Process::map_to_resqt(prerequisites),
            products: Process::map_to_resqt(products),
            time: time
        }
    }

    pub fn new_ptr(
        name: String,
        prerequisites: Vec<(ResourcePtr, usize)>,
        products: Vec<(ResourcePtr, usize)>,
        time: usize
    ) -> ProcessPtr {
        Rc::new(RefCell::new(
                Process::new(name, prerequisites, products, time)))
    }

    /// This function inform each of the resources it relates with
    /// (requirements and products) of this dependencies.
    /// The goal is to create a graph of resources/processes.
    ///
    /// The `self_ptr` is the `Rc` pointer to this process.
    pub fn resolve_dependency(&mut self,
        self_ptr: ProcessPtr
    ) {
        for prerequisite in self.prerequisites.iter() {
            prerequisite.res.borrow_mut().add_consumer(self_ptr.clone());
        }
        for product in self.products.iter() {
            product.res.borrow_mut().add_creator(self_ptr.clone());
        }
    }
}

use std::fmt::{Formatter, Debug, Error};

impl Debug for Process
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        write!(f, "{}", self.name);
        Ok(())
    }
}
