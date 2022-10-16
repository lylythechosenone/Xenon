use crate::input::Input;
use log::debug;
use winit::event::{Event, KeyboardInput, WindowEvent};
use crate::rendering::RenderState;

pub struct Window<Renderer: crate::rendering::Renderer> {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
    input: Input,
    renderer: Renderer,
    render_state: RenderState,
}
impl<Renderer: crate::rendering::Renderer> Window<Renderer> {
    pub fn new(title: &str) -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        let render_state = RenderState::new();
        let renderer = Renderer::with_state(&format!("{} renderer", title), &mut render_state);
        Self {
            window,
            event_loop,
            input: Input::new(),
            renderer,
            render_state
        }
    }
    pub fn run(mut self) -> ! {
        self.event_loop.run(move |event, _elwt, control_flow| {
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
                _ => {
                    debug!("Ignored event: {:?}", event);
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
    pub fn build<Renderer: crate::rendering::Renderer>(self, renderer: Renderer, render_state: RenderState) -> Window<Renderer> {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = self.window_builder.build(&event_loop).unwrap();
        Window {
            event_loop,
            window,
            input: Input::new(),
            renderer,
            render_state,
        }
    }
    pub fn run<Renderer: crate::rendering::Renderer>(self, renderer: Renderer, render_state: RenderState) -> ! {
        self.build(renderer, render_state).run()
    }
}
