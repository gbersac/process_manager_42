use project::{ProcessPtr, ResourcePtr, ProjectPtr};

#[derive(Debug, Clone)]
pub struct ResourceList {
    time_consumed: usize,
    list: Vec<(ResourcePtr, usize)>,
}

impl ResourceList {
    pub fn new() -> ResourceList {
        ResourceList {
            time_consumed: 0,
            list: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn from_vec(project: ProjectPtr, vec: Vec<usize>) -> ResourceList {
        if project.nb_resource() < vec.len() {
            panic!("ResourceList::from_vec error: too many resources in vector");
        }
        let mut to_return = ResourceList {
            time_consumed: 0,
            list: Vec::new(),
        };
        for (i_resource, nb_resource) in vec.iter().enumerate() {
            if *nb_resource == 0 {
                continue ;
            }
            let resource = project.get_resource_by_index(i_resource);
            to_return.add_resource(resource, *nb_resource);
        }
        to_return
    }

    #[cfg(test)]
    pub fn set_time_consumed(&mut self, time_consumed: usize) {
        self.time_consumed = time_consumed;
    }

    pub fn add_resource(&mut self, resource: ResourcePtr, nb: usize) {
        for tuple in self.list.iter_mut() {
            if tuple.0.borrow().get_index() == resource.borrow().get_index() {
                *tuple = (resource.clone(), tuple.1 + nb);
                return ;
            }
        }
        self.list.push((resource.clone(), nb));
    }

    fn delete_resource(&mut self, resource: ResourcePtr, nb: usize) {
        for tuple in self.list.iter_mut() {
            if tuple.0.borrow().get_index() == resource.borrow().get_index() {
                if tuple.1 < nb {
                    panic!("Not enough resource {:?}", resource.borrow().get_name());
                }
                *tuple = (resource.clone(), tuple.1 - nb);
                return ;
            }
        }
        panic!("No resource {:?}", resource.borrow().get_name());
    }

    /// Return a new `ResourceList` with the number of resource decreased by
    /// the number of resource required to launch `nb_process` new process
    /// of index `i_process`.
    pub fn launch_process(&mut self,
                          process: ProcessPtr,
                          nb_process: usize) {
        for prerequisite in process.borrow().get_prerequisites() {
            self.delete_resource(prerequisite.get_resource(),
                                 prerequisite.get_value() * nb_process);
        }
        self.time_consumed += process.borrow().get_time() * nb_process;
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

    pub fn nb_resource(&self, resource: ResourcePtr) -> usize {
        for &(ref res, ref nb) in self.list.iter() {
            if res.borrow().get_index() == resource.borrow().get_index() {
                return *nb;
            }
        }
        0
    }

    pub fn time_consumed(&self) -> usize {
        self.time_consumed
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for ResourceList {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "ResourceList[time:{}", self.time_consumed);
        for &(ref resource, ref nb) in &self.list {
            write!(f, ", {}:{}", resource.borrow().get_name(), nb);
        }
        write!(f, "]");
        Ok(())
    }
}

impl PartialEq for ResourceList {
    fn eq(&self, other: &ResourceList) -> bool {
        if self.time_consumed != other.time_consumed {
            return false;
        }
        for &(ref resource, nb_resource) in &self.list {
            if other.nb_resource(resource.clone()) != nb_resource {
                return false;
            }
        }
        for &(ref resource, nb_resource) in &other.list {
            if other.nb_resource(resource.clone()) != nb_resource {
                return false;
            }
        }
        true
    }
}
