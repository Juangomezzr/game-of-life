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
        self.print_grid();
    }

  
    
}
