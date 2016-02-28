use project::{ProcessPtr};
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct ProcessList {
    list: Vec<(ProcessPtr, usize)>,
}

impl ProcessList {
    pub fn new() -> ProcessList {
        ProcessList {
            list: Vec::new(),
        }
    }

    pub fn add(&mut self, nprocess: ProcessPtr, nb: usize) {
        for tuple in self.list.iter_mut() {
            if tuple.0.borrow().get_index() == nprocess.borrow().get_index() {
                *tuple = (nprocess.clone(), tuple.1 + nb);
                return ;
            }
        }
        self.list.push((nprocess.clone(), nb));
    }

    pub fn iter(&self) -> Iter<(ProcessPtr, usize)> {
        self.list.iter()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}
