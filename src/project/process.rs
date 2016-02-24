use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use super::{ResourcePtr};
use project::{ArcPtr};
use std;

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
    time: usize,
    index: usize,

    /// A vector with pre_vec[i] = number of resource of index i required to
    /// launch a new process. Set to none if uninitialized.
    ///
    /// pre_vec[0] is the time required to launch a process
    pre_vec: Vec<usize>,

    /// A vector with pre_vec[i] = number of resource of index i created by
    /// this process. Set to none if uninitialized.
    post_vec: Vec<usize>
}

impl Process {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn new(
        name: String,
        time: usize,
        index: usize
    ) -> Process {
        Process {
            name: name,
            prerequisites: Vec::new(),
            products: Vec::new(),
            time: time,
            index: index,
            post_vec: Vec::new(),
            pre_vec: Vec::new()
        }
    }

    pub fn new_ptr(
        name: String,
        time: usize,
        index: usize
    ) -> ProcessPtr {
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

    /// Have to be launch every time a prerequisites/products has been
    /// changed/added.
    pub fn init_resources_vec(&mut self, nb_resource: usize) {
        // init pre_vec
        let pre_vec_len = nb_resource + 1;
        self.pre_vec = std::iter::repeat(0).take(pre_vec_len)
                .collect::<Vec<usize>>();
        self.pre_vec[0] = self.time;
        for pre in &self.prerequisites {
            let resource = pre.get_resource().clone();
            let i_res = resource.borrow().get_index() + 1;
            let nb_resource = pre.get_value();
            self.pre_vec[i_res] = nb_resource;
        }

        // init post_vec
        let post_vec_len = nb_resource;
        self.post_vec = std::iter::repeat(0).take(post_vec_len)
                .collect::<Vec<usize>>();
        for post in &self.products {
            let resource = post.get_resource().clone();
            let i_res = resource.borrow().get_index();
            let nb_resource = post.get_value();
            self.post_vec[i_res] = nb_resource;
        }
    }

    pub fn get_pre_vec(&self) -> &Vec<usize> {
        &self.pre_vec
    }

    pub fn get_post_vec(&self) -> &Vec<usize> {
        &self.post_vec
    }
}
