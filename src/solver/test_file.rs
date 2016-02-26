use std::rc::Rc;
use project::Project;
use solver::resource_list::{ResourceList};
use solver::{NodePtr};
use solver;

fn print_solution(solution: Option<NodePtr>) {
    match solution {
        Some(node) => {
            println!("{}", node);
            print_solution(node.get_child());
        }
        None => {}
    }
}

fn test_one(file: &str,
            delay: usize,
            expected_resources: Vec<usize>,
            expected_weight: i32) {
    println!("\nFor {:?}", file);
    let project = Rc::new(Project::from_file(file, delay));
    println!("{:?}", project);
    let (weight, node) = solver::solve(project.clone());
    let expected_resources = ResourceList::new(expected_resources);
    let result = node.get_final_resources();

    println!("Result resources {:?} expected {:?}", result, expected_resources);
    println!("Result weight {:?} expected {:?}", weight, expected_weight);
    print_solution(Some(node.clone()));

    assert!(*result == expected_resources && weight == expected_weight);
}

#[test]
fn test_solve() {
    test_one("inputs/tests/long_way", 10, vec![4, 0, 0, 2], 2);
    test_one("inputs/tests/recycle", 10, vec![4, 0, 1, 0, 2], 2);
    test_one("inputs/tests/sacrifice", 6, vec![2, 2, 0], 2);
    test_one("inputs/tests/many_processes", 6, vec![100000, 0, 200000], 200000);
}
