mod grid;
use grid::*;
use nannou::prelude::*;
use nannou::image::{DynamicImage, ImageBuffer};
use nannou::wgpu::Texture;
fn main() {
    /*/
    for step_i in 0..10 {
        println!("Paso {step_i}");
        grid.step();
        println!("----------------");
    }
    */

    nannou::app(model).update(update).run()
}

struct Model {
    grid: Grid,
    cell_pading: f32,
    cell_color: Rgba,
    cell_size: f32,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(30.0));
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



    Model {
        grid: grid,
        cell_pading: 0.0,
        cell_color: rgba(1.0, 0.0, 0.0, 1.0),
        cell_size: cell_size

    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.grid.step();
    model.grid.set_medusa();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();

    
    draw_grid(app, &draw, model);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid(app: &App, draw: &Draw, model: &Model){
    let win = app.window_rect();
    let cell_size = model.cell_size;
    let color = model.cell_color;
    let mut index;

    //Draw grid


    for y in 0..model.grid.h {
        for x in 0..model.grid.w {
            index = x + model.grid.w * y;
            if model.grid.grid[index] == 0 {
                continue;
            }

            let px = win.left() + cell_size * x as f32 + cell_size / 2.0;
            let py = win.top() - cell_size * y as f32 - cell_size / 2.0;

           

            draw.rect()
                .x_y(px, py)
                .w_h(cell_size - model.cell_pading, cell_size - model.cell_pading)
                .color(color);
        }
    }

}

