mod grid;

use grid::*;
use nannou::image::{DynamicImage, ImageBuffer, Rgba};
use nannou::prelude::*;
fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    grid: Grid,
    cell_pading: f32,
    cell_color: Rgba<u8>,
    bg_color: Rgba<u8>,
    cell_size: f32,
    win: Rect,
    texture: wgpu::Texture,
    pixel_buffer: Vec<u8>,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(30.0));
    app.new_window()
        .maximized(true)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();

    let cell_size = 1.0;
    let window = app.main_window();
    let win = window.rect();

    let grid = Grid::new(
        (win.w() / cell_size) as usize,
        (win.h() / cell_size) as usize,
    );
    let cell_color = Rgba([255, 0, 0, 255]);
    let bg_color = Rgba([0, 0, 0, 255]);

    let texture = create_texture_from_grid(
        app,
        &grid.grid,
        cell_color,
        bg_color,
        grid.w as u32,
        grid.h as u32,
    );

    let pixel_buffer = vec![0u8; (grid.w * grid.h * 4) as usize];

    Model {
        grid: grid,
        cell_pading: 0.0,
        cell_color: cell_color,
        bg_color: bg_color,
        cell_size: cell_size,
        win: win,
        texture: texture,
        pixel_buffer: pixel_buffer,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.grid.step();
    model.grid.set_medusa();

    update_texture(model);
    send_to_gpu(app, model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.to_frame(app, &frame).unwrap();
    // 1. Limpiamos el fondo
    draw.background().color(BLACK);

    // 3. Dibujamos la textura directamente al tamaño de la ventana
    // (Por ahora le quitamos el filtro Nearest para asegurarnos de que se ve)
    texture_render(app, model);
    // 4. Mandamos el lienzo al monitor
    draw.to_frame(app, &frame).unwrap();
}
/// Texture Logics
fn create_texture_from_grid(
    app: &App,
    grid: &[u8],
    cell_c: Rgba<u8>,
    bg_c: Rgba<u8>,
    w: u32,
    h: u32,
) -> wgpu::Texture {
    let image_buffer = ImageBuffer::from_fn(w, h, |x, y| {
        let idx = (x + y * w) as usize;
        let cell = grid[idx];

        let color = if cell > 0 { cell_c } else { bg_c };

        color

        // Devolvemos el tipo Rgba, que por debajo es literalmente un array de 4 elementos: [u8; 4]
    });

    let dynamic_image = DynamicImage::ImageRgba8(image_buffer);

    wgpu::Texture::from_image(app, &dynamic_image)
}

fn texture_render(app: &App, model: &Model) {
    let win = app.window_rect();
    let draw = app.draw();

    let sampler_desc = wgpu::SamplerBuilder::new()
        .mag_filter(wgpu::FilterMode::Nearest)
        .min_filter(wgpu::FilterMode::Nearest)
        .into_descriptor();

    draw.sampler(sampler_desc)
        .texture(&model.texture)
        .w_h(win.w(), win.h());
}

fn update_texture(model: &mut Model) {
    for (cell, pixel) in model
        .grid
        .grid
        .iter()
        .zip(model.pixel_buffer.chunks_exact_mut(4))
    {
        let color = if *cell > 0 {
            model.cell_color
        } else {
            model.bg_color
        };
        pixel[0] = color[0]; // R
        pixel[1] = color[1]; // G
        pixel[2] = color[2]; // B
        pixel[3] = color[3]; // A
    }
}

fn send_to_gpu(app: &App, model: &Model) {
    let texture_size = wgpu::Extent3d {
        width: model.grid.w as u32,
        height: model.grid.h as u32,
        depth_or_array_layers: 1,
    };

    app.main_window().queue().write_texture(
        // Destino: La textura que ya creamos en `fn model`
        wgpu::ImageCopyTexture {
            texture: &model.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        // Origen: Nuestro array de bytes actualizado
        &model.pixel_buffer,
        // Formato: Le explicamos a la gráfica cómo leer nuestro array
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * model.grid.w as u32),
            rows_per_image: Some(model.grid.h as u32),
        },
        texture_size,
    );
}
