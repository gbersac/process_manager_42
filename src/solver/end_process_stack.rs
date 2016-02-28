use std::collections::BinaryHeap;
use project::{ProjectPtr, ResourceList, ProcessPtr};
use std;
use std::cmp::Ordering;

const TIME_TO_TERMINATE: usize = 0;

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct EndProcessStack {
    processes_to_end: BinaryHeap<ProcessEnd>,
}

impl EndProcessStack {
    pub fn new(project: ProjectPtr) -> EndProcessStack {
        EndProcessStack { processes_to_end: BinaryHeap::new() }
    }

    pub fn add_processes(&self,
                         project: ProjectPtr,
                         process: ProcessPtr,
                         nb_process: usize)
                         -> EndProcessStack {
        let mut to_return = self.clone();
        let time = process.borrow().get_time();
        let process_end = ProcessEnd::new(time, process, nb_process);
        to_return.processes_to_end.push(process_end);
        to_return
    }

    #[cfg(test)]
    pub fn add_processes__next_turn(&self,
                                    project: ProjectPtr,
                                    i_process: usize,
                                    nb_process: usize)
                                    -> EndProcessStack {
        let mut to_return = self.clone();
        let process_end = ProcessEnd::new(0, i_process, nb_process);
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
            let post = process.borrow().get_post_vec().clone();
            for i in 0..project.nb_resource() {
                to_return.add_resource(i, process_end.number * post[i]);
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
