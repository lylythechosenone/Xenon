use lyon::path::path::BuilderWithAttributes;

use crate::{colors::Color, math::Point2D, rendering::Image};

pub use lyon::lyon_tessellation::{StrokeOptions, LineCap, LineJoin};

#[repr(transparent)]
pub struct ColorPath(pub(crate) lyon::path::Path);
impl ColorPath {
    pub fn build() -> ColorPathBuilder {
        ColorPathBuilder::new()
    }
}
pub struct TexturePath(pub(crate) lyon::path::Path, pub(crate) Image);
impl TexturePath {
    pub fn build(image: Image) -> TexturePathBuilder {
        TexturePathBuilder::new(image)
    }
}

pub struct ColorPathBuilder {
    builder: BuilderWithAttributes,
    color: Color,
}
impl ColorPathBuilder {
    pub fn new() -> Self {
        Self {
            builder: BuilderWithAttributes::new(4),
            color: Color::BLACK,
        }
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn begin(mut self, point: Point2D) -> Self {
        self.builder.begin(
            point,
            &[
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
                self.color.a as f32 / 255.0,
            ],
        );
        self
    }
    pub fn end(mut self, close: bool) -> Self {
        self.builder.end(close);
        self
    }
    pub fn line_to(mut self, point: Point2D) -> Self {
        self.builder.line_to(
            point,
            &[
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
                self.color.a as f32 / 255.0,
            ],
        );
        self
    }
    pub fn quadratic_bezier_to(mut self, ctrl: Point2D, point: Point2D) -> Self {
        self.builder.quadratic_bezier_to(
            ctrl,
            point,
            &[
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
                self.color.a as f32 / 255.0,
            ],
        );
        self
    }
    pub fn cubic_bezier_to(mut self, ctrl1: Point2D, ctrl2: Point2D, point: Point2D) -> Self {
        self.builder.cubic_bezier_to(
            ctrl1,
            ctrl2,
            point,
            &[
                self.color.r as f32 / 255.0,
                self.color.g as f32 / 255.0,
                self.color.b as f32 / 255.0,
                self.color.a as f32 / 255.0,
            ],
        );
        self
    }
    pub fn build(self) -> ColorPath {
        ColorPath(self.builder.build())
    }
}

pub struct TexturePathBuilder {
    builder: BuilderWithAttributes,
    texture: Image,
}
impl TexturePathBuilder {
    pub fn new(texture: Image) -> Self {
        Self {
            builder: BuilderWithAttributes::new(4),
            texture,
        }
    }
    pub fn begin(mut self, point: Point2D, uv: Point2D) -> Self {
        self.builder.begin(point, &[uv.x, uv.y]);
        self
    }
    pub fn end(mut self, close: bool) -> Self {
        self.builder.end(close);
        self
    }
    pub fn line_to(mut self, point: Point2D, uv: Point2D) -> Self {
        self.builder.line_to(point, &[uv.x, uv.y]);
        self
    }
    pub fn quadratic_bezier_to(mut self, ctrl: Point2D, point: Point2D, uv: Point2D) -> Self {
        self.builder.quadratic_bezier_to(ctrl, point, &[uv.x, uv.y]);
        self
    }
    pub fn cubic_bezier_to(mut self, ctrl1: Point2D, ctrl2: Point2D, point: Point2D, uv: Point2D) -> Self {
        self.builder
            .cubic_bezier_to(ctrl1, ctrl2, point, &[uv.x, uv.y]);
        self
    }
    pub fn build(self) -> TexturePath {
        TexturePath(self.builder.build(), self.texture)
    }
}
