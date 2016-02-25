use std::rc::Rc;
use project::Project;
use solver::node::Node;
use solver::resource_list::ResourceList;
use solver::end_process_stack::EndProcessStack;
use std;

fn test_one_compute_weight(file: &str, resources: Vec<usize>, expected: i32) {
    let project = Rc::new(Project::from_file(file, 1));
    println!("project {:?}", project);
    let resource_list = ResourceList::new(resources);
    let (weight, _) = Node::new(project.clone(),
                                2,
                                resource_list,
                                EndProcessStack::new(project.clone()));
    println!("expected {} found {}", expected, weight);
    assert!(weight == expected);
}

#[test]
fn test_compute_weight_no_time() {
    test_one_compute_weight("inputs/pomme",
                            vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                                 100, 100, 100],
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
fn test_one_resource_list(file: &str,
                          i_process: usize,
                          nb_process: usize,
                          time: usize,
                          resources: Vec<usize>) {
    let project = Rc::new(Project::from_file(file, 1));
    println!("project {:?}", project);
    let resource_list = ResourceList::new(resources);
    let result = resource_list.launch_process(project.clone(), i_process, nb_process);
    let mut res_vec = std::iter::repeat(0)
                          .take(project.nb_resource() + 1)
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


/// The production of all the processes in `processes` must be equal test_one_resource_list
/// `expected_resources`
fn test_one_end_process_stack__pop_and_terminate(file: &str,
                                                 processes: Vec<(&str, usize)>,
                                                 expected_resources: Vec<usize>) {
    let project = Rc::new(Project::from_file(file, 1));
    println!("project {:?}", project);
    let res_vec = std::iter::repeat(0)
                      .take(project.nb_resource() + 1)
                      .collect::<Vec<usize>>();
    let resource_list = ResourceList::new(res_vec);
    let mut end_process_stack = EndProcessStack::new(project.clone());
    for (process_name, nb_process) in processes {
        let process = project.get_process_by_name(process_name).unwrap();
        end_process_stack = end_process_stack.add_processes__next_turn(project.clone(),
                                                                       process.borrow()
                                                                              .get_index(),
                                                                       nb_process);
    }
    let result = end_process_stack.pop_and_terminate(project, &resource_list);
    let expected = ResourceList::new(expected_resources);
    println!("expected {:?} found {:?}", expected, result);
    assert!(result == expected);
}

#[test]
fn test_end_process_stack__pop_and_terminate() {
    test_one_end_process_stack__pop_and_terminate("inputs/recre",
                                                  vec![("se_battre_dans_la_cours", 5),
                                                       ("jouer_a_la_marelle", 1)],
                                                  vec![0, 5, 6, 1]);
}
