
use nannou::prelude::*;                                                             
use crate::simulation::Simulation;                                                  
use crate::renderer::Renderer;                                                      
                
pub struct Controller{
    simulation: Simulation,
    renderer: Renderer,
    
}

impl Controller {
    pub fn new(app: &App, width: usize, height: usize, render_module: String) -> Self {
        let simulation = Simulation::new(width, height);
        let renderer = Renderer::new(app, width as u32, height as u32,render_module);

        Controller {
            simulation,
            renderer,
         
        }
    }

    pub fn update(&mut self,app: &App) {
        self.simulation.step();
        self.simulation.set_medusa();
        self.renderer.update_texture(app, self.simulation.get_grid());
    }

    pub fn render(&self, app: &App, draw: &Draw) {
        self.renderer.render(app, draw);
    }

    pub fn send_to_gpu(&self, app: &App) {
        self.renderer.send_to_gpu(app);
    }

    pub fn width(&self) -> usize {
        self.simulation.width()
    }

    pub fn height(&self) -> usize {
        self.simulation.height()
    }
}