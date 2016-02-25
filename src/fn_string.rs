use std::str::FromStr;
use std::fs::File;
use std::io::Read;

pub fn sub_string(model: &String, begin: usize, len: usize) -> Option<String> {
    let mut end = begin + len;

    if begin > model.len() {
        return None;
    }
    if begin + len > model.len() {
        end = model.len() - begin;
    }
    Some(String::from_str(&(model[begin..end])).unwrap())
}

pub fn get_line(s: &str, num_line: usize) -> Option<&str> {
    let split: Vec<&str> = s.split("\n").collect();
    if split.len() >= num_line {
        Some(split[num_line])
    } else {
        None
    }
}

pub fn file_as_string(file_name: &str) -> String {
    let mut f = File::open(file_name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    s
}

#[test]
fn test_sub_string() {
    let test1 = sub_string(&"aaabbbaaa".to_string(), 3, 3).unwrap();
    println!("test1: {}", test1);
    assert_eq!(test1, "bbb");

    let test2 = sub_string(&"bbb".to_string(), 0, 3).unwrap();
    println!("test2: {}", test2);
    assert_eq!(test2, "bbb");

    let test3 = sub_string(&"b".to_string(), 0, 3).unwrap();
    println!("test3: {}", test3);
    assert_eq!(test3, "b");

    let test4 = sub_string(&"b".to_string(), 5, 3);
    assert_eq!(test4, None);
}
