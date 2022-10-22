use crate::input::{MouseButton, VirtualKeyCode};

use crate::colors::Color;
use crate::math::{Point2D, Size2D};
use crate::{input::Input, math::Box2D, rendering::Canvas};

pub trait Widget {
    fn size(&self, max: Size2D) -> Size2D { max }
    fn resize(&self, new_size: Size2D) -> Size2D { new_size }
    fn render<Renderer: crate::rendering::Renderer>(
            &mut self,
            canvas: Canvas<'_, Renderer>,
            );
    fn update(&mut self, _input: &Input) -> bool { false }
    fn focus(&mut self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct Rectangle {
    pub color: Color,
    pub max_size: Size2D,
}
impl Widget for Rectangle {
    fn size(&self, _max: Size2D) -> Size2D {
        self.max_size
    }
    fn resize(&self, _new_size: Size2D) -> Size2D {
        self.max_size
    }
    fn render<Renderer: crate::rendering::Renderer>(
        &mut self,
        mut canvas: Canvas<'_, Renderer>,
    ) {
        canvas.fill_rect(
            Box2D::new(
                Point2D::new(0.0, 0.0),
                Point2D::new(
                    self.max_size.width,
                    self.max_size.height,
                ),
            ),
            self.color.clone(),
        );
    }
}

#[derive(Default)]
/// A rectangle that changes size, shape and color when space or lmb are clicked.
pub struct RandomRectangle {
    rectangle: Rectangle,
    max_size: Size2D,
}
impl RandomRectangle {
    pub fn new(max_size: Size2D) -> Self {
        RandomRectangle {
            rectangle: Rectangle {
                color: Color::rgba(
                    fastrand::u8(0..255),
                    fastrand::u8(0..255),
                    fastrand::u8(0..255),
                    255,
                ),
                max_size: Size2D::new(
                    fastrand::f32() * max_size.width,
                    fastrand::f32() * max_size.height,
                ),
            },
            max_size,
        }
    }
}
impl Widget for RandomRectangle {
    fn size(&self, _max: Size2D) -> Size2D {
        self.rectangle.max_size
    }
    fn resize(&self, _new_size: Size2D) -> Size2D {
        self.rectangle.max_size
    }
    fn render<Renderer: crate::rendering::Renderer>(
        &mut self,
        canvas: Canvas<'_, Renderer>,
    ) {
        self.rectangle.render(canvas);
    }
    fn update(&mut self, input: &Input) -> bool {
        if input.is_pressed(VirtualKeyCode::Space)
            || input.is_mouse_clicked(MouseButton::Left).is_some()
        {
            self.rectangle.max_size = Size2D::new(
                fastrand::f32() * self.max_size.width,
                fastrand::f32() * self.max_size.height,
            );
            self.rectangle.color = Color::rgba(
                fastrand::u8(0..255),
                fastrand::u8(0..255),
                fastrand::u8(0..255),
                255,
            );
            true
        } else {
            false
        }
    }
}
