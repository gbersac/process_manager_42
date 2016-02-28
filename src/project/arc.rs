use project::{ProcessPtr, ResourcePtr, ProcessList, ResourceList};
use std::rc::Rc;

pub type ArcPtr = Rc<Arc>;

enum ArcType {
    Pre,
    Post,
}

pub struct Arc {
    process: ProcessPtr,
    resource: ResourcePtr,
    value: usize,
    arc_type: ArcType,
}

impl Arc {
    /// Creation of an arc where the resource is a prerequisite of the process.
    pub fn new_pre(resource: ResourcePtr, process: ProcessPtr, value: usize) -> ArcPtr {
        let new_arc = Rc::new(Arc {
            process: process.clone(),
            resource: resource.clone(),
            value: value,
            arc_type: ArcType::Pre,
        });
        resource.borrow_mut().add_consumer(new_arc.clone());
        process.borrow_mut().add_prerequisite(new_arc.clone());
        new_arc
    }

    /// Creation of an arc where the process produce the resource.
    pub fn new_post(resource: ResourcePtr, process: ProcessPtr, value: usize) -> ArcPtr {
        let new_arc = Rc::new(Arc {
            process: process.clone(),
            resource: resource.clone(),
            value: value,
            arc_type: ArcType::Post,
        });
        resource.borrow_mut().add_creator(new_arc.clone());
        process.borrow_mut().add_product(new_arc.clone());
        new_arc
    }

    pub fn get_resource(&self) -> ResourcePtr {
        self.resource.clone()
    }

    pub fn get_process(&self) -> ProcessPtr {
        self.process.clone()
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn is_resource(&self, resource: ResourcePtr) -> bool {
        resource.borrow().get_index() == self.resource.borrow().get_index()
    }

    /// Try to to create as many of this resource as possible.
    ///
    /// Return the list of processes launched, resulting resource list.
    pub fn trigger_and_providers(&self,
                                 mut resources: &mut ResourceList,
                                 already_tested: &mut Vec<ProcessPtr>)
                                 -> ProcessList {
        let mut new_processes = ProcessList::new();
        for creator in self.resource.borrow().get_creators() {
            let process = creator.get_process();
            let nb_process = process.borrow().can_trigger(&resources);
            if nb_process > 0 {
                *resources = resources.launch_process(process.clone(),
                                                     nb_process);
                new_processes.add(process.clone(), nb_process);
                already_tested.push(process.clone());
                if resources.is_empty() {
                    return new_processes;
                } else {
                    for pre in process.borrow().get_prerequisites() {
                        let mut new_processes2 =
                                pre.trigger_and_providers(&mut resources,
                                                          already_tested);
                        new_processes.append(&mut new_processes2);
                    }
                }
            }
        }
        new_processes
    }
}

use std::fmt::{Formatter, Debug, Error};

impl Debug for Arc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.arc_type {
            ArcType::Pre => {
                write!(f,
                       "{}->{}",
                       self.resource.borrow().get_name(),
                       self.process.borrow().get_name())
            }
            ArcType::Post => {
                write!(f,
                       "{}->{}",
                       self.process.borrow().get_name(),
                       self.resource.borrow().get_name())
            }
        };
        Ok(())
    }
}
