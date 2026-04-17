use nannou::{App, Draw, draw::primitive::texture, image::Rgba, wgpu};

use crate::render_effects::RenderEffect;


pub struct PixelEffect{
    id: String
}
impl PixelEffect {
    pub fn new() -> PixelEffect {
        PixelEffect{ id: "Pixel".to_string()}
    }
}
impl RenderEffect for PixelEffect{
    fn update(&mut self, _app: &App, grid: &[u8], buffer: &mut [u8]) {
        let cell_color = Rgba([255, 0, 0, 255]);
        let bg_color = Rgba([0, 0, 0, 255]);
    
        for (cell, pixel) in grid.iter().zip(buffer.chunks_exact_mut(4)) {
            let color = if *cell > 0 { cell_color } else { bg_color };
            pixel[0] = color[0];
            pixel[1] = color[1];
    
            pixel[2] = color[2];
            pixel[3] = color[3];
        }
    }
    fn render(&self, app: &App, draw: &Draw, texture: &wgpu::Texture) {
        let win = app.window_rect();
        let sampler_desc = wgpu::SamplerBuilder::new()
            .mag_filter(wgpu::FilterMode::Nearest)
            .min_filter(wgpu::FilterMode::Nearest)
            .mipmap_filter(wgpu::FilterMode::Nearest)
            .into_descriptor();

        draw.sampler(sampler_desc)
            .texture(texture)
            .w_h(win.w(), win.h());

    }
 

    fn get_id(&self) -> &str {
        &self.id
    }
}