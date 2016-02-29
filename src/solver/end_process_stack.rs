use std::collections::BinaryHeap;
use project::{ProjectPtr, ResourceList, ProcessPtr, ProcessList};
use std;
use std::cmp::Ordering;

const TIME_TO_TERMINATE: usize = 0;

#[derive(Clone, Debug)]
struct ProcessEnd {
    pub time: usize,

    /// Index of the process to terminate
    pub process: ProcessPtr,

    /// Number of process to terminate
    pub number: usize,
}

impl ProcessEnd {
    pub fn new(time: usize, process: ProcessPtr, number: usize) -> ProcessEnd {
        ProcessEnd {
            time: time,
            process: process,
            number: number,
        }
    }

    pub fn decrement(&mut self) {
        self.time -= 1;
    }
}

impl Eq for ProcessEnd {
    // add code here
}

impl PartialEq for ProcessEnd {
    fn eq(&self, other: &Self) -> bool {
        self.process.borrow().get_index() == other.process.borrow().get_index() &&
                self.time == other.time &&
                self.number == other.number
    }
}

impl Ord for ProcessEnd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

impl PartialOrd for ProcessEnd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.time.cmp(&other.time).reverse())
    }
}

/// List of all the process which are going to end in the future.
#[derive(Clone)]
pub struct EndProcessStack {
    processes_to_end: BinaryHeap<ProcessEnd>,
}

impl EndProcessStack {
    pub fn new(project: ProjectPtr) -> EndProcessStack {
        EndProcessStack { processes_to_end: BinaryHeap::new() }
    }

    fn add_processes(&mut self,
                     process: ProcessPtr,
                     nb_process: usize) {
        let time = process.borrow().get_time();
        let process_end = ProcessEnd::new(time, process, nb_process);
        self.processes_to_end.push(process_end);
    }

    pub fn new_add_processes(&self,
                             process: ProcessPtr,
                             nb_process: usize)
                             -> EndProcessStack {
        let mut to_return = self.clone();
        to_return.add_processes(process, nb_process);
        to_return
    }

    pub fn add_process_list(&mut self, list: ProcessList) {
        for &(ref process, nb_process) in list.iter() {
            self.add_processes(process.clone(), nb_process);
        }
    }

    #[cfg(test)]
    pub fn add_processes__next_turn(&self,
                                    project: ProjectPtr,
                                    i_process: usize,
                                    nb_process: usize)
                                    -> EndProcessStack {
        let mut to_return = self.clone();
        let process = project.get_process_by_index(i_process);
        let process_end = ProcessEnd::new(0, process, nb_process);
        to_return.processes_to_end.push(process_end);
        to_return
    }

    /// Return true if at least one process terminate at the next turn
    pub fn process_terminate_at_next_turn(&self) -> bool {
        if self.processes_to_end.len() == 0 {
            return false;
        }
        let next = self.processes_to_end.peek().unwrap();
        next.time == TIME_TO_TERMINATE
    }

    pub fn decrement(&mut self) {
        let mut vec = self.processes_to_end.clone().into_vec();
        for process_end in vec.iter_mut() {
            process_end.decrement();
        }
        self.processes_to_end = BinaryHeap::from(vec);
    }

    fn processes_terminating_this_turn(processes_to_end: &BinaryHeap<ProcessEnd>) -> Vec<&ProcessEnd> {
        let mut to_return = Vec::new();
        for process_end in processes_to_end {
            if process_end.time == TIME_TO_TERMINATE {
                to_return.push(process_end);
            } else {
                return to_return;
            }
        }
        to_return
    }

    /// Pop all processes that terminate at the next turn and return all the
    /// created resources added to `resources`.
    pub fn pop_and_terminate(&mut self,
                             project: ProjectPtr,
                             resources: &ResourceList)
                             -> ResourceList {
        // terminate processes
        let new_bin = self.processes_to_end.clone();
        let process_to_terminate = EndProcessStack::processes_terminating_this_turn(&new_bin);
        let mut to_return = resources.clone();
        for process_end in process_to_terminate {
            let process = process_end.process.clone();
            let products = process.borrow().get_products().clone();
            for product in products {
                let nb = process_end.number * product.get_value();
                to_return.add_resource(product.get_resource(), nb);
            }
        }

        // remove terminated processes
        let mut vec = Vec::new();
        for process_end in &self.processes_to_end {
            if process_end.time != TIME_TO_TERMINATE {
                vec.push(process_end.clone());
            }
        }
        self.processes_to_end = BinaryHeap::from(vec);

        to_return
    }
}

use std::fmt::{Formatter, Debug, Error};

impl Debug for EndProcessStack {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "EndProcessStack[");
        for p in &self.processes_to_end {
            write!(f, "{}:{}, ", p.process.borrow().get_name(), p.number);
        }
        write!(f, "]");
        Ok(())
    }
}
