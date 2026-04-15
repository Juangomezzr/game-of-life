mod grid;
use grid::*;
use nannou::prelude::*;
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
    cell_size: f32
}

fn model(app: &App) -> Model {
    app.new_window()
        .maximized(true)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();
    app.set_loop_mode(LoopMode::rate_fps(30.0));

    let cell_size = 1.0;

    let grid = Grid::new(
        (app.window_rect().w() / cell_size) as usize,
        (app.window_rect().h() / cell_size) as usize,
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
    let win = app.window_rect();


    let cell_size = model.cell_size;
    let mut index;

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();

    //Draw grid
    for y in 0..model.grid.h {
        for x in 0..model.grid.w {
            index = x + model.grid.w * y;

            if model.grid.grid[index] == 0 {
                continue;
            }

            let px = win.left() + cell_size * x as f32 + cell_size / 2.0;
            let py = win.top() - cell_size * y as f32 - cell_size / 2.0;

            let color = model.cell_color;

            draw.rect()
                .x_y(px, py)
                .w_h(cell_size - model.cell_pading, cell_size - model.cell_pading)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
