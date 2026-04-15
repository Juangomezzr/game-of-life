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
    texture: wgpu::Texture
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(10.0));
    app.new_window()
        .maximized(true)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();
    

    let cell_size = 2.0;
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

    Model {
        grid: grid,
        cell_pading: 0.0,
        cell_color: cell_color,
        bg_color: bg_color,
        cell_size: cell_size,
        win: win,
        texture: texture,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.grid.step();
    model.grid.set_medusa();

    model.texture = create_texture_from_grid(
        app,
        &model.grid.grid,
        model.cell_color,
        model.bg_color,
        model.grid.w as u32,
        model.grid.h as u32,
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();

    texture_render(model, &draw);
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
    // from_fn recorre cada (x, y) de la imagen y nos pide que le devolvamos un color.

    let image_buffer = ImageBuffer::from_fn(w, h, |x, y| {
        // Calculamos el índice en nuestra cuadrícula 1D
        let idx = (x + y * w) as usize;
        let cell = grid[idx];

        let color = if cell > 0 { cell_c } else { bg_c };
        color

        // Devolvemos el tipo Rgba, que por debajo es literalmente un array de 4 elementos: [u8; 4]
    });

    let dynamic_image = DynamicImage::ImageRgba8(image_buffer);

    wgpu::Texture::from_image(app, &dynamic_image)
}

fn texture_render(model: &Model, draw: &Draw) {
    let sampler_desc = wgpu::SamplerBuilder::new()
        .mag_filter(wgpu::FilterMode::Nearest)
        .min_filter(wgpu::FilterMode::Nearest)
        .into_descriptor();

    draw.sampler(sampler_desc)
        .texture(&model.texture)
        .w_h(model.win.w(), model.win.h());
}
