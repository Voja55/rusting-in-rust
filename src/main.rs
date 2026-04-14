mod grid;
mod rules;
mod output;
mod par_sim;
mod seq_sim;

use std::slice::GetDisjointMutError;

use grid::Grid;

fn main() {
    println!("Rustin in rust starting...");

    let mut grid = Grid::new_random(40, 20);

    //par_sim::run(&mut grid, 50, 100);
    seq_sim::run(&mut grid, 50, 100, "output/run_01");
}
