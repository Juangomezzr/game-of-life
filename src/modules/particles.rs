use crate::render_effects::RenderEffect;
use nannou::App;

pub struct Particles {
    id: String,
}

impl Particles {
    pub fn new() -> Self {
        Self { id: "Particles".to_string() }
    }
}

impl RenderEffect for Particles {
    
    fn apply(&mut self, app: &App, grid: &[u8], buffer: &mut [u8]) {
        self.update(app, grid, buffer);
    }
    
    fn get_id(&self) -> &str {
        &self.id
    }
    
    fn render(&self, _app: &App) {}
    
    fn update(&mut self, app: &App, grid: &[u8], buffer: &mut [u8]) {
        let win = app.window_rect();
        let width = win.w() as u32;
        let height = win.h() as u32;

        // MAGIA RUST: Juntamos la celda del grid y el pixel del buffer, y además sacamos el índice (i)
        for (i, (&cell, pixel)) in grid.iter().zip(buffer.chunks_exact_mut(4)).enumerate() {
            
            // Si la célula de Conway está VIVA (mayor que 0)
            if cell > 0 {
                let x = (i as u32) % width;
                let y = (i as u32) / width;

                let norm_x = x as f32 / width as f32;
                let norm_y = y as f32 / height as f32;

                // Aplicamos el color basado en su posición
                pixel[0] = (norm_x * 255.0) as u8; // Rojo de izquierda a derecha
                pixel[1] = (norm_y * 255.0) as u8; // Verde de arriba a abajo
                pixel[2] = 120;                    // Azul fijo
                pixel[3] = 255;                    // Alpha
            } 
            // Si la célula está MUERTA
            else {
                // OPCIÓN A: Fondo negro instantáneo
                /*
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
                pixel[3] = 255;
                */

                // OPCIÓN B: Efecto de estela suave (Recomendado, se ve increíble con los colores)
                pixel[0] = pixel[0].saturating_sub(15);
                pixel[1] = pixel[1].saturating_sub(15);
                pixel[2] = pixel[2].saturating_sub(15);
                // El Alpha siempre en 255 para que wgpu no haga cosas raras con transparencias
                pixel[3] = 255; 
            }
        }
    }
}