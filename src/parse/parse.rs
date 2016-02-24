use super::tokenizer::Token;
use super::tokenizer::TokenInfo;
use super::tokenizer::Tokenizer;
use regex::Regex;
use std::collections::BTreeMap;
use std::cell::RefCell;
use std::rc::Rc;
use project::{Resource, ResourcePtr, TokenProcess};
use error::{KrpSimError};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TokenType
{
	OpenParenthesis,
	CloseParenthesis,
	Word,
	Colon,
	SemiColon,
	Number,
	Comment,
	Optimize,
	EndLine,
	Unknow
}

pub struct Parser {
	/// The index of the first token which have to be parsed
	index: usize,

	/// Tokens to parse
	tokens: Vec<Token<TokenType>>,

	/// The stack for generating the abstract syntax tree
	stack: BTreeMap<String, usize>,

	resources: Vec<ResourcePtr>,
	optimize: Vec<String>,
	processes: Vec<TokenProcess>
}

impl Parser {

	fn split_into_tokens(to_parse: &String) -> Vec<Token<TokenType>> {
		let token_types = vec![
			TokenInfo::new(TokenType::OpenParenthesis, Regex::new(r"\(").unwrap()),
			TokenInfo::new(TokenType::CloseParenthesis, Regex::new(r"\)").unwrap()),
			TokenInfo::new(TokenType::EndLine, Regex::new("\n").unwrap()),
			TokenInfo::new(TokenType::Optimize, Regex::new("optimize").unwrap()),
			TokenInfo::new(TokenType::Word, Regex::new("[A-Za-z][A-Za-z0-9_]*").unwrap()),
			TokenInfo::new(TokenType::Comment, Regex::new("#.*").unwrap()),
			TokenInfo::new(TokenType::Colon, Regex::new(":").unwrap()),
			TokenInfo::new(TokenType::SemiColon, Regex::new(";").unwrap()),
			TokenInfo::new(TokenType::Number, Regex::new("[0-9]+").unwrap()),
			TokenInfo::new(TokenType::Unknow, Regex::new(".*").unwrap()),
		];
		let tokenizer = Tokenizer::new(token_types, "[ \t]+");
		tokenizer.split(to_parse)
	}

	fn reached_end(&self) -> bool {
	    if self.index < self.tokens.len() {
	        false
	    } else {
	        true
	    }
	}

	fn save_state(&mut self) -> usize {
		self.index
	}

	fn restore_state(&mut self, restore: bool, old_state: usize) {
		if !restore {
			self.index = old_state;
			self.stack.clear()
		}
	}

	/// If one rule is optional mark it Parser::optional(Rule)
	fn optional(_: bool) -> bool {
	    true
	}

	fn tok_is_type(&mut self, tok_type: TokenType) -> bool {
	    let found = self.tokens[self.index].get_type().clone() == tok_type;
	    if found {
	        self.index += 1;
	    }
	    found
	}

	fn get_tok_content(&self, index: i32) -> String {
	    self.tokens[((self.index as i32) + index) as usize].get_content().clone()
	}

	fn get_tok_content_as_usize(&self, index: i32) -> usize {
	    let s = self.tokens[((self.index as i32) + index) as usize].
	    		get_content().clone();
		s.parse::<usize>().unwrap()
	}

	////////////////////////////////////////////////////////////////////// RULES

