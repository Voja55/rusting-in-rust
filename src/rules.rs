use crate::grid::{self, CellState, Grid};
use rand::Rng;

pub fn next_cell_state(grid: &Grid, x:usize, y:usize) -> CellState {
    let current = grid.rust[y][x];
    let humidity = grid.humidity[y][x] ;
    let oxygen = grid.oxygen[y][x] ;

    match current {
        CellState::Rotten => CellState::Rotten,

        CellState::HeavyRust => {
            let spread = humidity * oxygen;
            if rand:: thread_rng().gen_range(0.0..1.0) < spread * 0.6 {
                CellState::Rotten
            } else {
                CellState::HeavyRust
            }
        }

        CellState::SurfaceRust => {
            let spread = humidity * oxygen;
            if rand:: thread_rng().gen_range(0.0..1.0) < spread * 0.4 {
                CellState::HeavyRust
            } else {
                CellState::SurfaceRust
            }
        }

        CellState::Clean => {
            let spread = humidity * oxygen;
            if rand:: thread_rng().gen_range(0.0..1.0) < spread * 0.2 {
                CellState::SurfaceRust
            } else {
                CellState::Clean
            }
        }
    }
}