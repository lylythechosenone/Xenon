use crate::colors::Color;
use crate::vectors::Point2;

// TODO: implement `RenderState` (should be a wgpu render state)
pub struct RenderState {
    renderer: Option<&'static str>,
}
impl RenderState {
    pub fn new() -> Self {
        Self {
            renderer: None,
        }
    }
}

pub trait Renderer {
    /// Creates a new `Renderer`.
    fn new(name: &'static str) -> Self
    where
        Self: Sized;
    /// Configures the given `RenderState` to use this `Renderer`.
    fn configure(&mut self, state: &mut RenderState);
    /// Creates a new `Renderer` and sets up the given `RenderState` to work with it.
    fn with_state(name: &'static str, state: &mut RenderState) -> Self
    where
        Self: Sized,
    {
        let mut renderer = Self::new(name);
        renderer.configure(state);
        renderer
    }
    /// Returns this `Renderer`'s name.
    fn name(&self) -> &'static str;
    /// Renders to the given `RenderState`.
    fn render(&mut self, state: &mut RenderState) {
        #[cfg(debug_assertions)]
        if self.name() != state.renderer {
            panic!(
                "Renderer mismatch: expected {}, got {}",
                state.renderer,
                self.name()
            );
        }
        // Safe because we know this is the correct RenderState.
        unsafe {
            self.render_unchecked(state);
        }
    }
    /// Renders to the given `RenderState`,
    /// without checking if the `RenderState` belongs to this `Renderer`.
    /// # Safety
    /// Safe as long as the `RenderState` belongs to this `Renderer`.
    /// Often pointless to call, as `render` is unchecked on release builds anyway.
    unsafe fn render_unchecked(&mut self, state: &mut RenderState);
    /// Draws a line from `start` to `end` with the given `color`.
    fn line(&mut self, start: Point2, end: Point2, color: Color);
    /// Draws an arc with the given `color` and `radius` from `start` to `end`.
    /// Radius 0 is a straight line, radius (start - end).length() / 2 is round.
    fn arc(&mut self, start: Point2, end: Point2, radius: f32, color: Color);
    /// Draws an ellipse with the given `stroke`, `fill` and `radius` at `center`.
    fn ellipse(&mut self, center: Point2, radius: f32, stroke: (Color, f32), fill: Color);
    /// Draws a rectangle from `start` to `end` with the given `stroke`, `fill` and `radius`.
    fn rect(&mut self, start: Point2, end: Point2, radius: f32, stroke: (Color, f32), fill: Color);
    /// Draws a rounded rectangle from `start` to `end` with the given `stroke`, `fill` and `radius`.
    fn rounded_rect(
        &mut self,
        start: Point2,
        end: Point2,
        radius: f32,
        stroke: (Color, f32),
        fill: Color,
    );
    /// Draws a polygon with the given `stroke`, `fill` and `points`.
    fn polygon(&mut self, points: &[Point2], stroke: (Color, f32), fill: Color);
}
