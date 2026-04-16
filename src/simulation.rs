
use crate::grid::Grid;                                                              
                
pub struct Simulation {
    pub grid: Grid,
}

impl Simulation {
    pub fn new(w: usize, h: usize) -> Self {
        Simulation {
            grid: Grid::new(w, h),
        }
    }

    pub fn step(&mut self) {
        self.grid.step();
    }

    pub fn set_medusa(&mut self) {
        self.grid.set_medusa();
    }

    pub fn width(&self) -> usize {
        self.grid.w
    }

    pub fn height(&self) -> usize {
        self.grid.h
    }

    pub fn get_grid(&self) -> &[u8] {
        &self.grid.grid
    }
}