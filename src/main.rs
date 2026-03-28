

const SIZE: usize = 20;




fn main() {



    let mut grid: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    grid[1][2] = 1;
    grid[2][3] = 1;
    grid[3][1] = 1;
    grid[3][2] = 1;
    grid[3][3] = 1;
    
    for step_i in 0..10 {
        println!("Paso {step_i}");
        step(&mut grid);
        println!("----------------");
    }




}


fn step(grid: &mut [[i32; SIZE]; SIZE]) {
    let mut next: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let count = check_sides(x as isize, y as isize, &grid);

            let live = grid[y][x];

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

    std::mem::swap(grid, &mut next);
    print_grid(&grid);
}

fn check_sides(x: isize, y: isize, grid: &[[i32; SIZE]; SIZE]) -> i32 {
    let mut count = 0;

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

fn print_grid(grid: &[[i32; SIZE]; SIZE]) {
    for y in 0..SIZE {
        let mut row = String::with_capacity(SIZE * 2);
        for x in 0..SIZE {
            row.push(if grid[y][x] == 0 { '.' } else { '#' });
            row.push(' ');
        }
        println!("{}", row);
    }
}
