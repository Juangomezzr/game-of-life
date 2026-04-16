use std::{mem::swap, usize};
use rayon::prelude::*;


fn rules(count: u8, live: u8) -> u8{
    if live == 1 {
        if count == 3 || count == 3 { 
            1 
        } else { 
            0 
        }
    } else if count == 2 {
        1
    } else {
        0
    }
}

pub struct Grid {
    pub grid: Vec<u8>,
    next_grid: Vec<u8>,
    pub w: usize,
    pub h:usize,
    pub size: usize
}
impl Grid {
    pub fn new(w:usize,h:usize) -> Grid {
        Grid {
            w:w,
            h:h,
            size:w*h,
            grid: vec![0;w*h],
            next_grid:vec![0;w*h]
            
        }    
    }

    /// Cuenta las células vivas adyacentes a una coordenada (vecinos en ventana 3x3).
    ///
    /// ## Algoritmo
    /// 1. Define ventana 3x3 desde `(x-1, y-1)` hasta `(x+1, y+1)`
    /// 2. Itera con dos bucles anidados sobre las 9 posiciones
    /// 3. Excluye el centro `(i=1, j=1)` con `continue`
    /// 4. Valida límites: descarta coordenadas fuera de la cuadrícula
    /// 5. Convierte 2D a lineal: `n_x + w * n_y`
    /// 6. Acumula: `(grid[...] > 0) as u8` (bool→u8: true=1, false=0)
    ///
    /// ## Rust
    /// - `&[u8]`: **slice** inmutable - vista prestada a datos contiguos
    /// - `isize`: entero con signo para coordenadas temporales negativas (bordes)
    /// - `usize`: tipo nativo para índices/tamaños
    /// - `as`: conversión explícita (`usize`↔`isize`, `bool`→`u8`)
    /// - `0..3`: rango exclusivo (itera 0, 1, 2)
    /// - `continue`: salta a siguiente iteración del bucle interno
    /// - `(a, b).0` / `(a, b).1`: acceso a elementos de tupla
    fn check_sides(grid: &[u8], x: isize, y: isize, w: usize, h: usize) -> u8 {
        let mut count = 0;

        let start: (isize, isize) = (x - 1, y - 1);

        for i in 0..3 {
            let n_y = start.1 + i;
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }

                let n_x = start.0 + j;

                if n_x < 0 || n_x >= w as isize || n_y < 0 || n_y >= h as isize {
                    continue;
                }
                count += (grid[n_x as usize + w * n_y as usize] > 0) as u8;
            }
        }

        count
    }

    /// Avanza un paso en la simulación.
    ///
    /// ## Algoritmo
    /// 1. Itera en paralelo sobre `next_grid` distribuyendo trabajo entre hilos
    /// 2. Para cada célula: calcula coordenadas 2D (`x = idx % w`, `y = idx / w`)
    /// 3. Obtiene vecinos vivos con `check_sides()` desde `self.grid` (inmutable)
    /// 4. Aplica reglas del juego para determinar nuevo estado
    /// 5. Intercambia búferes: `next_grid` → `grid`, `grid` → `next_grid` (O(1))
    ///
    /// ## Rust
    /// - **Rayon** (`use rayon::prelude::*`): crate para paralelismo de datos
    /// - `par_iter_mut()`: iterador paralelo mutable - divide el `Vec` en chunks por hilo
    /// - `enumerate()`: añade índice a cada elemento, retorna `(usize, &mut T)`
    /// - `with_min_len(512)`: mínimo de elementos por hilo (evita overhead)
    /// - `for_each(|(idx, cell)| ...)`: consume el iterador, ejecuta closure en paralelo
    /// - `|(idx, cell)|`: closure que desestructura tupla por patrón
    /// - `&self.grid`: préstamo inmutable - seguro para lectura concurrente
    /// - `*cell = ...`: desreferencia de `&mut u8` para asignar valor
    /// - `swap()`: intercambia dos valores sin copia (solo punteros, O(1))
    /// - `as isize`: casting explícito `usize` → `isize`
    ///
    /// ## Seguridad
    /// Thread-safe garantizado por borrow checker: `grid` es inmutable (lectura paralela OK),
    /// cada `cell` es única por hilo (escritura sin colisiones).
    pub fn step(&mut self) {
        self.next_grid
            .par_iter_mut()
            .enumerate()
            .with_min_len(512)
            .for_each(|(idx, cell)| {
                let x = idx % self.w;
                let y = idx / self.w;

                let count = Grid::check_sides(&self.grid, x as isize, y as isize, self.w, self.h);
                let live = self.grid[idx];

                *cell = rules(count, live);
            });

        swap(&mut self.grid, &mut self.next_grid);
    }

    fn set_pixel(&mut self,x: usize,y: usize, value: u8){
        self.grid[x + self.w * y]  = value
    }

    
    fn print_grid(&self) {
        for y in 0..self.h {
            let mut row = String::with_capacity(self.size * 2);
            for x in 0..self.w {
                row.push(if self.grid[x + self.size * y] == 0 { '.' } else { '#' });
                row.push(' ');
            }
            println!("{}", row);
        }
    }


    fn set_block_at(&mut self, x: usize, y: usize) {
        self.set_pixel(x, y, 1);
        self.set_pixel(x + 1, y, 1);
        self.set_pixel(x, y + 1, 1);
        self.set_pixel(x + 1, y + 1, 1);
    }

    fn set_beehive_at(&mut self, x: usize, y: usize) {
        self.set_pixel(x + 1, y, 1);
        self.set_pixel(x + 2, y, 1);
        self.set_pixel(x, y + 1, 1);
        self.set_pixel(x + 3, y + 1, 1);
        self.set_pixel(x + 1, y + 2, 1);
        self.set_pixel(x + 2, y + 2, 1);
    }

    fn set_tub_at(&mut self, x: usize, y: usize) {
        self.set_pixel(x + 1, y, 1);
        self.set_pixel(x, y + 1, 1);
        self.set_pixel(x + 2, y + 1, 1);
        self.set_pixel(x + 1, y + 2, 1);
    }

    fn read_pixel(&self, x: usize, y: usize) -> u8{
        self.grid[x + y * self.w]
    }


    pub fn set_glider(&mut self){
        self.set_pixel(1, 2, 1);
        self.set_pixel(2, 3, 1);
        self.set_pixel(3, 1, 1);
        self.set_pixel(3, 2, 1);
        self.set_pixel(3, 3, 1);
    }

    pub fn set_blinker(&mut self){
        self.set_pixel(9, 10, 1);
        self.set_pixel(10, 10, 1);
        self.set_pixel(11, 10, 1);
    }

    pub fn set_medusa(&mut self){
        let mid_x = self.w / 2;
        let mid_y = self.h / 2;
        let cells = [
            (mid_x - 2, mid_y - 4),
            (mid_x - 1, mid_y - 4),
            (mid_x, mid_y - 4),
            (mid_x + 1, mid_y - 4),
            (mid_x + 2, mid_y - 4),
            (mid_x - 3, mid_y - 3),
            (mid_x + 3, mid_y - 3),
            (mid_x - 3, mid_y - 2),
            (mid_x - 1, mid_y - 2),
            (mid_x, mid_y - 2),
            (mid_x + 1, mid_y - 2),
            (mid_x + 3, mid_y - 2),
            (mid_x - 2, mid_y - 1),
            (mid_x + 2, mid_y - 1),
            (mid_x - 2, mid_y),
            (mid_x - 2, mid_y + 1),
            (mid_x - 3, mid_y + 2),
            (mid_x - 1, mid_y + 2),
            (mid_x + 1, mid_y + 2),
            (mid_x + 3, mid_y + 2),
            (mid_x - 3, mid_y + 3),
            (mid_x - 1, mid_y + 3),
            (mid_x + 1, mid_y + 3),
            (mid_x + 3, mid_y + 3),
            (mid_x - 4, mid_y + 4),
            (mid_x - 2, mid_y + 4),
            (mid_x, mid_y + 4),
            (mid_x + 2, mid_y + 4),
            (mid_x + 4, mid_y + 4),
        ];

        for (x, y) in cells {
            self.set_pixel(x, y, 1);
        }
            
    }

    pub fn set_stable_colony(&mut self) {
        let mid_x = self.w as isize / 2;
        let mid_y = self.h as isize / 2;

        let blocks = [
            (-30, -30), (-18, -30), (-6, -30), (6, -30), (18, -30), (30, -30),
            (-30, -18), (30, -18),
            (-30, -6), (-6, -6), (6, -6), (30, -6),
            (-30, 6), (-6, 6), (6, 6), (30, 6),
            (-30, 18), (30, 18),
            (-30, 30), (-18, 30), (-6, 30), (6, 30), (18, 30), (30, 30),
        ];

        let beehives = [
            (-12, -18), (0, -18), (12, -18),
            (-18, 0), (18, 0),
            (-12, 18), (0, 18), (12, 18),
        ];

        let tubs = [
            (-18, -12), (18, -12),
            (-18, 12), (18, 12),
            (0, -6), (0, 6),
        ];

        for (dx, dy) in blocks {
            self.set_block_at((mid_x + dx) as usize, (mid_y + dy) as usize);
        }

        for (dx, dy) in beehives {
            self.set_beehive_at((mid_x + dx) as usize, (mid_y + dy) as usize);
        }

        for (dx, dy) in tubs {
            self.set_tub_at((mid_x + dx) as usize, (mid_y + dy) as usize);
        }
    }

  
    
}
