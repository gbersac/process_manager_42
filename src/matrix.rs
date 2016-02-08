#[derive(PartialEq)]
pub struct Matrix {
	width: usize,
	height: usize,

	/// This is a row first matrix.
	/// First come width cases for row 0, then width cases for row 2...
    cases: Vec<i32>
}

impl Matrix {
    /// Create a matrix where every case is set to 0.
	pub fn new(width: usize, height: usize) -> Matrix
	{
		Matrix {
			width: width,
			height: height,
			cases: vec![0; (width * height)]
		}
	}

	fn to_mat_index(&self, x: usize, y: usize) -> usize {
	    self.width * y + x
	}

	pub fn set(&mut self, x: usize, y: usize, value: i32){
	    let index = self.to_mat_index(x, y);
	    let mut r = self.cases.get_mut(index).unwrap();
	    *r = value;
	}

	pub fn get(&self, x: usize, y: usize) -> i32 {
	    *self.cases.get(self.to_mat_index(x, y)).unwrap()
	}

    pub fn from_vec(width: usize, height: usize, v: Vec<i32>) -> Matrix {
        if v.len() != width * height {
            panic!("wrong vector size");
        }

        let mut v = v.clone();
        let mut to_return = Matrix::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let value = v.remove(0);
                to_return.set(x, y, value);
            }
        }
        to_return
    }

    /// Return the column of index `i_col`.
    pub fn get_col(&self, i_col: usize) -> Vec<i32> {
        let mut to_return = Vec::with_capacity(self.height);
        for i in 0..self.height {
            let value = self.cases[i * self.width + i_col];
            to_return.push(value);
        }
        to_return
    }
}

use std::fmt::{Formatter, Debug, Error};

impl Debug for Matrix
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{} ", self.get(x, y));
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
