use project::{ProcessPtr, Project, ResourceList};
use matrix::Matrix;
use main;

// fn test_one(file_name: &str, expected: Vec<(&str, usize)>) {
// 	println!("for {:?}", file_name);
//
// 	// parse instructions file
// 	let mut project = Project::from_file(file_name);
// 	let result: Vec<(ProcessPtr, usize)> = project.processes_to_launch();
//
// 	// test if result match expected
// 	for (exp, exp_num) in expected {
// 		let mut found = false;
// 		for &(ref res, ref res_num) in &result {
// 		    if exp == res.borrow().get_name() {
// 		    	if exp_num == *res_num {
// 		    	    found = true;
// 		    	} else {
// 		    	    panic!("Process {} is launch {} time instead of {}",
// 		    	    		exp, res_num, exp_num);
// 		    	}
// 		    }
// 		}
// 		if !found {
// 		    panic!("Process {:?} not found", exp);
// 		}
// 	}
// }
//
// #[test]
// fn test_recycle() {
//     test_one("recycle", vec![("recyle", 1)]);
// }
//
// #[test]
// fn test_long_way() {
//     test_one("long_way", vec![("ProvideLongEfficient1", 2)]);
// }
//
// #[test]
// fn test_sacrifice() {
//     test_one("sacrifice", vec![("buy", 1)]);
// }

fn test_one_can_trigger_process(file: &str,
                                process_name: &str,
                                resources: Vec<usize>,
                                expected: usize) {
    let project = Project::from_file(file, 100);
    println!("project {:?}", project);
    let process = project.get_process_by_name(process_name).unwrap();
    let resource_list = ResourceList::new(resources);
    let result = process.borrow().can_trigger(&resource_list);
    println!("expected {} found {}", expected, result);
    assert!(result == expected);
}

#[test]
fn test_can_trigger_process_simple() {
    test_one_can_trigger_process("inputs/simple", "achat_materiel",
                                 vec![0, 80, 0, 0, 0], 10);
    test_one_can_trigger_process("inputs/simple", "achat_materiel",
                                 vec![0, 0, 9, 9, 9], 0);
    test_one_can_trigger_process("inputs/simple", "achat_materiel",
                                 vec![0, 9, 9, 9, 9], 1);
}

#[test]
fn test_can_trigger_process_pomme() {
    test_one_can_trigger_process("inputs/pomme",
                                 "do_flan",
                                 vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                                 0);
    test_one_can_trigger_process("inputs/pomme",
                                 "do_flan",
                                 vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                                 1);
    test_one_can_trigger_process("inputs/pomme",
                                 "do_flan",
                                 vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 0, 0],
                                 1);
    test_one_can_trigger_process("inputs/pomme",
                                 "do_flan",
                                 vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 0, 0, 0, 0, 0, 0],
                                 3);
}
