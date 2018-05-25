// "ami" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use Vec4;
use Vec3;
use Plane;
use Frustum;

/// A no-op transform (identity matrix).
pub const IDENTITY: Mat4 = Mat4([1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
	0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,]);

/// A 4x4 Matrix
#[derive(Clone, Copy, PartialEq)]
pub struct Mat4(pub [f64; 16]);

impl Mat4 {
	/// Multiply `self` by a matrix.
	pub fn matrix(self, matrix: [f64; 16]) -> Mat4 {
		self * Mat4(matrix)
	}

	/// Multiply `self` by a scale transformation matrix.
	pub fn scale<T: Into<f64>>(self, x: T, y: T, z: T) -> Mat4 {
		self.matrix([
			x.into(), 0.0, 0.0, 0.0,
			0.0, y.into(), 0.0, 0.0,
			0.0, 0.0, z.into(), 0.0,
			0.0, 0.0, 0.0, 1.0,
		])
	}

	/// Multiply `self` by a translation matrix.
	pub fn translate<T: Into<f64>>(self, x: T, y: T, z: T) -> Mat4 {
		self.matrix([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			x.into(), y.into(), z.into(), 1.0,
		])
	}

	/// Multiply `self` by a rotation matrix.  `x`, `y` and `z` are in PI
	/// Radians.
	pub fn rotate<T: Into<f64>>(self, x: T, y: T, z: T) -> Mat4 {
		let num9 = z.into() * ::std::f64::consts::PI;
		let num6 = num9.sin();
		let num5 = num9.cos();
		let num8 = x.into() * ::std::f64::consts::PI;
		let num4 = num8.sin();
		let num3 = num8.cos();
		let num7 = y.into() * ::std::f64::consts::PI;
		let num2 = num7.sin();
		let num = num7.cos();

		let qx = ((num * num4) * num5) + ((num2 * num3) * num6);
		let qy = ((num2 * num3) * num5) - ((num * num4) * num6);
		let qz = ((num * num3) * num6) - ((num2 * num4) * num5);
		let qw = ((num * num3) * num5) + ((num2 * num4) * num6);

		let nx = -qx;
		let ny = -qy;
		let nz = -qz;

		self.matrix([
			qw,nz,qy,nx,
			qz,qw,nx,ny,
			ny,qx,qw,nz,
			qx,qy,qz,qw
		]).matrix([
			qw,nz,qy,qx,
			qz,qw,nx,qy,
			ny,qx,qw,qz,
			nx,ny,nz,qw
		])
	}

	/// Convert into an array of f32s
	pub fn to_f32_array(&self) -> [f32; 16] {
		[
			self.0[0]as f32,self.0[1]as f32,self.0[2]as f32,self.0[3]as f32,
			self.0[4]as f32,self.0[5]as f32,self.0[6]as f32,self.0[7]as f32,
			self.0[8]as f32,self.0[9]as f32,self.0[10]as f32,self.0[11]as f32,
			self.0[12]as f32,self.0[13]as f32,self.0[14]as f32,self.0[15]as f32,
		]
	}
}

impl ::std::ops::Mul<Frustum> for Mat4 {
	type Output = Frustum;

	fn mul(self, rhs: Frustum) -> Self::Output {
		Frustum {
			center: self * rhs.center,
			radius: rhs.radius,
			wfov: rhs.wfov,
			hfov: rhs.hfov,
			xrot: rhs.xrot, // TODO
			yrot: rhs.yrot, // TODO
//			near: self * rhs.near,
//			far: self * rhs.far,
//			top: self * rhs.top,
//			bottom: self * rhs.bottom,
//			right: self * rhs.right,
//			left: self * rhs.left,
		}
	}
}

impl ::std::ops::Mul<Plane> for Mat4 {
	type Output = Plane;

	fn mul(self, rhs: Plane) -> Self::Output {
		let mat = self.to_f32_array();

		let facing = rhs.facing.transform_dir(self);
		// translation vector
		let t = Vec3::new(mat[12], mat[13], mat[14]);
		//
		if t == Vec3::zero() {
			return Plane { facing, offset: rhs.offset };
		}
		// Angle between normal and translation
		let mut a = facing.angle(t).abs();

		// If more than full circle, reduce
		while a > ::std::f32::consts::PI * 2.0 {
			a -= ::std::f32::consts::PI * 2.0;
		}

		let mut b = 1.0;

		// If value is over 90° it can be reduced
		if a > ::std::f32::consts::PI / 2.0 {
			// 90°-180° quadrant
			if a < ::std::f32::consts::PI {
				a = ::std::f32::consts::PI - a;
				b = -b;
			// 180°-270° quadrant
			} else if a < 3.0 * ::std::f32::consts::PI / 2.0 {
				a -= ::std::f32::consts::PI;
				b = -b;
			// 270°-360° quadrant
			} else {
				a = (2.0 * ::std::f32::consts::PI) - a;
			}
		}

		// if a == 90°
		let offset = rhs.offset + if a >= ::std::f32::consts::PI / 2.0 {
			0.0
		} else {
			a.cos() * t.mag() * b
		};

		Plane { facing, offset }
	}
}

