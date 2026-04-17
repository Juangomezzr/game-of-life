
use crate::render_effects::RenderEffect;
use crate::modules::{self, *};
use nannou::App;




pub struct Effect {
    module: String,
    pub r: Box<dyn RenderEffect>
}

impl Effect {
    pub fn new(module: &str) -> Self {
        Effect { 
                module: module.to_string(), 
                r: Effect::find_module(module)
            }
    }

    fn set_module(&mut self, module: String){
        self.r = Effect::find_module(&module);
    }

    fn find_module(module: &str) -> Box<dyn RenderEffect>{
        match module {
            "Pixel" =>{Box::new(modules::pixel::PixelEffect::new())},
            "Particles" => {Box::new(modules::particles::Particles::new())},
            _ =>{print!("Module not found");Box::new(modules::pixel::PixelEffect::new())}
            
        }
    }
  
}


