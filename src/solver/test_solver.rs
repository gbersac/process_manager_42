use std::rc::Rc;
use project::Project;
use solver::node::Node;
use project::ResourceList;
use solver::end_process_stack::EndProcessStack;
use std;

/// Launching `nb_process` process of index `i_process` must consume all the
/// resources of  `resources` and consume `time` turn
fn test_one0(file: &str,
             process_name: &str,
             nb_process: usize,
             time: usize,
             resources: Vec<usize>) {
    let project = Rc::new(Project::from_file(file, 1));
    println!("project {:?}", project);
    let resource_list = ResourceList::new(resources);
    let process = project.get_process_by_name(process_name).unwrap().clone();
    let result = resource_list.launch_process(project.clone(),
                                              process.borrow().get_index(),
                                              nb_process);
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
    test_one0("inputs/ikea",
              "do_armoire_ikea",
              3,
              90,
              vec![0, 0, 6, 3, 9, 0]);
    test_one0("inputs/simple",
              "achat_materiel",
              1,
              10,
              vec![0, 8, 0, 0, 0]);
}

/// For EndProcessStack::pop_and_terminate
/// The production of all the processes in `processes` must be equal test_one_resource_list
/// `expected_resources`
fn test_one1(file: &str, processes: Vec<(&str, usize)>, expected_resources: Vec<usize>) {
    let project = Rc::new(Project::from_file(file, 1));
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
    test_one1("inputs/recre",
              vec![("se_battre_dans_la_cours", 5), ("jouer_a_la_marelle", 1)],
              vec![0, 5, 6, 1]);
    test_one1("inputs/simple",
              vec![("achat_materiel", 1), ("realisation_produit", 1)],
              vec![0, 0, 1, 1, 0]);
}
