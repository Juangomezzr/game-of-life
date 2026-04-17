
use nannou::image::{DynamicImage, ImageBuffer, Rgba};                               
use nannou::prelude::*;                                                             
use crate::effects::{Effect};
use crate::{Model};                                                 
                                                                                    
pub struct Renderer {                                                               
    pub texture: wgpu::Texture,                                                     
    pixel_buffer: Vec<u8>,                                                          
    cell_color: Rgba<u8>,
    bg_color: Rgba<u8>,
    width: u32,
    height: u32,
    effect: Effect

}

impl Renderer {
    pub fn new(app: &App, width: u32, height: u32, render_module: String) -> Self {
        let pixel_buffer = vec![0u8; (width * height * 4) as usize];
        let cell_color = Rgba([255, 0, 0, 255]);
        let bg_color = Rgba([0, 0, 0, 255]);

        // Crear textura inicial vacía
        let image_buffer = ImageBuffer::from_fn(width, height, |_, _| bg_color);
        let dynamic_image = DynamicImage::ImageRgba8(image_buffer);
        let texture = wgpu::Texture::from_image(app, &dynamic_image);
        let effect = Effect::new(&render_module);
        Renderer {
            texture,
            pixel_buffer,
            cell_color,
            bg_color,
            width,
            height,
            effect
        }
    }

    ///Actualiza el pixel_buffer que sobrescribira la textura en gpu.
    pub fn update_texture(&mut self, app: &App, grid: &[u8]) {
       
       self.effect.apply(app, grid, &mut self.pixel_buffer); 


    }

    pub fn send_to_gpu(&self, app: &App) {
        let texture_size = wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        };

        app.main_window().queue().write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.pixel_buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.width),
                rows_per_image: Some(self.height),
            },
            texture_size,
        );
    }

    pub fn render(&self, app: &App, draw: &Draw) {
        let win = app.window_rect();
        let sampler_desc = wgpu::SamplerBuilder::new()
            .mag_filter(wgpu::FilterMode::Nearest)
            .min_filter(wgpu::FilterMode::Nearest)
            .into_descriptor();

        draw.sampler(sampler_desc)
            .texture(&self.texture)
            .w_h(win.w(), win.h());
    }

}