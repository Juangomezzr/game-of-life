use crate::render_effects::RenderEffect;
use nannou::prelude::*;

// 1. La entidad matemática de nuestra chispa
struct PuntoVectorial {
    pos: Vec2,
    vel: Vec2,
    vida: f32,
}

// 2. El módulo principal
pub struct Particles {
    id: String,
    vectores: Vec<PuntoVectorial>,
}

impl Particles {
    pub fn new() -> Self {
        Self { 
            id: "Particles".to_string(),
            vectores: Vec::new(),
        }
    }
}

impl RenderEffect for Particles {
    

    
    fn get_id(&self) -> &str {
        &self.id
    }
    
    // ==========================================
    // FASE 1: MATEMÁTICAS (Update)
    // ==========================================
    fn update(&mut self, app: &App, grid: &[u8], _buffer: &mut [u8]) {
        let win = app.window_rect();
        let width = win.w() as u32;
        let height = win.h() as u32;

        // 1. Envejecemos y movemos los vectores que ya existen
        for p in &mut self.vectores {
            p.pos += p.vel;
            p.vida -= 0.05; // Velocidad a la que se apaga la chispa
        }
        
        // Limpiamos la memoria de las partículas muertas
        self.vectores.retain(|p| p.vida > 0.0);

        // 2. Leemos la cuadrícula y generamos nuevas chispas
        for (i, &cell) in grid.iter().enumerate() {
            // Genera partículas solo si la célula está viva y ganamos una "lotería" del 15%
            if cell > 0 && random_f32() < 0.15 {
                
                let x = (i as u32) % width;
                let y = (i as u32) / width;

                // Traducimos el índice de la cuadrícula a coordenadas Nannou
                let screen_x = map_range(x, 0, width, win.left(), win.right());
                
                // Mapeamos la Y. Nannou tiene Y positivo arriba, la cuadrícula suele tener Y positivo abajo.
                let screen_y = map_range(y, 0, height, win.top(), win.bottom());

                self.vectores.push(PuntoVectorial {
                    pos: vec2(screen_x, screen_y),
                    // Vector de velocidad: se mueven un poco a los lados, y bastante hacia arriba
                    vel: vec2(random_range(-1.0, 1.0), random_range(1.0, 3.0)), 
                    vida: 1.0,
                });
            }
        }
    }

    // ==========================================
    // FASE 2: DIBUJO (Render)
    // ==========================================
    fn render(&self,_app: &App, draw: &Draw,_texture: &wgpu::Texture) {
        
        // No tocamos la matriz de píxeles, solo le decimos a la GPU qué dibujar
        for p in &self.vectores {
            // Creamos un color de neón (Cyan) que se vuelve transparente al morir
            let color = hsla(0.5, 1.0, 0.6, p.vida); 

            // Dibujamos nuestra partícula vectorial
            draw.ellipse()
                .xy(p.pos)
                .w_h(4.0, 4.0)
                .color(color);
        }
    }
}