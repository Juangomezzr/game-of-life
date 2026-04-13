const SIZE: usize = 50;

pub struct Grid {
    pub size: usize,
    pub grid: [[u32; SIZE]; SIZE],
}
impl Grid {
    pub fn new() -> Grid {
        Grid {
            size: SIZE,
            grid: [[0; SIZE]; SIZE],
        }    
    }

    fn check_sides(&self, x: isize, y: isize) -> u32 {
        let mut count = 0;
        let grid = self.grid;
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
                count += grid[n_y as usize][n_x as usize];
            }
        }

        count
    }

    fn print_grid(&self) {
        let grid = self.grid;
        for y in 0..SIZE {
            let mut row = String::with_capacity(SIZE * 2);
            for x in 0..SIZE {
                row.push(if grid[y][x] == 0 { '.' } else { '#' });
                row.push(' ');
            }
            println!("{}", row);
        }
    }

    pub fn step(&mut self) {
        
        let mut next: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

        for y in 0..SIZE {
            for x in 0..SIZE {
                let count = self.check_sides(x as isize, y as isize);

                let live = self.grid[y][x];

                if live == 1 {
                    if count == 2 || count == 3 {
                        next[y][x] = 1;
                    } else {
                        next[y][x] = 0
                    }
                } else {
                    if count == 3 {
                        next[y][x] = 1;
                    }
                }
            }
        }

        std::mem::swap(&mut self.grid, &mut next);
        
    }

    pub fn set_glider(&mut self){
        self.grid[1][2] = 1;
        self.grid[2][3] = 1;
        self.grid[3][1] = 1;
        self.grid[3][2] = 1;
        self.grid[3][3] = 1;
    }

    pub fn set_blinker(&mut self){
        self.grid[10][9] = 1;
        self.grid[10][10] = 1;
        self.grid[10][11] = 1;
    }

    pub fn set_medusa(&mut self){
        let mid_x = self.size / 2;
        let mid_y = self.size / 2;
        let cells = [
            (mid_y - 4, mid_x - 2),
            (mid_y - 4, mid_x - 1),
            (mid_y - 4, mid_x),
            (mid_y - 4, mid_x + 1),
            (mid_y - 4, mid_x + 2),
            (mid_y - 3, mid_x - 3),
            (mid_y - 3, mid_x + 3),
            (mid_y - 2, mid_x - 3),
            (mid_y - 2, mid_x - 1),
            (mid_y - 2, mid_x),
            (mid_y - 2, mid_x + 1),
            (mid_y - 2, mid_x + 3),
            (mid_y - 1, mid_x - 2),
            (mid_y - 1, mid_x + 2),
            (mid_y, mid_x - 2),
            (mid_y + 1, mid_x - 2),
            (mid_y + 2, mid_x - 3),
            (mid_y + 2, mid_x - 1),
            (mid_y + 2, mid_x + 1),
            (mid_y + 2, mid_x + 3),
            (mid_y + 3, mid_x - 3),
            (mid_y + 3, mid_x - 1),
            (mid_y + 3, mid_x + 1),
            (mid_y + 3, mid_x + 3),
            (mid_y + 4, mid_x - 4),
            (mid_y + 4, mid_x - 2),
            (mid_y + 4, mid_x),
            (mid_y + 4, mid_x + 2),
            (mid_y + 4, mid_x + 4),
        ];

        for (y, x) in cells {
            self.grid[y][x] = 1;
        }
    }

  
    
}
