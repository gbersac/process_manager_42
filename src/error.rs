use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum KrpSimError {
	ParseError(usize),
}

impl Display for KrpSimError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		match *self {
			KrpSimError::ParseError(num_line)	=> {
				write!(f, "Error on line {}", num_line)
			},
		};
		Ok(())
	}
}

impl error::Error for KrpSimError {
	fn description(&self) -> &str {
		match *self {
		    KrpSimError::ParseError(_) => "The file is not correct",
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}
