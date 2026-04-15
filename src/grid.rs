use std::usize;
use rayon::prelude::*;



fn rules(count: u8, live: u8) -> u8{
    if live == 1 {
        if count == 2 || count == 3 { 
            1 
        } else { 
            0 
        }
    } else if count == 3 {
        1
    } else {
        0
    }
}

pub struct Grid {
    pub grid: Vec<u8>,
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
            grid: vec![0;w*h]
            
        }    
    }

    fn check_sides(&self,grid: &[u8], x: isize, y: isize) -> u8 {
        let mut count = 0;
        
        let start: (isize, isize) = (x - 1, y - 1);

        for i in 0..3 {
            let n_y = start.1 + i;
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }

                let n_x = start.0 + j;
                

                if n_x < 0 || n_x >= self.w as isize || n_y < 0 || n_y >= self.h as isize {
                    continue;
                }
                count += (grid[n_x as usize + self.w * n_y as usize] > 0) as u8;
            }
        }

        count
    }

    pub fn step(&mut self) {
        let current = self.grid.clone();
        let mut next = vec![0;self.size];

        next.par_chunks_mut(self.w)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..self.w {
                let idx = x + y * self.w;
                let count = self.check_sides(&current, x as isize, y as isize);
                let live = current[idx];


                row[x] = rules(count, live) ;
            }
        });

    self.grid = next;
        
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
