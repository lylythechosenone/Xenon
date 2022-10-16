macro_rules! vector_impl {
    ($ident:ident) => {
        impl<T: Vector2> std::ops::Add<T> for $ident {
            type Output = Self;
            fn add(mut self, other: T) -> Self {
                self += other;
                self
            }
        }
        impl<T: Vector2> std::ops::AddAssign<T> for $ident {
            fn add_assign(&mut self, other: T) {
                self.x += other.x();
                self.y += other.y();
            }
        }
        impl<T: Vector2> std::ops::Sub<T> for $ident {
            type Output = Self;
            fn sub(mut self, other: T) -> Self {
                self -= other;
                self
            }
        }
        impl<T: Vector2> std::ops::SubAssign<T> for $ident {
            fn sub_assign(&mut self, other: T) {
                self.x -= other.x();
                self.y -= other.y();
            }
        }
        impl<T: Vector2> std::ops::Mul<T> for $ident {
            type Output = Self;
            fn mul(mut self, other: T) -> Self {
                self *= other;
                self
            }
        }
        impl<T: Vector2> std::ops::MulAssign<T> for $ident {
            fn mul_assign(&mut self, other: T) {
                self.x *= other.x();
                self.y *= other.y();
            }
        }
        impl<T: Vector2> std::ops::Div<T> for $ident {
            type Output = Self;
            fn div(mut self, other: T) -> Self {
                self /= other;
                self
            }
        }
        impl<T: Vector2> std::ops::DivAssign<T> for $ident {
            fn div_assign(&mut self, other: T) {
                self.x /= other.x();
                self.y /= other.y();
            }
        }
        impl std::ops::Mul<f32> for $ident {
            type Output = Self;
            fn mul(mut self, other: f32) -> Self {
                self *= other;
                self
            }
        }
        impl std::ops::MulAssign<f32> for $ident {
            fn mul_assign(&mut self, other: f32) {
                self.x *= other;
                self.y *= other;
            }
        }
        impl std::ops::Div<f32> for $ident {
            type Output = Self;
            fn div(mut self, other: f32) -> Self {
                self /= other;
                self
            }
        }
        impl std::ops::DivAssign<f32> for $ident {
            fn div_assign(&mut self, other: f32) {
                self.x /= other;
                self.y /= other;
            }
        }
        impl std::ops::Neg for $ident {
            type Output = Self;
            fn neg(mut self) -> Self {
                self *= -1.0;
                self
            }
        }
        impl $ident {
            pub const fn new(x: f32, y: f32) -> Self {
                Self {
                    x, y
                }
            }
            pub fn length(&self) -> f32 {
                self.length_squared().sqrt()
            }
            pub fn length_squared(&self) -> f32 {
                self.dot(self.clone())
            }
            pub fn dot(&self, other: impl Vector2) -> f32 {
                self.x * other.x() + self.y * other.y()
            }
            pub fn min(&self, other: Self) -> Self {
                if *self < other {
                    self.clone()
                } else {
                    other
                }
            }
            pub fn max(&self, other: Self) -> Self {
                if *self > other {
                    self.clone()
                } else {
                    other
                }
            }
            pub fn lerp(&self, other: Self, t: f32) -> Self {
                self.clone() + (self.clone() - other) * t
            }
        }
        impl Vector2 for $ident {
            fn x(&self) -> f32 {
                self.x
            }
            fn y(&self) -> f32 {
                self.y
            }
        }
    };
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}
vector_impl!(Point2);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Extent2 {
    pub x: f32,
    pub y: f32,
}
vector_impl!(Extent2);
impl Extent2 {
    pub fn as_point(self) -> Point2 {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }
}

pub trait Vector2 {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}
