//! Decomposition implementations for the `matrix` struct

use itertools::Itertools;
use crate::core::Matrix;

impl Matrix {
	/// Calculates the LU decomposition of a matrix
	///
	/// Returns `Option::None` is the LU decomposition cannot be found
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
