use std::rc::Rc;
use project::{ProcessPtr, Project, ResourceList};
use matrix::Matrix;
use main;

fn test_one_can_trigger_process(file: &str,
                                process_name: &str,
                                resources: Vec<usize>,
                                expected: usize) {
    let project = Rc::new(Project::from_file(file, 100));
    println!("project {:?}", project);
    let process = project.get_process_by_name(process_name).unwrap();
    println!("###process name {:?}", process.borrow().get_name());
    let resource_list = ResourceList::from_vec(project, resources);
    let result = process.borrow().can_trigger(&resource_list);
    println!("expected {} found {}", expected, result);
    assert!(result == expected);
}

#[test]
fn test_can_trigger_process_simple() {
    test_one_can_trigger_process("inputs/simple", "achat_materiel",
                                 vec![80, 0, 0, 0], 10);
    test_one_can_trigger_process("inputs/simple", "achat_materiel",
                                 vec![0, 9, 9, 9], 0);
    test_one_can_trigger_process("inputs/simple", "achat_materiel",
                                 vec![9, 9, 9, 9], 1);
}

#[test]
fn test_can_trigger_process_pomme() {
    test_one_can_trigger_process("inputs/pomme",
                                 "reunion_oeuf",
                                 vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                                 0);
    test_one_can_trigger_process("inputs/pomme",
                                 "reunion_oeuf",
                                 vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                                 1);
    test_one_can_trigger_process("inputs/pomme",
                                 "reunion_oeuf",
                                 vec![0, 0, 0, 0, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 0, 0],
                                 1);
    test_one_can_trigger_process("inputs/pomme",
                                 "reunion_oeuf",
                                 vec![0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 0, 0, 0, 0, 0, 0],
                                 3);
}
