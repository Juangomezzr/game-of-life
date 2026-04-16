mod grid;                                                                           
mod simulation;                                                                     
mod renderer;                                                                       
mod controller; 
mod effects;

use controller::Controller;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    controller: Controller,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(30.0));
    app.new_window()
        .maximized(true)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();

    let win = app.main_window().rect();
    let cell_size = 4.0;

    let width = (win.w() / cell_size) as usize;
    let height = (win.h() / cell_size) as usize;

    let controller = Controller::new(app, width, height);

    Model { controller }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.controller.update(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.controller.render(app, &draw);

    draw.to_frame(app, &frame).unwrap();

    model.controller.send_to_gpu(app);
}