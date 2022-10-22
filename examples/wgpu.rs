use xenon::{window::Window, widgets::RandomRectangle, rendering::WgpuRenderer, math::Size2D};

fn main() {
    env_logger::init();
    let window = Window::<WgpuRenderer, _>::new("WGPU Example", RandomRectangle::new(Size2D::new(800.0, 600.0)));
    window.run();
}
