use std::rc::Rc;
use project::Project;
use solver::node::Node;
use project::ResourceList;
use solver::end_process_stack::EndProcessStack;
use std;

/// Launching `nb_process` process of name `process_name` must consume all the
/// resources of  `resources` and consume `time` turn
fn test_one0(file: &str,
             process_name: &str,
             nb_process: usize,
             time: usize,
             resources: Vec<usize>) {
    let project = Rc::new(Project::from_file(file, 1));
    println!("project {:?}", project);
    let resource_list = ResourceList::from_vec(project.clone(), resources);
    let process = project.get_process_by_name(process_name).unwrap().clone();
    let result = resource_list.new_launch_process(process.clone(),
                                                  nb_process);
    let mut expected = ResourceList::new();
    expected.set_time_consumed(time);
    println!("expected {} found {}", expected, result);
    assert!(result == expected);
}

#[test]
fn test_resource_list_launch_process() {
    test_one0("inputs/ikea",
              "do_armoire_ikea",
              3,
              90,
              vec![0, 6, 3, 9, 0]);
    test_one0("inputs/simple",
              "achat_materiel",
              1,
              10,
              vec![8, 0, 0, 0]);
}

/// For EndProcessStack::pop_and_terminate
/// The production of all the processes in `processes` must be equal test_one_resource_list
/// `expected_resources`
fn test_one1(file: &str, processes: Vec<(&str, usize)>, expected_resources: Vec<usize>) {
    let project = Rc::new(Project::from_file(file, 1));
    let resource_list = ResourceList::new();
    let mut end_process_stack = EndProcessStack::new(project.clone());
    for (process_name, nb_process) in processes {
        let process = project.get_process_by_name(process_name).unwrap();
        end_process_stack = end_process_stack.add_processes__next_turn(project.clone(),
                                                                       process.borrow()
                                                                              .get_index(),
                                                                       nb_process);
    }
    let result = end_process_stack.pop_and_terminate(project.clone(), &resource_list);
    println!("ici");
    let expected = ResourceList::from_vec(project.clone(), expected_resources);
    println!("expected {:?} found {:?}", expected, result);
    assert!(result == expected);
}

#[test]
fn test_end_process_stack__pop_and_terminate() {
    test_one1("inputs/recre",
              vec![("se_battre_dans_la_cours", 5), ("jouer_a_la_marelle", 1)],
              vec![5, 6, 1]);
    test_one1("inputs/simple",
              vec![("achat_materiel", 1), ("realisation_produit", 1)],
              vec![0, 1, 1, 0]);
}