	fn rule_optimisation(&mut self) -> bool {
		let old_state = self.save_state();
		let mut opt = Vec::new();
		let mut to_return = self.tok_is_type(TokenType::Optimize) &&
				self.tok_is_type(TokenType::Colon) &&
				self.tok_is_type(TokenType::OpenParenthesis) &&
				self.tok_is_type(TokenType::Word);
		if !to_return {
			self.restore_state(to_return, old_state);
			return false;
		}
		opt.push(self.get_tok_content(-1));
		let mut carry_on = to_return;
		if carry_on {
			carry_on = self.tok_is_type(TokenType::SemiColon) &&
					self.tok_is_type(TokenType::Word);
			if carry_on {
				opt.push(self.get_tok_content(-1));
			}
		}
		to_return = to_return &&
				self.tok_is_type(TokenType::CloseParenthesis) &&
				self.tok_is_type(TokenType::EndLine);
		if to_return {
		    self.optimize.append(&mut opt);
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_name_number(&mut self) -> bool {
		let old_state = self.save_state();
		let mut to_return = self.tok_is_type(TokenType::Word) &&
				self.tok_is_type(TokenType::Colon) &&
				self.tok_is_type(TokenType::Number);
		if to_return {
			let name = self.get_tok_content(-3);
			let quantity = self.get_tok_content_as_usize(-1);
			self.stack.insert(name, quantity);
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_name_number_list(&mut self) -> bool {
		let old_state = self.save_state();
		let mut to_return = self.tok_is_type(TokenType::OpenParenthesis) &&
				self.rule_name_number();
		let mut carry_on = to_return;
		while carry_on {
			carry_on = self.tok_is_type(TokenType::SemiColon) &&
					self.rule_name_number();
		}
		to_return = to_return &&
				self.tok_is_type(TokenType::CloseParenthesis);
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_process(&mut self) -> bool {
		let old_state = self.save_state();
		let mut to_return =	self.tok_is_type(TokenType::Word);
		if !to_return {
			self.restore_state(to_return, old_state);
		    return false;
		}
		let name = self.get_tok_content(-1);
		let to_return = self.tok_is_type(TokenType::Colon) &&
				self.rule_name_number_list();
		if !to_return {
			self.restore_state(to_return, old_state);
		    return false;
		}
		let needs = self.stack.clone();
		self.stack.clear();
		let to_return = self.tok_is_type(TokenType::Colon) &&
				self.rule_name_number_list();
		if !to_return {
			self.restore_state(to_return, old_state);
		    return false;
		}
		let results = self.stack.clone();
		let to_return = self.tok_is_type(TokenType::Colon) &&
				self.tok_is_type(TokenType::Number) &&
				self.tok_is_type(TokenType::EndLine);
		if !to_return {
			self.restore_state(to_return, old_state);
		    return false;
		}
		let time = self.get_tok_content_as_usize(-2);
		let new_proc = TokenProcess::new(name, needs, results, time);
		self.processes.push(new_proc);
		true
	}

	fn rule_initial_stock(&mut self) -> bool {
		let old_state = self.save_state();
		let mut to_return =	self.tok_is_type(TokenType::Word) &&
				self.tok_is_type(TokenType::Colon) &&
				self.tok_is_type(TokenType::Number) &&
				self.tok_is_type(TokenType::EndLine);
		if to_return {
			let name = self.get_tok_content(-4);
			let quantity = self.get_tok_content_as_usize(-2) as usize;
			let mut res = Resource::new_ptr(&name, self.resources.len());
			res.borrow_mut().add(quantity);
			self.resources.push(res);
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_empty_line(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return =
				Parser::optional(self.tok_is_type(TokenType::Comment)) &&
				self.tok_is_type(TokenType::EndLine);
		self.restore_state(to_return, old_state);
		to_return
	}

	pub fn parse(
		to_parse: &String
	) -> Result<(Vec<ResourcePtr>, Vec<String>, Vec<TokenProcess>), KrpSimError> {
		// init parser struct
		let mut tokens = Parser::split_into_tokens(to_parse);
		tokens.push(Token::new(TokenType::EndLine, "\n".to_string()));
		let mut parser = Parser{
			index: 0,
			tokens: tokens,
			stack: BTreeMap::new(),
			resources: Vec::new(),
			optimize: Vec::new(),
			processes: Vec::new()
		};

		// test tokens against rules
		let mut carry_on = true;
		let mut num_line = 0;
		while carry_on && !parser.reached_end() {
			carry_on = parser.rule_empty_line() ||
					parser.rule_initial_stock() ||
					parser.rule_process() ||
					parser.rule_optimisation();
			num_line += 1;
		}
		let indexi = parser.index;

		// return value
		if carry_on {
		    Ok((parser.resources, parser.optimize, parser.processes))
		} else {
			Err(KrpSimError::ParseError(num_line))
		}
	}
}
