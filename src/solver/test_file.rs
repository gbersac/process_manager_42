use std::rc::Rc;
use project::Project;
use project::{ResourceList};
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
            time: usize,
            expected_resources: Vec<usize>,
            expected_weight: i32) {
    println!("\nFor {:?}", file);
    let project = Rc::new(Project::from_file(file, delay));
    println!("{:?}", project);
    let (weight, node) = solver::solve(project.clone());
    let mut expected_resources = ResourceList::from_vec(project, expected_resources);
    let result = node.get_final_resources();
    expected_resources.set_time_consumed(time);

    println!("Result resources {} expected {}", result, expected_resources);
    println!("Result weight {} expected {}", weight, expected_weight);
    print_solution(Some(node.clone()));

    assert!(*result == expected_resources && weight == expected_weight);
}

#[test]
fn test_solve() {
    test_one("inputs/tests/long_way", 10, 4, vec![0, 0, 2], 2);
    test_one("inputs/tests/recycle", 10, 4, vec![0, 1, 0, 2], 2);
    test_one("inputs/tests/sacrifice", 6, 2, vec![2, 0], 2);
    test_one("inputs/tests/many_processes", 6, 1000000, vec![0, 2000000], 2000000);
    test_one("inputs/tests/split_resources", 9, 3, vec![0, 0, 0, 2], 2);
}
