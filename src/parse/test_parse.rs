use super::tokenizer::Token;
use super::tokenizer::TokenInfo;
use super::tokenizer::Tokenizer;
use super::parse::Parser;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;
use project::{Ressource, RessourcePtr, TokenProcess};

fn test_parsability(s: &str, is_correct: bool) {
	println!("\nFor : {:?}", s);
	let result = Parser::parse(&s.to_string());
	if is_correct {
	    assert!(result.is_some());
	} else {
	    assert!(result.is_none());
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

// fn test_tree(s: &str, tree: Rc<Imply>) {
//     println!("\nFor : {:?}", s);
//     let result = Parser::parse(&s.to_string());
//     match result {
//         Some(expr) => {
//             let result_tree = expr.get_instrs().get(0).unwrap();
//             println!("tree {:?}", result_tree.get_ident());
//             assert!(result_tree.eq(tree as Rc<Exp>));
//         },
//         None => panic!("The expr #{:?}# is false.", s),
//     };
// }

// fn test_tree2(s: &str) {
//     println!("\nFor : {:?}", s);
//     let result = Parser::parse(&s.to_string());
//     match result {
//         Some(expr) => {
//             let result_tree = expr.get_instrs().get(0).unwrap();
//             println!("tree {:?}", result_tree.get_ident());
//             assert!(result_tree.get_ident().unwrap() == s.to_string());
//         },
//         None => panic!("The expr #{:?}# is false.", s),
//     };
// }

// #[test]
// fn test_parse_tree() {
//     let tree = Imply::new (
//         Axiom::new('A') as Rc<Exp>,
//         Axiom::new('B') as Rc<Exp>
//     );
//     test_tree("A => B", tree);

//     test_tree2("(A|(B+C))=>D");
//     test_tree2("((!(A)+!(B))+!(C))=>D");
//     test_tree2("(!((A+C))|D)=>D");
// }
