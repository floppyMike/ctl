use std::ops::{Add, Sub};

// -----------------------------------------------------------------------------
// Point
// -----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T>
{
	pub x: T,
	pub y: T,
}

macro_rules! gen_point {
	(i => $T:ty) => {
		pub fn to_f32(self) -> Point<f32>
		{
			Point {
				x: self.x as f32,
				y: self.y as f32,
			}
		}
	};

	(f => $T:ty) => {
		pub fn to_i32(self) -> Point<i32>
		{
			Point {
				x: self.x as i32,
				y: self.y as i32,
			}
		}
	};

	($fp:ident $T:ty) => {
		impl Point<$T>
		{
			pub fn from_coords(x: $T, y: $T) -> Self
			{
				Point { x, y }
			}

            gen_point!($fp => $T);
		}

        impl Add for Point<$T>
        {
            type Output = Self;

            fn add(self, p: Self) -> Self
            {
                Self { x: self.x + p.x, y: self.y + p.y }
            }
        }

        impl Sub for Point<$T>
        {
            type Output = Self;

            fn sub(self, p: Self) -> Self
            {
                Self { x: self.x - p.x, y: self.y - p.y }
            }
        }
	};
}

gen_point!(i u8);
gen_point!(i u16);
gen_point!(i u32);
gen_point!(i u64);

gen_point!(i i8);
gen_point!(i i16);
gen_point!(i i32);
gen_point!(i i64);

gen_point!(f f32);
gen_point!(f f64);
