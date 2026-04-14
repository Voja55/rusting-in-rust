use std::{arch::x86_64, cell::Cell};

use rand::{Rng, random, rngs};

use crate::grid;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Clean,
    SurfaceRust,
    HeavyRust,
    Rotten
}

impl CellState {
    pub fn to_char(&self) -> char {
        match self {
            CellState::Clean => 'O',
            CellState::SurfaceRust => '@',
            CellState::HeavyRust => '#',
            CellState::Rotten => 'X'
        }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub rust: Vec<Vec<CellState>>,
    pub humidity: Vec<Vec<f32>>,
    pub oxygen: Vec<Vec<f32>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid { width,
            height,
            rust: vec![vec![CellState::Clean; width]; height],
            humidity: vec![vec![0.5; width]; height],
            oxygen: vec![vec![0.5; width]; height]
        }
    }   

    pub fn new_random(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut grid = Grid::new(width, height);

        for y in 0..height {
            for x in 0..width {
                grid.humidity[y][x] = rng.gen_range(0.0..1.0);
                grid.oxygen[y][x] = rng.gen_range(0.0..1.0);

                if rng.gen_range(0.0..1.0) < 0.1 {
                    grid.rust[y][x] = CellState::SurfaceRust;
                 }

            }
        } 
        grid
    }

    pub fn display(&self) {
        for row in &self.rust {
            let line: String = row.iter().map(|c| c.to_char()).collect();
            println!("{}", line);
        }
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let x = x as isize;
        let y = y as isize;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < self.width as isize && ny < self.height as isize {
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }

        neighbors
    }

}

