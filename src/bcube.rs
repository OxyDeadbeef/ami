// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017  Douglas P. Lau
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use std::fmt;

use Vec3;
use BBox;

/// Bounding cube
#[derive(Clone, Copy, PartialEq)]
pub struct BCube {
	pub(crate) center: Vec3,
	pub(crate) half_len: f32,
}

impl fmt::Debug for BCube {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}±{:?}", self.center, self.half_len)
	}
}

impl BCube {
	/// Create an new empty `BCube`.
	pub fn empty() -> BCube {
		let z = 0.0;

		BCube { center: Vec3::new(z, z, z), half_len: -1.0 }
	}

	/// Create an new `BCube` at position `p`.
	pub fn new(p: Vec3) -> BCube {
		BCube { center: p, half_len: 1.0 }
	}

	/// Extend the `BCube` to accommodate for `BBox`
	pub fn extend(&mut self, p: BBox) {
//		println!("Extend: {:?}", p);
		self.center = self.move_center(p);
		self.half_len *= 2.0;
	}

	fn move_center(&self, p: BBox) -> Vec3 {
		let (maxx, maxy, maxz) = p.bcube_sides(*self);

//		println!("MAX: {} {} {}", maxx, maxy, maxz);

		let min = self.center - self.half_len;
		let max = self.center + self.half_len;

		match (maxx, maxy, maxz) {
			(false, false, false) => Vec3::new(min.x, min.y, min.z),
			(false, false, true) => Vec3::new(min.x, min.y, max.z),
			(false, true, false) => Vec3::new(min.x, max.y, min.z),
			(false, true, true) => Vec3::new(min.x, max.y, max.z),
			(true, false, false) => Vec3::new(max.x, min.y, min.z),
			(true, false, true) => Vec3::new(max.x, min.y, max.z),
			(true, true, false) => Vec3::new(max.x, max.y, min.z),
			(true, true, true) => Vec3::new(max.x, max.y, max.z),
		}
	}

	/// Check if `BCube` contains point `p`.
	pub fn contains(&self, p: Vec3) -> bool {
		let Vec3 { x, y, z } = self.center;
		let hl = self.half_len;
		(p.x >= x - hl) &&
		(p.x < x + hl) &&
		(p.y >= y - hl) &&
		(p.y < y + hl) &&
		(p.z >= z - hl) &&
		(p.z < z + hl)
	}

	/// Get two opposite points that are the bounds of the BCube.
	pub fn to_point_pair(&self) -> (Vec3, Vec3) {
		let half_cube = Vec3::new(self.half_len, self.half_len,
			self.half_len);

		(self.center + half_cube, self.center - half_cube)
	}

	/// Turn into a bbox.
	pub fn to_bbox(&self) -> BBox {
		let (max, min) = self.to_point_pair();
		BBox::new(min, max)
	}

	/// Get all 6 points or the `BCube`.
	pub fn all_points(&self) -> [Vec3; 7] {
		let z = 0.0;

		[
			self.center,
			self.center + Vec3::new(self.half_len, z, z),
			self.center + Vec3::new(z, self.half_len, z),
			self.center + Vec3::new(z, z, self.half_len),
			self.center + Vec3::new(-self.half_len, z, z),
			self.center + Vec3::new(z, -self.half_len, z),
			self.center + Vec3::new(z, z, -self.half_len),
		]
	}

	/// Get a positive and negative pair of opposite points that are the
	/// bounds of the BCube, based around a normal.
	pub fn pn_pair_from_normal(&self, normal: Vec3)
		-> (Vec3, Vec3)
	{
		let mut pvertex = self.center;
		let mut nvertex = self.center;

		if normal.x >= 0.0 {
			pvertex.x += self.half_len;
			nvertex.x -= self.half_len;
		} else {
			nvertex.x += self.half_len;
			pvertex.x -= self.half_len;
		}

		if normal.y >= 0.0 {
			pvertex.y += self.half_len;
			nvertex.y -= self.half_len;
		} else {
			nvertex.y += self.half_len;
			pvertex.y -= self.half_len;
		}

		if normal.z >= 0.0 {
			pvertex.z += self.half_len;
			nvertex.z -= self.half_len;
		} else {
			nvertex.z += self.half_len;
			pvertex.z -= self.half_len;
		}

		(nvertex, pvertex)
	}
}
