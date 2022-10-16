macro_rules! vector_impl {
    ($ident:ident, $($component:ident),*) => {
        impl std::ops::Add for $ident {
            type Output = Self;
            fn add(mut self, other: Self) -> Self {
                self += other;
                self
            }
        }
        impl std::ops::AddAssign for $ident {
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    $($component: self.$component + other.$component),*
                };
            }
        }
        impl std::ops::Sub for $ident {
            type Output = Self;
            fn sub(mut self, other: Self) -> Self {
                self -= other;
                self
            }
        }
        impl std::ops::SubAssign for $ident {
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    $($component: self.$component - other.$component),*
                };
            }
        }
        impl std::ops::Mul for $ident {
            type Output = Self;
            fn mul(mut self, other: Self) -> Self {
                self *= other;
                self
            }
        }
        impl std::ops::MulAssign for $ident {
            fn mul_assign(&mut self, other: Self) {
                *self = Self {
                    $($component: self.$component * other.$component),*
                };
            }
        }
        impl std::ops::Div for $ident {
            type Output = Self;
            fn div(mut self, other: Self) -> Self {
                self /= other;
                self
            }
        }
        impl std::ops::DivAssign for $ident {
            fn div_assign(&mut self, other: Self) {
                *self = Self {
                    $($component: self.$component / other.$component),*
                };
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
                *self = Self {
                    $($component: self.$component * other),*
                };
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
                *self = Self {
                    $($component: self.$component / other),*
                };
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
            pub const fn new($($component: f32),*) -> Self {
                Self {
                    $($component),*
                }
            }
            pub fn length(&self) -> f32 {
                self.length_squared().sqrt()
            }
            pub fn length_squared(&self) -> f32 {
                self.dot(self.clone())
            }
            pub fn dot(&self, other: Self) -> f32 {
                0.0 $(+ self.$component * other.$component)*
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}
vector_impl!(Point2, x, y);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
vector_impl!(Vector2, x, y);
