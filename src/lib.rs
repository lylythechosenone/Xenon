pub mod colors;
pub mod input;
pub mod rendering;
pub mod window;
pub mod math {
    pub use euclid::*;
    pub type Point2D = euclid::Point2D<f32, UnknownUnit>;
    pub type Box2D = euclid::Box2D<f32, UnknownUnit>;
    pub type Size2D = euclid::Size2D<f32, UnknownUnit>;
    pub type USize2D = euclid::Size2D<u32, UnknownUnit>;
}
pub mod path;
pub mod widgets;
