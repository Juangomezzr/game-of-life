use std::usize;


const SIZE: usize = 1000;


pub struct Grid {
    pub size: usize,
    pub grid: Vec<u8>
}
impl Grid {
    pub fn new() -> Grid {
        Grid {
            size: SIZE,
            grid: vec![0;SIZE*SIZE]
        }    
    }

    fn check_sides(&self, x: isize, y: isize) -> u8 {
        let mut count = 0;
        let grid = &self.grid;
        let start: (isize, isize) = (x - 1, y - 1);

        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }

                let n_x = start.0 + i;
                let n_y = start.1 + j;

                if n_x < 0 || n_x >= SIZE as isize || n_y < 0 || n_y >= SIZE as isize {
                    continue;
                }
                count += grid[n_x as usize + SIZE * n_y as usize];
            }
        }

        count
    }

    fn print_grid(&self) {
        for y in 0..SIZE {
            let mut row = String::with_capacity(SIZE * 2);
            for x in 0..SIZE {
                row.push(if self.grid[x + SIZE * y] == 0 { '.' } else { '#' });
                row.push(' ');
            }
            println!("{}", row);
        }
    }

    pub fn step(&mut self) {
        let mut next = vec![0;SIZE * SIZE];

        for y in 0..SIZE {
            for x in 0..SIZE {
                let count = self.check_sides(x as isize, y as isize);

                let live = self.grid[x + y *SIZE];

                if live == 1 {
                    if count == 2 || count == 3 {
                        next[x + y * SIZE] = 1;
                    } else {
                        next[x + y *SIZE] = 0
                    }
                } else {
                    if count == 3 {
                        next[x + y * SIZE] = 1;
                    }
                }
            }
        }

        std::mem::swap(&mut self.grid, &mut next);
        
    }

    fn set_pixel(&mut self,x: usize,y: usize, value: u8){
        self.grid[x + y * SIZE] = value
    }

    fn read_pixel(&self, x: usize, y: usize) -> u8{
        self.grid[x + y * SIZE]
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
        let mid_x = self.size / 2;
        let mid_y = self.size / 2;
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

  
    
}
