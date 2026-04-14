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
        .size(1080, 1080)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();
    app.set_loop_mode(LoopMode::rate_fps(30.0));

    let mut grid = Grid::new();

    grid.set_glider();
    
    Model { grid: grid }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    
    model.grid.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let size = model.grid.size;
    let cell_size = win.w() / size as f32;
    let mut  index;
    
    draw.background().color(GRAY);
    draw.to_frame(app, &frame).unwrap();

    //Draw grid
    for y in 0..size {
        for x in 0..size {
            index = x + size * y;

            if model.grid.grid[index] == 0 {
                continue;
            }

            let px = win.left() + cell_size * x as f32 + cell_size / 2.0;
            let py = win.top() - cell_size * y as f32 - cell_size / 2.0;

            let mut color = BLACK;

            if model.grid.grid[index] > 1{
                color = RED; 
            }

            draw.rect()
                .x_y(px, py)
                .w_h(cell_size - 1.0, cell_size - 1.0)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