impl ::std::ops::Mul<Vec3> for Mat4 {
	type Output = Vec3;

	/// Transform as a position.
	fn mul(self, rhs: Vec3) -> Self::Output {
		let mat = self.to_f32_array();

		let x = mat[0]*rhs.x + mat[4]*rhs.y + mat[8]*rhs.z + mat[12]*1.0;
		let y = mat[1]*rhs.x + mat[5]*rhs.y + mat[9]*rhs.z + mat[13]*1.0;
		let z = mat[2]*rhs.x + mat[6]*rhs.y + mat[10]*rhs.z + mat[14]*1.0;

		Vec3::new(x, y, z)
	}
}

impl ::std::ops::Mul<Vec4> for Mat4 {
	type Output = Vec4;

	/// Transform as a position.
	fn mul(self, rhs: Vec4) -> Self::Output {
		let mat = self.to_f32_array();

		let x = mat[0]*rhs.x + mat[4]*rhs.y + mat[8]*rhs.z + mat[12]*rhs.w;
		let y = mat[1]*rhs.x + mat[5]*rhs.y + mat[9]*rhs.z + mat[13]*rhs.w;
		let z = mat[2]*rhs.x + mat[6]*rhs.y + mat[10]*rhs.z + mat[14]*rhs.w;
		let w = mat[3]*rhs.x + mat[7]*rhs.y + mat[11]*rhs.z + mat[15]*rhs.w;

		Vec4::new(x, y, z, w)
	}
}

impl ::std::ops::Mul<Mat4> for Mat4 {
	type Output = Mat4;

	fn mul(self, rhs: Mat4) -> Self::Output {
		Mat4([
			(self.0[0] * rhs.0[0]) + (self.0[1] * rhs.0[4]) +
			(self.0[2] * rhs.0[8]) + (self.0[3] * rhs.0[12]),
			(self.0[0] * rhs.0[1]) + (self.0[1] * rhs.0[5]) +
			(self.0[2] * rhs.0[9]) + (self.0[3] * rhs.0[13]),
			(self.0[0] * rhs.0[2]) + (self.0[1] * rhs.0[6]) +
			(self.0[2] * rhs.0[10]) + (self.0[3] * rhs.0[14]),
			(self.0[0] * rhs.0[3]) + (self.0[1] * rhs.0[7]) +
			(self.0[2] * rhs.0[11]) + (self.0[3] * rhs.0[15]),

			(self.0[4] * rhs.0[0]) + (self.0[5] * rhs.0[4]) +
			(self.0[6] * rhs.0[8]) + (self.0[7] * rhs.0[12]),
			(self.0[4] * rhs.0[1]) + (self.0[5] * rhs.0[5]) +
			(self.0[6] * rhs.0[9]) + (self.0[7] * rhs.0[13]),
			(self.0[4] * rhs.0[2]) + (self.0[5] * rhs.0[6]) +
			(self.0[6] * rhs.0[10]) + (self.0[7] * rhs.0[14]),
			(self.0[4] * rhs.0[3]) + (self.0[5] * rhs.0[7]) +
			(self.0[6] * rhs.0[11]) + (self.0[7] * rhs.0[15]),

			(self.0[8] * rhs.0[0]) + (self.0[9] * rhs.0[4]) +
			(self.0[10] * rhs.0[8]) + (self.0[11] * rhs.0[12]),
			(self.0[8] * rhs.0[1]) + (self.0[9] * rhs.0[5]) +
			(self.0[10] * rhs.0[9]) + (self.0[11] * rhs.0[13]),
			(self.0[8] * rhs.0[2]) + (self.0[9] * rhs.0[6]) +
			(self.0[10] * rhs.0[10]) + (self.0[11] * rhs.0[14]),
			(self.0[8] * rhs.0[3]) + (self.0[9] * rhs.0[7]) +
			(self.0[10] * rhs.0[11]) + (self.0[11] * rhs.0[15]),

			(self.0[12] * rhs.0[0]) + (self.0[13] * rhs.0[4]) +
			(self.0[14] * rhs.0[8]) + (self.0[15] * rhs.0[12]),
			(self.0[12] * rhs.0[1]) + (self.0[13] * rhs.0[5]) +
			(self.0[14] * rhs.0[9]) + (self.0[15] * rhs.0[13]),
			(self.0[12] * rhs.0[2]) + (self.0[13] * rhs.0[6]) +
			(self.0[14] * rhs.0[10]) + (self.0[15] * rhs.0[14]),
			(self.0[12] * rhs.0[3]) + (self.0[13] * rhs.0[7]) +
			(self.0[14] * rhs.0[11]) + (self.0[15] * rhs.0[15])
		])
	}
}

impl ::std::fmt::Display for Mat4 {
	fn fmt(&self, fmtr: &mut ::std::fmt::Formatter) ->
		::std::result::Result<(), ::std::fmt::Error>
	{
		write!(fmtr, "{:?}", self.0)
	}
}
