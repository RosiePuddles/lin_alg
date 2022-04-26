//! Core structs for the crate

use std::{
	fmt::{Display, Formatter},
};

/// Matrix struct
#[derive(Clone)]
pub struct Matrix {
	/// Holds the actual contents of the matrix
	pub(crate) contents: Vec<Vec<f64>>,
	/// Height of the matrix
	pub(crate) l: usize,
	/// Width of the matrix
	pub(crate) w: usize
}

impl Matrix {
	/// Makes a new matrix from a `Vec<Vec<f64>>`
	///
	/// If the inner vectors are not all the same size, then `Option::None` is returned
	pub fn new(mut inner: Vec<Vec<f64>>) -> Option<Self> {
		let l = inner.len();
		let w;
		let mut contents = vec![];
		if let Some(t) = inner.first() {
			w = t.len();
		} else {
			return None
		}
		for t in inner {
			if t.len() != w {
				return None
			}
			contents.push(t);
		}
		Some(Matrix {
			contents, l, w
		})
	}
	
	/// Generates an identity matrix for a given size
	pub fn identity(n: usize) -> Self {
		Matrix {
			contents: (0..n).fold(vec![], |mut acc, arg| {
				let mut temp = vec![0.; n];
				temp[arg] = 1.;
				acc.push(temp);
				acc
			}),
			l: n, w: n
		}
	}
	
	/// Generates a matrix contained with 0s of a given size
	pub fn blank(l: usize, w: usize) -> Self {
		Matrix {
			contents: vec![vec![0.; w]; l],
			l, w
		}
	}
}

impl Display for Matrix {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let t = f.precision().unwrap_or(10);
		write!(f, "{}", self.contents.iter().map(
			|r| format!("|{}|",
						r.iter().map(|i| format!("{}{:.t$}", if i < &0. { '-' } else { ' ' }, i.abs())).collect::<Vec<String>>().join(" ,")
			)
		).collect::<Vec<String>>().join("\n"))
	}
}

impl std::ops::Add<Matrix> for Matrix {
	type Output = Matrix;
	
	/// Add two matrices
	///
	/// Will fail if the matrices are not the same size
	fn add(self, rhs: Matrix) -> Self::Output {
		assert_eq!(self.l, rhs.l);
		assert_eq!(self.w, rhs.w);
		let mut out = self.clone();
		for i in 0..self.l {
			for j in 0..self.w {
				out.contents[i][j] += rhs.contents[i][j]
			}
		}
		out
	}
}

impl std::ops::Mul<Matrix> for Matrix {
	type Output = Matrix;
	
	/// Multiply two matrices
	///
	/// Will fail if the width of the first matrix is not equal to the height of the second
	fn mul(self, rhs: Matrix) -> Self::Output {
		assert_eq!(self.w, rhs.l);
		let mut out = Matrix::blank(self.l, rhs.w);
		for i in 0..self.l {
			for j in 0..rhs.w {
				out.contents[i][j] = (0..self.w).fold(
					0., |acc, t| acc + self.contents[i][t] * rhs.contents[t][j]
				)
			}
		}
		out
	}
}

impl<T> std::ops::Mul<T> for Matrix where
	T: Into<f64>
{
	type Output = Matrix;
	
	fn mul(self, rhs: T) -> Self::Output {
		let rhs: f64 = rhs.into();
		let mut out = self.clone();
		for i in 0..self.l {
			for j in 0..self.w {
				out.contents[i][j] *= rhs;
			}
		}
		out
	}
}

impl<T> std::ops::Div<T> for Matrix where
	T: Into<f64>
{
	type Output = Matrix;
	
	fn div(self, rhs: T) -> Self::Output {
		let rhs: f64 = rhs.into();
		self * (1. / rhs)
	}
}
