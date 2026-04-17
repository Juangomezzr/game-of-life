use nannou::{App, Draw, draw::primitive::Texture, wgpu};

pub trait RenderEffect {
    fn update(&mut self, _app: &App, grid: &[u8], buffer: &mut [u8]);
    fn render(&self, app: &App, draw: &Draw, texture: &wgpu::Texture);
    fn get_id(&self) -> &str;
}