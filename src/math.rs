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
pub fn gcd(mut a: i32, mut b: i32) -> i32
{
	if a.abs() < b.abs() {
		(a, b) = (b, a);
	}

	if b == 0 {
		return a;
	}

	loop {
		let r = a % b;

		if r == 0 {
			return b;
		}

		a = b;
		b = r;
	}
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
		assert_eq!(gcd(11253, 2607), 33)
	}

	#[test]
	fn test_extended_gcd()
	{
		assert_eq!(extended_gcd(161, 28), (7, -1, 6));
		assert_eq!(extended_gcd(713, 552), (23, 7, -9));
		assert_eq!(extended_gcd(713, 0), (713, 1, 0));
		assert_eq!(extended_gcd(552, 713), (23, 7, -9));
		assert_eq!(extended_gcd(-552, -713), (-23, 7, -9));
		assert_eq!(extended_gcd(11253, 2607), (33, 19, -82))
	}
}
