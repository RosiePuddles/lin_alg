use std::{
	fmt::{Display, Formatter},
};
use itertools::Itertools;

/// Matrix struct
#[derive(Clone)]
pub struct Matrix {
	contents: Vec<Vec<f64>>,
	l: usize,
	w: usize
}

impl Matrix {
	/// Makes a new matrix from
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
	
	pub fn blank(l: usize, w: usize) -> Self {
		Matrix {
			contents: vec![vec![0.; w]; l],
			l, w
		}
	}
	
	pub fn lu_decompose(&self) -> Option<(Self, Self)> {
		if self.l != self.w {
			return None
		}
		let mut upper = self.clone();
		let mut lower = Matrix::identity(self.l);
		for r in 0..self.l {
			// Check if the element in position (r, r) is 0
			let scale = upper.contents[r][r];
			if scale == 0. {
				return None
			}
			// Scale the r-th row in the upper to give it a leading 1
			upper.contents[r] = upper.contents.get(r).unwrap().iter().map(|t| t / scale).collect();
			// Scale the r-th row in the lower (inverse of the previous ERO)
			lower.contents[r][r] = scale;
			// Add a scaled version of the r-th row to all the rows below it
			let short_row_upper = upper.contents[r][r + 1..].to_vec().clone();
			for below in r + 1..self.l {
				let scale = upper.contents[below][r];
				if scale == 0. {
					continue
				}
				let mut new_row = vec![0.; r + 1];
				new_row.extend(
					upper.contents[below][r + 1..].to_vec().iter().zip(short_row_upper.iter()).map(
						|(lower_item, r_row_item)| lower_item - scale * r_row_item
					)
				);
				upper.contents[below] = new_row;
				lower.contents[below][r] = scale
			}
		}
		Some((lower, upper))
	}
	
	pub fn plu_decomposition(&self) -> Option<(Self, Self, Self)> {
		if self.l != self.w {
			return None
		}
		let mut out = self.clone();
		if let Some((lower, upper)) = out.lu_decompose() {
			return Some((lower, upper, Matrix::identity(self.l)))
		}
		for mut t in (0..self.w).combinations(2) {
			let mut permutation = Matrix::identity(self.l);
			let row = permutation.contents.get(*t.first().unwrap()).unwrap().clone();
			let row2 = permutation.contents.get(*t.last().unwrap()).unwrap().clone();
			permutation.contents[*t.last().unwrap()] = row;
			permutation.contents[*t.first().unwrap()] = row2;
			
			let row = out.contents.get(*t.first().unwrap()).unwrap().clone();
			let row2 = out.contents.get(*t.last().unwrap()).unwrap().clone();
			out.contents[*t.last().unwrap()] = row;
			out.contents[*t.first().unwrap()] = row2;
			if let Some((lower, upper)) = out.lu_decompose() {
				return Some((lower, upper, permutation))
			}
		}
		None
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

pub enum ERO {
	/// Scale the first row by the second value
	Scale(usize, f64),
	/// Add the scaled second row (by the third value) to the first row
	Add(usize, usize, f64),
	/// Switch the two rows
	Switch(usize, usize)
}
