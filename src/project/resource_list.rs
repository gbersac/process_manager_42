use project::{ProjectPtr, ProcessPtr};

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceList {
    /// A list of size nb_resource with list[i] = number of resource which
    /// index is i.
    ///
    /// Resource of index 0 is time
    list: Vec<usize>,
}

impl ResourceList {
    pub fn new(list: Vec<usize>) -> ResourceList {
        ResourceList { list: list }
    }

    /// Return a new `ResourceList` with the number of resource decreased by
    /// the number of resource required to launch `nb_process` new process
    /// of index `i_process`.
    pub fn launch_process(&mut self,
                          process: ProcessPtr,
                          nb_process: usize) {
        let pre_vec = process.borrow().get_pre_vec().clone();
        for i in 1..self.list.len() {
            let res_consumed = pre_vec[i];
            self.list[i] -= res_consumed * nb_process;
        }
        self.list[0] += pre_vec[0] * nb_process;
    }

    pub fn new_launch_process(&self,
                              process: ProcessPtr,
                              nb_process: usize) -> ResourceList {
        let mut to_return = self.clone();
        to_return.launch_process(process, nb_process);
        to_return
    }

    pub fn is_empty(&self) -> bool {
        for i in 1..self.list.len() {
            if i != 0 {
                return false;
            }
        }
        true
    }

    pub fn get_list(&self) -> &Vec<usize> {
        &self.list
    }

    pub fn add_resource(&mut self, i_resource: usize, value: usize) {
        self.list[i_resource + 1] += value;
    }

    pub fn nb_resource(&self, i_resource: usize) -> usize {
        self.list[i_resource]
    }

    pub fn time_consumed(&self) -> usize {
        self.list[0]
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for ResourceList {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.list);
        Ok(())
    }
}
