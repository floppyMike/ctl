use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

// -----------------------------------------------------------------------------
// GCD
// -----------------------------------------------------------------------------

/// Calculates the gcd of 2 values
///
/// # Arguments
///
/// * `a` - First gcd integer parameter
/// * `b` - Second gcd integer parameter
///
/// # Examples
///
/// ```
/// use ctl::math::gcd;
/// let x = gcd(713, 552); // 23
/// ```
pub fn gcd(a: i32, b: i32) -> i32
{
	if b == 0 {
		return a;
	}

	gcd(b, a % b)
}

/// Extended version of the gcd algorithm. It also calculates s and t from gcd(a, b) = as + bt.
///
/// # Arguments
///
/// * `a` - First gcd integer parameter
/// * `b` - Second gcd integer parameter
///
/// # Examples
///
/// ```
/// use ctl::math::extended_gcd;
/// let x = extended_gcd(713, 552); // (23, 7, -9) <- (gcd(713, 552), s, t)
/// ```
pub fn extended_gcd(mut a: i32, mut b: i32) -> (i32, i32, i32)
{
	if b == 0 {
		return (a.abs(), 1, 0);
	}

	if a * a < b * b {
		(a, b) = (b, a);
	}

	let mut s = [1, 0];
	let mut t = [0, 1];

	loop {
		let q = a / b;
		let r = a % b;

		if r == 0 {
			return (b, s[1], t[1]);
		}

		let s3 = s[0] - q * s[1];
		let t3 = t[0] - q * t[1];

		s[0] = s[1];
		s[1] = s3;

		t[0] = t[1];
		t[1] = t3;

		a = b;
		b = r;
	}
}

// -----------------------------------------------------------------------------
// Fraction
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Fraction
{
	pub q: i32,
	pub d: i32,
}

/// Helper function to ease the creation of fractions.
///
/// # Arguments
///
/// * `a` - Top of fraction
/// * `b` - Bottom of fraction
///
/// # Examples
///
/// ```
/// use ctl::math::frac;
/// let a = frac(1, 2); // Creates: Fraction { q: 1, d: 2 }
/// ```
pub fn frac(a: i32, b: i32) -> Fraction
{
    Fraction { q: a, d: b }
}

impl Fraction
{
	/// Convert fraction to floating point representation.
	///
	/// # Examples
	///
	/// ```
	/// use ctl::math::Fraction;
	/// let f = Fraction { q: 1, d: 4 };
	/// let r = f.to_f64(); // Outputs .25
	/// ```
	pub fn to_f64(self) -> f64
	{
		return self.q as f64 / self.d as f64;
	}
}

impl Add for Fraction
{
	type Output = Fraction;

	fn add(self, rhs: Self) -> Self::Output
	{
		let a = Fraction {
			q: self.q * rhs.d,
			d: self.d * rhs.d,
		};
		let b = Fraction {
			q: rhs.q * self.d,
			d: rhs.d * self.d,
		};

		let r = Fraction {
			q: a.q + b.q,
			d: a.d,
		};
		let g = gcd(r.q, r.d);

		Fraction {
			q: r.q / g,
			d: r.d / g,
		}
	}
}

impl Sub for Fraction
{
	type Output = Fraction;

	fn sub(self, rhs: Self) -> Self::Output
	{
		let a = Fraction {
			q: self.q * rhs.d,
			d: self.d * rhs.d,
		};
		let b = Fraction {
			q: rhs.q * self.d,
			d: rhs.d * self.d,
		};

		let r = Fraction {
			q: a.q - b.q,
			d: a.d,
		};
		let g = gcd(r.q, r.d);

		Fraction {
			q: r.q / g,
			d: r.d / g,
		}
	}
}

impl Mul for Fraction
{
	type Output = Fraction;

	fn mul(self, rhs: Self) -> Self::Output
	{
		let r = Fraction {
			q: self.q * rhs.q,
			d: self.d * rhs.d,
		};

		let g = gcd(r.q, r.d);

		Fraction {
			q: r.q / g,
			d: r.d / g,
		}
	}
}

impl Div for Fraction
{
	type Output = Fraction;

	fn div(self, rhs: Self) -> Self::Output
	{
		let r = Fraction {
			q: self.q * rhs.d,
			d: self.d * rhs.q,
		};

		let g = gcd(r.q, r.d);

		Fraction {
			q: r.q / g,
			d: r.d / g,
		}
	}
}

impl PartialEq for Fraction
{
	fn eq(&self, other: &Self) -> bool
	{
		return self.q * other.d == self.d * other.q;
	}
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn test_gcd()
	{
		assert_eq!(gcd(713, 552), 23);
		assert_eq!(gcd(713, 0), 713);
		assert_eq!(gcd(552, 713), 23);
		assert_eq!(gcd(-552, -713), -23);
		assert_eq!(gcd(11253, 2607), 33);
		assert_eq!(gcd(-11253, 2607), -33);
	}

	#[test]
	fn test_extended_gcd()
	{
		assert_eq!(extended_gcd(161, 28), (7, -1, 6));
		assert_eq!(extended_gcd(713, 552), (23, 7, -9));
		assert_eq!(extended_gcd(713, 0), (713, 1, 0));
		assert_eq!(extended_gcd(552, 713), (23, 7, -9));
		assert_eq!(extended_gcd(-552, -713), (-23, 7, -9));
		assert_eq!(extended_gcd(11253, 2607), (33, 19, -82));
	}

	#[test]
	fn test_fractions()
	{
		let a = frac(1, 2);
		let b = frac(7, 3);
		let c = Fraction { q: 8, d: 9 };
		let d = Fraction { q: 23, d: 79 };
		let e = Fraction { q: -56, d: 2 };

		assert_eq!(a + a, Fraction { q: 2, d: 2 });
		assert_eq!(a - b, Fraction { q: -11, d: 6 });
		assert_eq!(d * c, Fraction { q: 184, d: 711 });
		assert_eq!(e / d, Fraction { q: -2212, d: 23 });
	}
}
