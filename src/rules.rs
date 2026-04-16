use std::cell::Cell;

use crate::grid::{self, CellState, Grid};
use rand::Rng;

pub fn next_cell_state(grid: &Grid, x:usize, y:usize) -> CellState {
    let current = grid.rust[y][x];
    let humidity = grid.humidity[y][x] ;
    let oxygen = grid.oxygen[y][x] ;
    let spread = humidity * oxygen;

    match current {
        CellState::Rotten => CellState::Rotten,

        CellState::HeavyRust => {
            if rand:: thread_rng().gen_range(0.0..1.0) < spread * 0.6 {
                CellState::Rotten
            } else {
                CellState::HeavyRust
            }
        }

        CellState::SurfaceRust => {
            if rand:: thread_rng().gen_range(0.0..1.0) < spread * 0.4 {
                CellState::HeavyRust
            } else {
                CellState::SurfaceRust
            }
        }

        CellState::Clean => {
            let neighbors = grid.get_neighbors(x, y);

            let worst_neigh = neighbors.iter()
                .map(|&(nx,ny)| &grid.rust[ny][nx])
                .filter(|&&ref state| *state != CellState::Clean)
                .max_by_key(|&&ref state| match state {
                    CellState::SurfaceRust => 1,
                    CellState::HeavyRust => 2,
                    CellState::Rotten => 3,
                    CellState::Clean => 0
                });

            match  worst_neigh {
                None => CellState::Clean,
                Some(neigh_state) => {
                    let neigh_effect = match neigh_state {
                        CellState::SurfaceRust => 0.2,
                        CellState::HeavyRust => 0.5,
                        CellState::Rotten => 0.8,
                        CellState::Clean => 0.0
                    };

                    let spread_chance = neigh_effect * spread;

                    if rand::thread_rng().gen_range(0.0..1.0) < spread_chance {
                        CellState::SurfaceRust
                    } else {
                        CellState::Clean
                    }
                }
            }
        }
    }
}