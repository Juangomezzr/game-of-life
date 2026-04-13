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

    nannou::app(model).run()


}

struct Model {
    grid: Grid
}

fn model(app: &App) -> Model {

    app.new_window()
        .size(800, 800)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();


    let mut grid = Grid::new();

    grid.grid[1][2] = 1;
    grid.grid[2][3] = 1;
    grid.grid[3][1] = 1;
    grid.grid[3][2] = 1;
    grid.grid[3][3] = 1;

    Model {grid: grid}


}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App,model: &Model,frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    let size = model.grid.size;
    let cell_size = win.w()/size as f32;


    draw.background().color(GRAY);
    draw.to_frame(app, &frame).unwrap();
    
    
    

        for y in 0..size {
            for x in 0..size {

                let alive = model.grid.grid[y][x] == 1;

                let px = win.left() + cell_size * x as f32 + cell_size / 2.0;
                let py = win.top() - cell_size * y as f32 - cell_size / 2.0;


                draw.rect()
                .x_y(px, py)
                .w_h(cell_size - 1.0, cell_size - 1.0)
                .color(if alive { BLACK } else { LIGHTGRAY });
                
            }
        }
        draw.to_frame(app, &frame).unwrap();    

    
    

}