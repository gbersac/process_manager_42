use super::tokenizer::Token;
use super::tokenizer::TokenInfo;
use super::tokenizer::Tokenizer;
use super::parse::Parser;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;
use project::{Resource, ResourcePtr, TokenProcess};

fn test_parsability(s: &str, is_correct: bool) {
	println!("\nFor : {:?}", s);
	let result = Parser::parse(&s.to_string());
	if is_correct {
	    assert!(result.is_ok());
	} else {
	    assert!(result.is_err());
	}
}

#[test]
fn test_parser_basics() {
    test_parsability("", true);
    test_parsability("# aaaaaa", true);
    test_parsability("euro:10", true);
    test_parsability("achat_materiel:(euro:8):(materiel:1):10", true);
    test_parsability("achat_materiel:(euro:8;blop:8):(materiel:1;clap:1):10", true);
    test_parsability("optimize:(time;client_content)", true);

    test_parsability("A ", false);
    test_parsability("achat_materiel:(euro:8):(materiel:1):10:10", false);
    test_parsability("euro:10:", false);
}
