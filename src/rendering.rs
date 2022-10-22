use crate::{math::Box2D, widgets::Widget};
use image::{ImageBuffer, Rgba};
use log::debug;
use lyon::lyon_tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
    StrokeVertex, VertexBuffers,
};
use raw_window_handle::HasRawWindowHandle;
use winit::window::Window;

use crate::{colors::Color, math::Size2D, path::ColorPath};

mod wgpu;
pub use self::wgpu::*;

pub type Image = ImageBuffer<Rgba<f32>, Vec<f32>>;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ColorVertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct TextureVertex {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2],
    pub tex_index: u32,
}

pub enum FaceType {
    Textured(Image),
    Colored(Color),
}

/// A type that renders triangles and lines to a window.
pub trait Renderer {
    /// Render, clearing the render queue
    fn render(&mut self);
    /// Resize the window
    fn resize(&mut self, new_size: Size2D, scale_factor: f32);
    /// Configure the renderer for the given window
    fn new(window: &winit::window::Window) -> Self
    where
        Self: Sized;
    /// Add a shape to the render queue
    fn add_colored_object(&mut self, vertices: VertexBuffers<ColorVertex, u16>);
    /// Add a textured shape to the render queue
    fn add_textured_object(&mut self, vertices: VertexBuffers<TextureVertex, u16>);
    /// Register a texture. Returns the texture index.
    /// Implementors should be careful to ensure the texture isn't already registered.
    fn register_texture(&mut self, texture: Image) -> u32;
}

/// A `Renderer` that just `debug!`s everything.
pub struct DebugRenderer {
    colored_buffer: VertexBuffers<ColorVertex, u16>,
}
impl Renderer for DebugRenderer {
    fn render(&mut self) {
        debug!("Vertices: {:?}", self.colored_buffer.vertices);
        debug!("Indices: {:?}", self.colored_buffer.indices);
        self.colored_buffer.vertices.clear();
        self.colored_buffer.indices.clear();
    }

    fn resize(&mut self, new_size: Size2D, _scale_factor: f32) {
        debug!("Resizing to {:?}", new_size);
    }

    fn new(window: &Window) -> Self
    where
        Self: Sized,
    {
        debug!(
            "Setting up debug renderer for window with handle {:?}",
            window.raw_window_handle()
        );
        Self {
            colored_buffer: VertexBuffers::new(),
        }
    }

    fn add_colored_object(&mut self, mut vertices: VertexBuffers<ColorVertex, u16>) {
        debug!(
            "Adding colored object with vertices: {:?} and indices: {:?}",
            vertices.vertices, vertices.indices
        );
        self.colored_buffer.vertices.append(&mut vertices.vertices);
        self.colored_buffer.indices.append(&mut vertices.indices);
    }

    fn add_textured_object(&mut self, _vertices: VertexBuffers<TextureVertex, u16>) {
        todo!()
    }

    fn register_texture(&mut self, _texture: Image) -> u32 {
        todo!()
    }
}

/// A type that outputs basic shapes to a given `Renderer`.
pub struct Canvas<'a, Renderer: self::Renderer> {
    renderer: &'a mut Renderer,
    pub(crate) bounds: Box2D,
}
impl<'a, Renderer: self::Renderer> Canvas<'a, Renderer> {
    /// Create a new canvas from a renderer.
    pub(crate) fn new(renderer: &'a mut Renderer, bounds: Box2D) -> Self {
        Self {
            renderer,
            bounds,
        }
    }
    pub fn render<T: Widget>(&mut self, widget: &mut T, bounds: Box2D) {
        widget.render(Canvas::new(self.renderer, bounds))
    }
    /// Draw a colored filled path.
    pub fn fill_path(&mut self, path: ColorPath) {
        let mut geometry = VertexBuffers::new();
        let mut tesellator = FillTessellator::new();
        {
            tesellator
                .tessellate_path(
                    &path.0,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(&mut geometry, |mut vertex: FillVertex| ColorVertex {
                        pos: [
                            (vertex.position().x.max(0.0) + self.bounds.min.x)
                                .min(self.bounds.max.x),
                            (vertex.position().y.max(0.0) + self.bounds.min.y)
                                .min(self.bounds.max.y),
                        ],
                        color: [
                            vertex.interpolated_attributes()[0],
                            vertex.interpolated_attributes()[1],
                            vertex.interpolated_attributes()[2],
                            vertex.interpolated_attributes()[3],
                        ],
                    }),
                )
                .expect("Failed to tessellate path.");
        }
        self.renderer.add_colored_object(geometry);
    }
    /// Draw a colored stroked path
    pub fn stroke_path(&mut self, path: ColorPath, stroke: StrokeOptions) {
        let mut geometry = VertexBuffers::new();
        let mut tesellator = StrokeTessellator::new();
        {
            tesellator
                .tessellate_path(
                    &path.0,
                    &stroke,
                    &mut BuffersBuilder::new(&mut geometry, |mut vertex: StrokeVertex| {
                        ColorVertex {
                            pos: [
                                (vertex.position().x.max(0.0) + self.bounds.min.x)
                                    .min(self.bounds.max.x),
                                (vertex.position().y.max(0.0) + self.bounds.min.y)
                                    .min(self.bounds.max.y),
                            ],
                            color: [
                                vertex.interpolated_attributes()[0],
                                vertex.interpolated_attributes()[1],
                                vertex.interpolated_attributes()[2],
                                vertex.interpolated_attributes()[3],
                            ],
                        }
                    }),
                )
                .expect("Failed to tessellate path.");
        }
        self.renderer.add_colored_object(geometry);
    }
    pub fn fill_rect(&mut self, rect: Box2D, color: Color) {
        let mut geometry = VertexBuffers::new();
        let mut tesellator = FillTessellator::new();
        {
            tesellator
                .tessellate_rectangle(
                    &rect,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| ColorVertex {
                        pos: [
                            (vertex.position().x.max(0.0) + self.bounds.min.x)
                                .min(self.bounds.max.x),
                            (vertex.position().y.max(0.0) + self.bounds.min.y)
                                .min(self.bounds.max.y),
                        ],
                        color: [
                            color.r as f32 / 255.0,
                            color.g as f32 / 255.0,
                            color.b as f32 / 255.0,
                            color.a as f32 / 255.0,
                        ],
                    }),
                )
                .expect("Failed to tessellate path.");
        }
        self.renderer.add_colored_object(geometry);
    }
    pub fn stroke_rect(&mut self, rect: Box2D, color: Color, stroke: StrokeOptions) {
        let mut geometry = VertexBuffers::new();
        let mut tesellator = StrokeTessellator::new();
        {
            tesellator
                .tessellate_rectangle(
                    &rect,
                    &stroke,
                    &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| ColorVertex {
                        pos: [
                            (vertex.position().x.max(0.0) + self.bounds.min.x)
                                .min(self.bounds.max.x),
                            (vertex.position().y.max(0.0) + self.bounds.min.y)
                                .min(self.bounds.max.y),
                        ],
                        color: [
                            color.r as f32 / 255.0,
                            color.g as f32 / 255.0,
                            color.b as f32 / 255.0,
                            color.a as f32 / 255.0,
                        ],
                    }),
                )
                .expect("Failed to tessellate path.");
        }
        self.renderer.add_colored_object(geometry);
    }
}
