use project::{ProcessPtr, Project};
use main;

fn test_one(file_name: &str, expected: Vec<(&str, usize)>) {
	println!("for {:?}", file_name);

	// parse instructions file
	let mut project = Project::project_from_file(file_name);
	let result: Vec<(ProcessPtr, usize)> = project.processes_to_launch();

	// test if result match expected
	for (exp, exp_num) in expected {
		let mut found = false;
		for &(ref res, ref res_num) in &result {
		    if exp == res.borrow().get_name() {
		    	if exp_num == *res_num {
		    	    found = true;
		    	} else {
		    	    panic!("Process {} is launch {} time instead of {}",
		    	    		exp, res_num, exp_num);
		    	}
		    }
		}
		if !found {
		    panic!("Process {:?} not found", exp);
		}
	}
}

#[test]
fn test_recycle() {
    test_one("recycle", vec![("recyle", 1)]);
}

#[test]
fn test_long_way() {
    test_one("long_way", vec![("ProvideLongEfficient1", 2)]);
}

#[test]
fn test_sacrifice() {
    test_one("sacrifice", vec![("buy", 1)]);
}