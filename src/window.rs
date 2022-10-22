use crate::{
    input::Input,
    math::{Box2D, Point2D, Size2D},
    rendering::Canvas,
    widgets::Widget,
};
use log::{debug, trace};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, KeyboardInput, WindowEvent},
};

pub struct Window<Renderer: crate::rendering::Renderer, Root: Widget> {
    pub(crate) window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
    input: Input,
    renderer: Renderer,
    root: Root,
}
impl<Renderer: crate::rendering::Renderer, Root: Widget> Window<Renderer, Root> {
    pub fn new(title: &str, mut root: Root) -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let size = root
            .size(Size2D::new(800.0, 600.0))
            .max(Size2D::new(1.0, 1.0));
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(size.width, size.height))
            .build(&event_loop)
            .unwrap();
        let mut renderer = Renderer::new(&window);
        let mut input = Input::new(Box2D::new(
            Point2D::new(0.0, 0.0),
            Point2D::new(size.width, size.height),
        ));
        input.update();
        root.update(&input);
        root.render(Canvas::new(
            &mut renderer,
            Box2D::new(
                Point2D::new(0.0, 0.0),
                Point2D::new(size.width, size.height),
            ),
        ));
        renderer.render();
        Self {
            window,
            event_loop,
            input,
            renderer,
            root,
        }
    }
    pub fn run(mut self) -> !
    where
        Renderer: 'static,
        Root: 'static,
    {
        self.event_loop.run(move |event, _elwt, control_flow| {
            trace!("Event loop run");
            *control_flow = winit::event_loop::ControlFlow::Wait;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state,
                                    virtual_keycode: Some(key),
                                    ..
                                },
                            ..
                        },
                    ..
                } => match state {
                    winit::event::ElementState::Pressed => {
                        self.input.process_pressed(key);
                    }
                    winit::event::ElementState::Released => {
                        self.input.process_released(key);
                    }
                },
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { state, button, .. },
                    ..
                } => match state {
                    winit::event::ElementState::Pressed => {
                        self.input.process_mouse_click(button);
                    }
                    winit::event::ElementState::Released => {
                        self.input.process_mouse_release(button);
                    }
                },
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    let position = position.to_logical(self.window.scale_factor());
                    self.input
                        .process_mouse_move(Point2D::new(position.x, position.y));
                }
                Event::MainEventsCleared => {
                    self.input.update();
                    if self.root.update(&self.input) {
                        debug!("Updating");
                        let window_size = self
                            .window
                            .inner_size()
                            .to_logical(self.window.scale_factor());
                        let size = self.root.size(
                            Size2D::new(window_size.width, window_size.height)
                                .max(Size2D::new(1.0, 1.0)),
                        );
                        self.root.render(Canvas::new(
                            &mut self.renderer,
                            Box2D::new(
                                Point2D::new(0.0, 0.0),
                                Point2D::new(size.width, size.height),
                            ),
                        ));
                        if LogicalSize::new(size.width, size.height) != window_size {
                            debug!("Resizing window to: {:?}", size);
                            self.input.set_bounds(Box2D::new(
                                Point2D::new(0.0, 0.0),
                                Point2D::new(size.width, size.height),
                            ));
                            self.window
                                .set_inner_size(PhysicalSize::new(size.width, size.height));
                            self.renderer
                                .resize(size, self.window.scale_factor() as f32);
                        }
                        self.window.request_redraw();
                    }
                }
                Event::RedrawRequested(_) => {
                    self.renderer.render();
                }
                _ => {
                    trace!("Ignored event: {:?}", event);
                }
            }
        });
    }
}

#[derive(Default)]
pub struct WindowBuilder {
    window_builder: winit::window::WindowBuilder,
}
impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_title(mut self, title: &str) -> Self {
        self.window_builder = self.window_builder.with_title(title);
        self
    }
    pub fn with_inner_size(mut self, size: winit::dpi::LogicalSize<u32>) -> Self {
        self.window_builder = self.window_builder.with_inner_size(size);
        self
    }
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.window_builder = self.window_builder.with_resizable(resizable);
        self
    }
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.window_builder = self.window_builder.with_decorations(decorations);
        self
    }
    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.window_builder = self.window_builder.with_transparent(transparent);
        self
    }
    pub fn with_always_on_top(mut self, always_on_top: bool) -> Self {
        self.window_builder = self.window_builder.with_always_on_top(always_on_top);
        self
    }
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.window_builder = self.window_builder.with_visible(visible);
        self
    }
    pub fn with_window_icon(mut self, icon: Option<winit::window::Icon>) -> Self {
        self.window_builder = self.window_builder.with_window_icon(icon);
        self
    }
    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.window_builder = self.window_builder.with_maximized(maximized);
        self
    }
    pub fn with_fullscreen(mut self, fullscreen: Option<winit::window::Fullscreen>) -> Self {
        self.window_builder = self.window_builder.with_fullscreen(fullscreen);
        self
    }
    pub fn with_min_inner_size(mut self, size: winit::dpi::LogicalSize<u32>) -> Self {
        self.window_builder = self.window_builder.with_min_inner_size(size);
        self
    }
    pub fn with_max_inner_size(mut self, size: winit::dpi::LogicalSize<u32>) -> Self {
        self.window_builder = self.window_builder.with_max_inner_size(size);
        self
    }
    pub fn build<Renderer: crate::rendering::Renderer, Root: Widget>(
        self,
        root: Root,
    ) -> Window<Renderer, Root> {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = self.window_builder.build(&event_loop).unwrap();
        let renderer = Renderer::new(&window);
        let size = window.inner_size().to_logical(window.scale_factor());
        Window {
            window,
            event_loop,
            input: Input::new(Box2D::new(
                Point2D::new(0.0, 0.0),
                Point2D::new(size.width, size.height),
            )),
            renderer,
            root,
        }
    }
}
