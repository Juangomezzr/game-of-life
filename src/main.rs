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

    nannou::app(model) 
        .update(update)
        .run()
}

struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    app
        .new_window()
        .size(1000, 1000)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();
    app.set_loop_mode(LoopMode::rate_fps(30.0));

    let mut grid = Grid::new();

    grid.set_medusa();

    Model { grid: grid }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    
    model.grid.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let size = model.grid.size;
    let cell_size = win.w() / size  as f32;

    draw.background().color(LIGHTGRAY);
    draw.to_frame(app, &frame).unwrap();
    for y in 0..size {
        for x in 0..size {
            if model.grid.grid[y][x] == 0 {
                continue;
            }

            let px = win.left() + cell_size * x as f32 + cell_size / 2.0;
            let py = win.top() - cell_size * y as f32 - cell_size / 2.0;

            draw.rect()
                .x_y(px, py)
                .w_h(cell_size - 1.0, cell_size - 1.0)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
