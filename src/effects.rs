use nannou::App;
use nannou::image::Rgba;

const DEF_MODULE: Module = Module::Cell;
const DEF_F: fn(app: &App, grid: &[u8], buffer: &mut [u8]) = cells_fun;

#[derive(Clone, Copy, PartialEq)]
enum Module {
    Cell,
    Lines,
}

pub struct Effect {
    module: Module,
    f: fn(app: &App, grid: &[u8], buffer: &mut [u8]),
}

impl Effect {
    pub fn new() -> Self {
        let mut effect = Effect {
            module: DEF_MODULE,
            f: DEF_F,
        };

        effect.set_module(effect.module);

        effect
    }

    fn set_module(&mut self, module: Module) -> &mut Self {
        self.module = module;

        match self.module {
            Module::Cell => self.f = cells_fun,
            _ => self.f = cells_fun,
        }

        self
    }

    pub fn apply(&mut self, app: &App, grid: &[u8], buffer: &mut [u8]) -> &mut Self {
        (self.f)(app, grid, buffer);

        self
    }
}

pub fn cells_fun(app: &App, grid: &[u8], buffer: &mut [u8]) {
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
