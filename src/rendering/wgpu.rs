use log::debug;
use lyon::lyon_tessellation::VertexBuffers;
use wgpu::util::DeviceExt;

use crate::math::Size2D;

use super::{ColorVertex, Image, Renderer};

pub struct WgpuRenderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    color_pipeline: wgpu::RenderPipeline,
    size_buffer: wgpu::Buffer,
    scale_factor_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    color_buffer: VertexBuffers<ColorVertex, u16>,
    scale_factor: f32,
}
impl Renderer for WgpuRenderer {
    fn render(&mut self) {
        if self.color_buffer.vertices.is_empty() {
            return;
        }
        debug!(
            "Rendering {:?} vertices with {:?} indices",
            self.color_buffer.vertices, self.color_buffer.indices
        );
        let color_vertex_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Color vertex buffer"),
                    contents: bytemuck::cast_slice(self.color_buffer.vertices.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let color_index_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Color vertex buffer"),
                    contents: bytemuck::cast_slice(self.color_buffer.indices.as_slice()),
                    usage: wgpu::BufferUsages::INDEX,
                });
        let output = self
            .surface
            .get_current_texture()
            .expect("Failed to get current texture");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Color render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.color_pipeline);
            render_pass.set_vertex_buffer(0, color_vertex_buffer.slice(..));
            render_pass.set_index_buffer(color_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw_indexed(0..self.color_buffer.indices.len() as u32, 0, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.color_buffer.vertices.clear();
        self.color_buffer.indices.clear();
    }

    fn resize(&mut self, new_size: Size2D, scale_factor: f32) {
        self.scale_factor = scale_factor;
        self.config.width = (new_size.width * self.scale_factor).round() as u32;
        self.config.height = (new_size.height * self.scale_factor).round() as u32;
        self.surface.configure(&self.device, &self.config);
        self.queue.write_buffer(
            &self.size_buffer,
            0,
            bytemuck::cast_slice(&[new_size.width, new_size.height]),
        );
        self.queue.write_buffer(
            &self.scale_factor_buffer,
            0,
            bytemuck::cast_slice(&[self.scale_factor]),
        );
    }

    fn new(window: &winit::window::Window) -> Self
    where
        Self: Sized,
    {
        let size = window.inner_size();
        let size = Size2D::new(size.width as f32, size.height as f32);
        let scale_factor = window.scale_factor() as f32;

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ))
        .unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width as u32,
            height: size.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
        };
        surface.configure(&device, &config);
        let size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Size buffer"),
            contents: bytemuck::cast_slice(&[size.width, size.height]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let scale_factor_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scale factor buffer"),
            contents: bytemuck::cast_slice(&[scale_factor]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: size_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: scale_factor_buffer.as_entire_binding(),
                },
            ],
        });
        let color_shader = device.create_shader_module(wgpu::include_wgsl!("wgpu/colored.wgsl"));
        let color_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Colored render pipeline layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });
        let color_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Colored render pipeline"),
            layout: Some(&color_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &color_shader,
                entry_point: "vertex",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<ColorVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 8,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &color_shader,
                entry_point: "fragment",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        Self {
            surface,
            device,
            queue,
            config,
            color_pipeline,
            size_buffer,
            scale_factor_buffer,
            uniform_bind_group,
            color_buffer: VertexBuffers::new(),
            scale_factor,
        }
    }

    fn add_colored_object(&mut self, mut buffer: VertexBuffers<super::ColorVertex, u16>) {
        self.color_buffer.vertices.append(&mut buffer.vertices);
        self.color_buffer.indices.append(&mut buffer.indices);
    }

    fn add_textured_object(&mut self, _vertices: VertexBuffers<super::TextureVertex, u16>) {
        todo!("Textured objects are not yet supported")
    }

    fn register_texture(&mut self, _texture: Image) -> u32 {
        todo!("Textured objects are not yet supported")
    }
}

#[cfg(test)]
mod tests {
    use crate::{colors::Color, widgets::Rectangle, window::Window};

    use super::*;

    #[test]
    fn test_renderer() {
        env_logger::init();
        let window = Window::<WgpuRenderer, _>::new(
            "WGPU renderer test",
            Rectangle {
                color: Color::WHITE,
                max_size: Size2D::new(10.0, 10.0),
            },
        );
        window.run();
    }
}
