use std::rc::Rc;
use project::{Project};
use solver::node::{Node};
use solver::resource_list::{ResourceList};
use solver::end_process_stack::{EndProcessStack};
use std;

fn test_one_compute_weight(
    file: &str,
    resources: Vec<usize>,
    expected: i32
) {
    let project = Rc::new(Project::from_file(file, 1));
    println!("project {:?}", project);
    let resource_list = ResourceList::new(resources);
    let (weight, _) = Node::new(project.clone(), 2, resource_list,
    		EndProcessStack::new(project.clone()));
    println!("expected {} found {}", expected, weight);
    assert!(weight == expected);
}

#[test]
fn test_compute_weight_no_time() {
	test_one_compute_weight("inputs/pomme",
            vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100],
			100);
	test_one_compute_weight("inputs/pomme",
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			0);
	test_one_compute_weight("inputs/pomme",
            vec![0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			100);
}

/// Launching `nb_process` process of index `i_process` must consume all the
/// resources of  `resources` and consume `time` turn
fn test_one_resource_list(
    file: &str,
    i_process: usize,
    nb_process: usize,
    time: usize,
    resources: Vec<usize>
) {
    let project = Rc::new(Project::from_file(file, 1));
    println!("project {:?}", project);
    let resource_list = ResourceList::new(resources);
    let result = resource_list.launch_process(project.clone(), i_process, nb_process);
    let mut res_vec = std::iter::repeat(0).take(project.nb_resource() + 1)
    		.collect::<Vec<usize>>();
    res_vec[0] = time;
    let expected = ResourceList::new(res_vec);
    println!("expected {:?} found {:?}", expected, result);
    assert!(result == expected);
}

#[test]
fn test_resource_list_launch_process() {
	test_one_resource_list("inputs/ikea", 3, 3, 90, vec![0, 0, 6, 3, 9, 0]);
}
